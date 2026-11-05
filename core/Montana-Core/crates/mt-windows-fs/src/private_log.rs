// The Windows half of the owner-only file of details (#22, K1). Unix closes the file with a mode;
// Windows closes it with a protected DACL that grants one right to one trustee -- the user this
// process runs as -- and to nobody else. The rule is put in force on the handle that is written
// through, never on a name that could be swapped between the check and the write, and it is read
// back before the first detail is written. A file that cannot be made owner-only is refused.

use std::ffi::c_void;
use std::fs::File;
use std::io;
use std::os::windows::io::FromRawHandle;
use std::path::Path;

use crate::nt::{self, Parent};
use windows_sys::Win32::Foundation::{
    CloseHandle, LocalFree, ERROR_SUCCESS, HANDLE, HLOCAL, INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Security::Authorization::{
    GetSecurityInfo, SetEntriesInAclW, SetSecurityInfo, EXPLICIT_ACCESS_W, GRANT_ACCESS,
    SE_FILE_OBJECT, TRUSTEE_IS_SID, TRUSTEE_IS_USER,
};
use windows_sys::Win32::Security::{
    EqualSid, GetAce, GetLengthSid, GetSecurityDescriptorControl, GetSecurityDescriptorDacl,
    GetTokenInformation, InitializeSecurityDescriptor, SetSecurityDescriptorControl,
    SetSecurityDescriptorDacl, TokenUser, ACCESS_ALLOWED_ACE, ACL, DACL_SECURITY_INFORMATION,
    NO_INHERITANCE, OWNER_SECURITY_INFORMATION, PROTECTED_DACL_SECURITY_INFORMATION,
    SECURITY_DESCRIPTOR, SE_DACL_PROTECTED, TOKEN_QUERY, TOKEN_USER,
};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ALL_ACCESS, FILE_APPEND_DATA, FILE_ATTRIBUTE_NORMAL, FILE_READ_ATTRIBUTES,
    FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, READ_CONTROL, SYNCHRONIZE, WRITE_DAC,
};
use windows_sys::Win32::System::SystemServices::{
    ACCESS_ALLOWED_ACE_TYPE, SECURITY_DESCRIPTOR_REVISION,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

// The one right the owner holds in the file: the standard full right of a file. Reading it back
// is what tells a narrow DACL from a DACL that merely looks narrow.

struct Handle(HANDLE);
impl Drop for Handle {
    fn drop(&mut self) {
        if self.0 != INVALID_HANDLE_VALUE && self.0 != 0 {
            unsafe { CloseHandle(self.0) };
        }
    }
}

struct LocalMemory(*mut c_void);
impl Drop for LocalMemory {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { LocalFree(self.0 as HLOCAL) };
        }
    }
}

fn os_error(code: u32) -> io::Error {
    io::Error::from_raw_os_error(code as i32)
}

fn last_error() -> io::Error {
    io::Error::last_os_error()
}

/// The SID of the user this process runs as. The token is closed before the bytes are returned.
fn current_user_sid() -> io::Result<Vec<u8>> {
    let mut token: HANDLE = 0;
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
        return Err(last_error());
    }
    let token = Handle(token);
    let mut needed = 0u32;
    unsafe {
        GetTokenInformation(token.0, TokenUser, std::ptr::null_mut(), 0, &mut needed);
    }
    if needed == 0 {
        return Err(last_error());
    }
    let mut buffer = vec![0u8; needed as usize];
    if unsafe {
        GetTokenInformation(
            token.0,
            TokenUser,
            buffer.as_mut_ptr() as *mut c_void,
            needed,
            &mut needed,
        )
    } == 0
    {
        return Err(last_error());
    }
    let user = unsafe { &*(buffer.as_ptr() as *const TOKEN_USER) };
    let sid = user.User.Sid;
    if sid.is_null() || unsafe { GetLengthSid(sid) } == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the process token carries no user SID",
        ));
    }
    let length = unsafe { GetLengthSid(sid) } as usize;
    let mut copied = vec![0u8; length];
    if unsafe {
        windows_sys::Win32::Security::CopySid(length as u32, copied.as_mut_ptr() as _, sid)
    } == 0
    {
        return Err(last_error());
    }
    Ok(copied)
}

