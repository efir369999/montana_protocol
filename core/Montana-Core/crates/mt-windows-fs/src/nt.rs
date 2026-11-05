//! A path opened through held directory handles. The walk starts at the volume root, opens every
//! parent relative to the one that holds it with FILE_OPEN_REPARSE_POINT, refuses a reparse point
//! encountered on the way, and opens the final object relative to the held parent. This is the
//! Windows safety addition described in the parity architecture; it does not guarantee that a
//! hostile namespace writer cannot rename a directory after the walk.

use std::ffi::{c_void, OsStr, OsString};
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::{Component, Path};

use windows_sys::Wdk::Foundation::OBJECT_ATTRIBUTES;
use windows_sys::Wdk::Storage::FileSystem::{
    NtCreateFile, RtlNtStatusToDosErrorNoTeb, FILE_DIRECTORY_FILE, FILE_OPEN,
    FILE_OPEN_REPARSE_POINT, FILE_SYNCHRONOUS_IO_NONALERT,
};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, UNICODE_STRING};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FileAttributeTagInfo, GetFileInformationByHandleEx, FILE_ATTRIBUTE_DIRECTORY,
    FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_TAG_INFO,
    FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_LIST_DIRECTORY,
    FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_TRAVERSE,
    OPEN_EXISTING, READ_CONTROL, SYNCHRONIZE,
};
use windows_sys::Win32::System::Kernel::OBJ_CASE_INSENSITIVE;
use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;

pub struct Handle(pub HANDLE);
impl Drop for Handle {
    fn drop(&mut self) {
        if self.0 != INVALID_HANDLE_VALUE && self.0 != 0 {
            unsafe { CloseHandle(self.0) };
        }
    }
}

fn status_error(status: i32) -> io::Error {
    let code = unsafe { RtlNtStatusToDosErrorNoTeb(status) };
    io::Error::from_raw_os_error(code as i32)
}

/// Opens a name relative to a held directory. No component of `name` is followed through a
/// reparse point; the caller checks the opened object.
#[allow(clippy::too_many_arguments)]
fn open_relative(
    parent: HANDLE,
    name: &OsStr,
    access: u32,
    disposition: u32,
    options: u32,
    attributes: u32,
    share: u32,
    security: *const c_void,
) -> io::Result<HANDLE> {
    let wide: Vec<u16> = name.encode_wide().collect();
    if wide.is_empty() || wide.len() > u16::MAX as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "a name of no width",
        ));
    }
    let us = UNICODE_STRING {
        Length: (wide.len() * 2) as u16,
        MaximumLength: (wide.len() * 2) as u16,
        Buffer: wide.as_ptr() as *mut u16,
    };
    let oa = OBJECT_ATTRIBUTES {
        Length: std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
        RootDirectory: parent,
        ObjectName: &us,
        Attributes: OBJ_CASE_INSENSITIVE as u32,
        SecurityDescriptor: security,
        SecurityQualityOfService: std::ptr::null(),
    };
    let mut iosb: IO_STATUS_BLOCK = unsafe { std::mem::zeroed() };
    let mut handle: HANDLE = 0;
    let status = unsafe {
        NtCreateFile(
            &mut handle,
            access,
            &oa,
            &mut iosb,
            std::ptr::null(),
            attributes,
            share,
            disposition,
            options | FILE_OPEN_REPARSE_POINT | FILE_SYNCHRONOUS_IO_NONALERT,
            std::ptr::null_mut(),
            0,
        )
    };
    if status < 0 {
        return Err(status_error(status));
    }
    Ok(handle)
}

/// The object attributes of an opened file or directory, read through its handle.
pub fn attributes(handle: HANDLE) -> io::Result<u32> {
    let mut tag: FILE_ATTRIBUTE_TAG_INFO = unsafe { std::mem::zeroed() };
    if unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileAttributeTagInfo,
            &mut tag as *mut _ as *mut c_void,
            std::mem::size_of::<FILE_ATTRIBUTE_TAG_INFO>() as u32,
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    Ok(tag.FileAttributes)
}

/// Refuses a reparse point or a directory at the final name.
pub fn require_regular(handle: HANDLE) -> io::Result<()> {
    let attributes = attributes(handle)?;
    if attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "the path is a reparse point",
        ));
    }
    if attributes & FILE_ATTRIBUTE_DIRECTORY != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the path is a directory",
        ));
    }
    Ok(())
}

/// A held chain from the volume root to the parent of the file. Every handle stays open for the
/// life of the value; the file is opened relative to `dir`.
pub struct Parent {
    _chain: Vec<Handle>,
    dir: HANDLE,
    name: OsString,
}

impl Parent {
    /// Walks to the parent of `at` through held handles and refuses a reparse point encountered on
    /// the way. A network path is refused because its root cannot be held the same way.
    pub fn open(at: &Path) -> io::Result<Self> {
        let absolute = std::path::absolute(at)?;
        let name = absolute
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the path has no name"))?
            .to_os_string();
        let parent = absolute
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the path has no parent"))?;
        let text = parent.to_string_lossy().to_string();
        if text.starts_with("\\\\") {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "a network or verbatim path is refused: the data directory must be a normal local path",
            ));
        }
        let bytes = text.as_bytes();
        if bytes.len() < 3 || bytes[1] != b':' || !(bytes[2] == b'\\' || bytes[2] == b'/') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "the data directory must be an absolute local path",
            ));
        }
        let root = &text[..3];
        let root_handle = unsafe {
            CreateFileW(
                root.encode_utf16()
                    .chain(Some(0))
                    .collect::<Vec<u16>>()
                    .as_ptr(),
                FILE_LIST_DIRECTORY
                    | FILE_TRAVERSE
                    | FILE_READ_ATTRIBUTES
                    | READ_CONTROL
                    | SYNCHRONIZE,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
                0,
            )
        };
        if root_handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let mut chain = vec![Handle(root_handle)];
        let mut current = root_handle;
        for component in Path::new(&text[3..]).components() {
            let dir = match component {
                Component::Normal(dir) => dir,
                Component::CurDir => continue,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "the data directory has an unheld component",
                    ))
                }
            };
            let handle = open_relative(
                current,
                dir,
                FILE_LIST_DIRECTORY
                    | FILE_TRAVERSE
                    | FILE_READ_ATTRIBUTES
                    | READ_CONTROL
                    | SYNCHRONIZE,
                FILE_OPEN,
                FILE_DIRECTORY_FILE,
                FILE_ATTRIBUTE_NORMAL,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                std::ptr::null(),
            )?;
            let handle = Handle(handle);
            let attributes = attributes(handle.0)?;
            if attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "a parent directory is a reparse point",
                ));
            }
            if attributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "a parent component is not a directory",
                ));
            }
            chain.push(handle);
            current = chain.last().map(|h| h.0).unwrap_or(current);
        }
        Ok(Self {
            _chain: chain,
            dir: current,
            name,
        })
    }

    pub fn name(&self) -> &OsStr {
        &self.name
    }

    /// Opens/creates `name` relative to the held parent. The caller checks the object through its
    /// handle and applies/verifies the owner-only DACL before writing a detail.
    #[allow(clippy::too_many_arguments)]
    pub fn open_child(
        &self,
        name: &OsStr,
        access: u32,
        disposition: u32,
        options: u32,
        attributes: u32,
        share: u32,
        security: *const c_void,
    ) -> io::Result<HANDLE> {
        open_relative(
            self.dir,
            name,
            access,
            disposition,
            options,
            attributes,
            share,
            security,
        )
    }
}