/// A protected DACL that grants FILE_ALL_ACCESS to `sid` and to no other trustee.
fn owner_only_acl(sid: &[u8]) -> io::Result<*mut ACL> {
    let mut entry: EXPLICIT_ACCESS_W = unsafe { std::mem::zeroed() };
    entry.grfAccessPermissions = FILE_ALL_ACCESS;
    entry.grfAccessMode = GRANT_ACCESS;
    entry.grfInheritance = NO_INHERITANCE;
    entry.Trustee.TrusteeForm = TRUSTEE_IS_SID;
    entry.Trustee.TrusteeType = TRUSTEE_IS_USER;
    entry.Trustee.ptstrName = sid.as_ptr() as *mut u16;
    let mut acl: *mut ACL = std::ptr::null_mut();
    let code = unsafe { SetEntriesInAclW(1, &entry, std::ptr::null_mut(), &mut acl) };
    if code != ERROR_SUCCESS {
        return Err(os_error(code));
    }
    Ok(acl)
}

/// The security descriptor used at creation, so a new file is never born wider than its owner.
fn owner_only_descriptor(acl: *mut ACL) -> io::Result<SECURITY_DESCRIPTOR> {
    let mut descriptor: SECURITY_DESCRIPTOR = unsafe { std::mem::zeroed() };
    if unsafe {
        InitializeSecurityDescriptor(
            &mut descriptor as *mut _ as *mut c_void,
            SECURITY_DESCRIPTOR_REVISION,
        )
    } == 0
    {
        return Err(last_error());
    }
    if unsafe { SetSecurityDescriptorDacl(&mut descriptor as *mut _ as *mut c_void, 1, acl, 0) }
        == 0
    {
        return Err(last_error());
    }
    if unsafe {
        SetSecurityDescriptorControl(
            &mut descriptor as *mut _ as *mut c_void,
            SE_DACL_PROTECTED,
            SE_DACL_PROTECTED,
        )
    } == 0
    {
        return Err(last_error());
    }
    Ok(descriptor)
}

/// Reads the DACL and owner back from an open handle and refuses anything but one protected
/// ACCESS_ALLOWED entry for the process's own SID, held by the process's own SID.
fn verify(handle: HANDLE, sid: &[u8]) -> io::Result<()> {
    let mut owner: *mut c_void = std::ptr::null_mut();
    let mut dacl: *mut ACL = std::ptr::null_mut();
    let mut descriptor: *mut SECURITY_DESCRIPTOR = std::ptr::null_mut();
    let code = unsafe {
        GetSecurityInfo(
            handle,
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner,
            std::ptr::null_mut(),
            &mut dacl,
            std::ptr::null_mut(),
            &mut descriptor as *mut _ as *mut *mut c_void,
        )
    };
    if code != ERROR_SUCCESS {
        return Err(os_error(code));
    }
    let descriptor = LocalMemory(descriptor as *mut c_void);
    if owner.is_null() || unsafe { EqualSid(owner, sid.as_ptr() as *mut c_void) } == 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "the file is not owned by this process's user",
        ));
    }
    let mut control = 0u16;
    let mut revision = 0u32;
    if unsafe { GetSecurityDescriptorControl(descriptor.0, &mut control, &mut revision) } == 0 {
        return Err(last_error());
    }
    if control & SE_DACL_PROTECTED == 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "the file's DACL inherits from its directory",
        ));
    }
    let mut read_dacl: *mut ACL = std::ptr::null_mut();
    let mut present = 0i32;
    let mut defaulted = 0i32;
    if unsafe {
        GetSecurityDescriptorDacl(descriptor.0, &mut present, &mut read_dacl, &mut defaulted)
    } == 0
    {
        return Err(last_error());
    }
    if present == 0 || read_dacl.is_null() || unsafe { (*read_dacl).AceCount } != 1 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "the file's DACL does not hold exactly one entry",
        ));
    }
    let mut ace: *mut c_void = std::ptr::null_mut();
    if unsafe { GetAce(read_dacl, 0, &mut ace) } == 0 {
        return Err(last_error());
    }
    let ace = unsafe { &*(ace as *const ACCESS_ALLOWED_ACE) };
    if ace.Header.AceType != ACCESS_ALLOWED_ACE_TYPE as u8 || ace.Mask != FILE_ALL_ACCESS {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "the file's entry is not a full right for its owner",
        ));
    }
    let ace_sid = &ace.SidStart as *const u32 as *mut c_void;
    if unsafe { EqualSid(ace_sid, sid.as_ptr() as *mut c_void) } == 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "the file's entry grants another trustee",
        ));
    }
    Ok(())
}

/// Opens the owner-only file of details for appending. The object is created or opened relative to
/// the held parent; the owner-only DACL is put in force on that object and read back before the
/// caller writes a single detail. No second object, temporary, rename or hardlink decision exists:
/// Montana keeps one file, and a Windows DACL belongs to the object, so every hardlink is covered.
pub fn open_owner_only(at: &Path) -> io::Result<File> {
    let parent = Parent::open(at)?;
    let sid = current_user_sid()?;
    let acl = owner_only_acl(&sid)?;
    let acl = LocalMemory(acl as *mut c_void);
    let descriptor = owner_only_descriptor(acl.0 as *mut ACL)?;
    let handle = parent.open_child(
        parent.name(),
        FILE_APPEND_DATA | FILE_READ_ATTRIBUTES | READ_CONTROL | WRITE_DAC | SYNCHRONIZE,
        windows_sys::Wdk::Storage::FileSystem::FILE_OPEN_IF,
        windows_sys::Wdk::Storage::FileSystem::FILE_NON_DIRECTORY_FILE,
        FILE_ATTRIBUTE_NORMAL,
        FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
        &descriptor as *const _ as *const c_void,
    )?;
    let handle = Handle(handle);
    nt::require_regular(handle.0)?;
    let code = unsafe {
        SetSecurityInfo(
            handle.0,
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            acl.0 as *mut ACL,
            std::ptr::null_mut(),
        )
    };
    if code != ERROR_SUCCESS {
        return Err(os_error(code));
    }
    verify(handle.0, &sid)?;
    let raw = handle.0;
    std::mem::forget(handle);
    Ok(unsafe { File::from_raw_handle(raw as *mut c_void) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::fs::symlink_file;
    use windows_sys::Win32::Storage::FileSystem::CreateFileW;

    fn fresh_dir(name: &str) -> std::path::PathBuf {
        let at = std::env::temp_dir().join(format!("mt-private-win-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(at.as_path());
        std::fs::create_dir_all(at.as_path()).expect("a temporary directory");
        at
    }

    fn set_acl(at: &Path, entries: &mut [EXPLICIT_ACCESS_W], protected: bool) -> io::Result<()> {
        let mut acl: *mut ACL = std::ptr::null_mut();
        let code = unsafe {
            SetEntriesInAclW(
                entries.len() as u32,
                entries.as_mut_ptr(),
                std::ptr::null_mut(),
                &mut acl,
            )
        };
        if code != ERROR_SUCCESS {
            return Err(os_error(code));
        }
        let acl = LocalMemory(acl as *mut c_void);
        let wide: Vec<u16> = at.as_os_str().encode_wide().chain(Some(0)).collect();
        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                READ_CONTROL | WRITE_DAC,
                0,
                std::ptr::null_mut(),
                windows_sys::Win32::Storage::FileSystem::OPEN_ALWAYS,
                FILE_ATTRIBUTE_NORMAL,
                0,
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(last_error());
        }
        let handle = Handle(handle);
        let mut info = DACL_SECURITY_INFORMATION;
        if protected {
            info |= PROTECTED_DACL_SECURITY_INFORMATION;
        }
        let code = unsafe {
            SetSecurityInfo(
                handle.0,
                SE_FILE_OBJECT,
                info,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                acl.0 as *mut ACL,
                std::ptr::null_mut(),
            )
        };
        if code != ERROR_SUCCESS {
            return Err(os_error(code));
        }
        Ok(())
    }

    fn entry(sid: &[u8], mask: u32, mode: i32, inheritance: u32) -> EXPLICIT_ACCESS_W {
        let mut entry: EXPLICIT_ACCESS_W = unsafe { std::mem::zeroed() };
        entry.grfAccessPermissions = mask;
        entry.grfAccessMode = mode;
        entry.grfInheritance = inheritance;
        entry.Trustee.TrusteeForm = TRUSTEE_IS_SID;
        entry.Trustee.TrusteeType = TRUSTEE_IS_USER;
        entry.Trustee.ptstrName = sid.as_ptr() as *mut u16;
        entry
    }

    fn everyone_sid() -> Vec<u8> {
        let text: Vec<u16> = std::ffi::OsStr::new("S-1-1-0")
            .encode_wide()
            .chain(Some(0))
            .collect();
        let mut sid: *mut c_void = std::ptr::null_mut();
        let ok = unsafe {
            windows_sys::Win32::Security::Authorization::ConvertStringSidToSidW(
                text.as_ptr(),
                &mut sid,
            )
        };
        assert_ne!(ok, 0, "the Everyone SID");
        let length = unsafe { GetLengthSid(sid) } as usize;
        let mut copied = vec![0u8; length];
        unsafe {
            windows_sys::Win32::Security::CopySid(length as u32, copied.as_mut_ptr() as _, sid)
        };
        unsafe { LocalFree(sid as HLOCAL) };
        copied
    }

    #[test]
    fn a_new_file_is_born_owner_only() {
        let dir = fresh_dir("new");
        let at = dir.join("private.log");
        let mut file = open_owner_only(at.as_path()).expect("the file opens");
        use std::io::Write;
        writeln!(file, "a detail").expect("a detail is written");
        drop(file);
        let sid = current_user_sid().expect("a user SID");
        let raw = std::fs::File::open(at.as_path()).expect("the file opens for reading");
        use std::os::windows::io::AsRawHandle;
        verify(raw.as_raw_handle() as HANDLE, &sid).expect("owner-only DACL");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_wide_existing_file_is_narrowed_before_it_is_written() {
        let dir = fresh_dir("wide");
        let at = dir.join("private.log");
        std::fs::write(at.as_path(), b"old detail\n").expect("a wide file");
        let sid = current_user_sid().expect("a user SID");
        let everyone = everyone_sid();
        let mut entries = [entry(
            &everyone,
            FILE_ALL_ACCESS,
            GRANT_ACCESS,
            NO_INHERITANCE,
        )];
        set_acl(at.as_path(), &mut entries, false).expect("a wide DACL");
        let mut file = open_owner_only(at.as_path()).expect("the file is narrowed");
        use std::io::Write;
        writeln!(file, "new detail").expect("a line");
        drop(file);
        let raw = std::fs::File::open(at.as_path()).expect("the file reads");
        use std::os::windows::io::AsRawHandle;
        verify(raw.as_raw_handle() as HANDLE, &sid).expect("owner-only DACL");
        let text = std::fs::read_to_string(at.as_path()).expect("the file reads");
        assert!(
            text.contains("old detail") && text.contains("new detail"),
            "{text:?}"
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_dacl_with_another_trustee_is_refused_before_a_write() {
        let dir = fresh_dir("foreign");
        let at = dir.join("private.log");
        std::fs::write(at.as_path(), b"held").expect("a file");
        let sid = current_user_sid().expect("a user SID");
        let everyone = everyone_sid();
        let mut entries = [
            entry(&sid, FILE_ALL_ACCESS, GRANT_ACCESS, NO_INHERITANCE),
            entry(&everyone, FILE_ALL_ACCESS, GRANT_ACCESS, NO_INHERITANCE),
        ];
        set_acl(at.as_path(), &mut entries, true).expect("a second trustee");
        let raw = std::fs::File::open(at.as_path()).expect("the file reads");
        use std::os::windows::io::AsRawHandle;
        assert!(
            verify(raw.as_raw_handle() as HANDLE, &sid).is_err(),
            "a DACL with another trustee is not owner-only"
        );
        assert_eq!(
            std::fs::read(at.as_path()).expect("the file reads"),
            b"held"
        );
        let _ = std::fs::remove_dir_all(dir);
    }
    #[test]
    fn a_reader_opened_before_narrowing_still_reads() {
        let dir = fresh_dir("held-reader");
        let at = dir.join("private.log");
        let mut live = open_owner_only(at.as_path()).expect("a live log");
        use std::io::Write;
        writeln!(live, "kept").expect("a line");
        drop(live);
        let mut held = std::fs::File::open(at.as_path()).expect("a reader before the narrowing");
        let mut file = open_owner_only(at.as_path()).expect("the log narrows");
        writeln!(file, "new").expect("a line");
        drop(file);
        let mut seen = String::new();
        use std::io::Read;
        held.read_to_string(&mut seen)
            .expect("the old handle reads");
        assert!(seen.contains("kept"), "{seen:?}");
        assert!(
            seen.contains("new"),
            "a pre-opened handle sees the append: {seen:?}"
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_hardlink_shares_the_owner_only_object() {
        let dir = fresh_dir("hardlink");
        let at = dir.join("private.log");
        std::fs::write(at.as_path(), b"old detail\n").expect("a wide file");
        let everyone = everyone_sid();
        let mut entries = [entry(
            &everyone,
            FILE_ALL_ACCESS,
            GRANT_ACCESS,
            NO_INHERITANCE,
        )];
        set_acl(at.as_path(), &mut entries, false).expect("a wide DACL");
        let old = dir.join("old-copy.log");
        std::fs::hard_link(at.as_path(), old.as_path()).expect("a hard link");
        let mut file = open_owner_only(at.as_path()).expect("the object is narrowed");
        use std::io::Write;
        writeln!(file, "new detail").expect("a line");
        drop(file);
        let sid = current_user_sid().expect("a user SID");
        for path in [at.as_path(), old.as_path()] {
            let raw = std::fs::File::open(path).expect("the name opens");
            use std::os::windows::io::AsRawHandle;
            verify(raw.as_raw_handle() as HANDLE, &sid).expect("owner-only DACL");
            let text = std::fs::read_to_string(path).expect("the name reads");
            assert!(
                text.contains("old detail") && text.contains("new detail"),
                "{text:?}"
            );
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_foreign_temporary_is_not_deleted() {
        let dir = fresh_dir("foreign-temp");
        let at = dir.join("private.log");
        let mut live = open_owner_only(at.as_path()).expect("a live log");
        use std::io::Write;
        writeln!(live, "kept").expect("a line");
        drop(live);
        let foreign = dir.join(".private.log.new-owner-document");
        std::fs::write(&foreign, b"document").expect("a foreign file");
        let link = dir.join(".private.log.new-owner-link");
        std::fs::hard_link(&foreign, &link).expect("a foreign hard link");
        let file = open_owner_only(at.as_path()).expect("the live log stands");
        drop(file);
        assert_eq!(
            std::fs::read(&foreign).expect("the document reads"),
            b"document"
        );
        assert_eq!(std::fs::read(&link).expect("the link reads"), b"document");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_parent_junction_is_refused() {
        let dir = fresh_dir("parent-link");
        let target = dir.join("target");
        std::fs::create_dir(target.as_path()).expect("a target directory");
        let linked = dir.join("linked");
        let made = std::process::Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(&linked)
            .arg(&target)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !made {
            eprintln!("skipped: the junction privilege is not present");
            let _ = std::fs::remove_dir_all(dir);
            return;
        }
        assert!(
            open_owner_only(linked.join("private.log").as_path()).is_err(),
            "a parent junction is refused"
        );
        assert!(
            !target.join("private.log").exists(),
            "the target is untouched"
        );
        let _ = std::process::Command::new("cmd")
            .args(["/c", "rmdir"])
            .arg(&linked)
            .status();
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_live_log_that_is_a_junction_is_refused() {
        let dir = fresh_dir("live-link");
        let target = dir.join("target");
        std::fs::create_dir(target.as_path()).expect("a target directory");
        let at = dir.join("private.log");
        let made = std::process::Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(&at)
            .arg(&target)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !made {
            eprintln!("skipped: the junction privilege is not present");
            let _ = std::fs::remove_dir_all(dir);
            return;
        }
        assert!(
            open_owner_only(at.as_path()).is_err(),
            "a junction is refused"
        );
        assert!(target.exists(), "the junction's target is untouched");
        let _ = std::process::Command::new("cmd")
            .args(["/c", "rmdir"])
            .arg(&at)
            .status();
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_reparse_point_is_refused_and_its_target_is_not_written() {
        let dir = fresh_dir("link");
        let target = dir.join("target.log");
        let link = dir.join("private.log");
        std::fs::write(target.as_path(), b"untouched").expect("a target");
        if symlink_file(target.as_path(), link.as_path()).is_err() {
            eprintln!("SKIP: the privilege to make a file symlink is not present");
            let _ = std::fs::remove_dir_all(dir);
            return;
        }
        assert!(
            open_owner_only(link.as_path()).is_err(),
            "the link is refused"
        );
        assert_eq!(
            std::fs::read(target.as_path()).expect("the target reads"),
            b"untouched"
        );
        let _ = std::fs::remove_dir_all(dir);
    }
}
