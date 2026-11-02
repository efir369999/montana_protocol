//! THE BOOK (iOS MTPipeBook + ChatStore; Android Pipes.kt Book, Msg, Chat): the pipe of each correspondence, held by its one
//! owner (pipekey::PipeKey) and filed under its local name (SHA-256(secret)[0..16] in hex), and the feed of it. Everything is
//! sealed in the device vault; the pipes never leave this machine. From the erase day (03.11.2026 UTC) every pipe is stored in
//! the floor's form, on every write and on the first read of each day. The rows keep Android's store names (m, t, o, at, s …)
//! so a copy of a history reads alike.

use crate::pipeheal::{Part as HealPart, Side as HealSide, Walk as HealWalk};
use crate::pipekey::PipeKey;
use crate::vault::Vault;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// The pipes (iOS MTPipeBook storeKey): reference -> base64 of each pipe in the form of the day.
const SECRETS: &str = "pipeSecrets";
/// THE CLOCK GUARD'S RECORD (nodeclock, MONTANA_CLOCK_GUARD), sealed beside the pipe book under the same seal and never in the
/// archive: M, the highest day a node's Date named; H and Hc, the device's day and the confirmed day when the first floor first
/// waited; done, the first floor applied (rule 4, kept even when the book's own write is refused). A restart keeps it; a restore
/// by the words starts without it. A record that does not read is never written over.
const GUARD_RECORD: &str = "floorGuard";
/// Each pipe's side of a heal, sealed beside the pipe book as iOS 2323–2325 keeps it.
const HEALS: &str = "pipeHeal";
/// A refused write of the book is tried again at the first read this long after the refusal (#38, 22:04 UTC, point 6).
const RETRY_WAIT: std::time::Duration = std::time::Duration::from_secs(60);
const CHATS: &str = "chats";

/// SAVED MESSAGES (iOS savedMessagesKey): the room of the person writing to themselves -- its key English and never translated,
/// no pipe, no wire: nothing there is sent, waits or fails (iOS isLocalRoom). It reaches the identity's other devices as the
/// archive's blocks alone, under the folder of its own key.
pub const SAVED: &str = "Saved Messages";

pub fn is_saved(r: &str) -> bool {
    r == SAVED
}

/// One letter of the feed. `state` is the rung (iOS DeliveryStatus.rung): 0 sending, 1 sent, 2 delivered, 3 read, -1 not sent.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Msg {
    #[serde(rename = "m")]
    pub mid: String,
    #[serde(rename = "t")]
    pub text: String,
    #[serde(rename = "o")]
    pub mine: bool,
    pub at: i64,
    #[serde(rename = "s", default)]
    pub state: i32,
    #[serde(rename = "sa", default)]
    pub status_at: i64,
    /// the quoted letter's words (at most 200) and its mid: a reply (fields 4 and 5 of the body)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub qt: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub qm: String,
    /// their words changed after the letter came (iOS «edited»)
    #[serde(rename = "ed", default, skip_serializing_if = "std::ops::Not::not")]
    pub edited: bool,
    /// my answer to the letter and theirs (one reaction per person, iOS myReact / peerReact)
    #[serde(rename = "mr", default, skip_serializing_if = "String::is_empty")]
    pub my_react: String,
    #[serde(rename = "pr", default, skip_serializing_if = "String::is_empty")]
    pub peer_react: String,
    /// in a group: who said it ("gm:<group>/<seat>"); empty for mine and for a pair's letters
    #[serde(rename = "fr", default, skip_serializing_if = "String::is_empty")]
    pub from: String,
    /// the card of a link the letter carries (iOS linkPreview: {u, s, t, d, p}), as JSON; empty -- none
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub lp: String,
    /// the author field their letter rode with (iOS Message.authorTag): given back with it when they restore, it proves the
    /// letter theirs by their own key, never by this machine's word
    #[serde(rename = "au", default, skip_serializing_if = "String::is_empty")]
    pub author: String,
}

/// A word of the wire no node took yet: it goes again with the next walk, for a day (iOS queue).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Pending {
    #[serde(rename = "m")]
    pub mid: String,
    #[serde(rename = "t")]
    pub text: String,
    pub at: i64,
}

impl Msg {
    /// THE ONE DOOR TO THE RUNG (iOS ChatStore.advance): only forward; «read» only from «delivered»; «not sent» only from
    /// «sending» or «sent».
    pub fn advance(&mut self, next: i32) -> bool {
        let ok = match next {
            -1 => self.state == 0 || self.state == 1,
            3 => self.state == 2,
            n => n > self.state,
        };
        if ok {
            self.state = next;
            self.status_at = crate::wire::now_ms();
        }
        ok
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Chat {
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(rename = "n", default)]
    pub name: String,
    /// my own name for them (iOS MTNameBook.manual): it stands over the one they gave themselves; empty -- theirs
    #[serde(rename = "al", default, skip_serializing_if = "String::is_empty")]
    pub alias: String,
    #[serde(rename = "p", default)]
    pub perm: bool,
    #[serde(rename = "u", default)]
    pub unread: u32,
    #[serde(default)]
    pub msgs: Vec<Msg>,
    /// my name as their receipt confirmed it (iOS Announced.recordDelivered): said again only when it changes
    #[serde(rename = "an", default)]
    pub announced: String,
    /// the name on its way: name, the letter's mid and the moment it left; not said twice within the hour (Android annFly)
    #[serde(rename = "af", default)]
    pub name_flight: String,
    /// the birth of the newest letter of mine they said they read (Android PeerRead): a later receipt catches up to it
    #[serde(rename = "pr", default)]
    pub peer_read: i64,
    /// the names of letters taken away: a copy that comes again is answered, never landed (iOS graves)
    #[serde(rename = "gr", default, skip_serializing_if = "Vec::is_empty")]
    pub graves: Vec<String>,
    /// their edits of letters not landed yet (mid, words), at most 32 (iOS held edits)
    #[serde(rename = "he", default, skip_serializing_if = "Vec::is_empty")]
    pub held_edits: Vec<(String, String)>,
    /// the conversation was erased at this birth: nothing born before it lands again (iOS peerFuneral)
    #[serde(rename = "fu", default)]
    pub funeral: i64,
    /// my own erasure for both, waiting for its receipt; then the pipe is forgotten (iOS funeral)
    #[serde(rename = "fm", default, skip_serializing_if = "String::is_empty")]
    pub funeral_mid: String,
    /// out of the list until a letter lands again
    #[serde(rename = "dy", default)]
    pub hidden: bool,
    /// words of the wire waiting for a node
    #[serde(rename = "wq", default, skip_serializing_if = "Vec::is_empty")]
    pub pending: Vec<Pending>,
    /// their face: the birth of the word that set it (0 = none held); a word older than it changes nothing (iOS stateIsCurrent)
    #[serde(rename = "fa", default)]
    pub face_at: i64,
    /// my face as their receipt confirmed it (its tag), and the face on its way: tag, mid, moment (Android annFace2 / annFlyF)
    #[serde(rename = "fs", default, skip_serializing_if = "String::is_empty")]
    pub face_said: String,
    #[serde(rename = "ff", default, skip_serializing_if = "String::is_empty")]
    pub face_flight: String,
    /// my words about myself as their receipt confirmed them, and the ones on their way (tag, mid, moment)
    #[serde(rename = "bs", default, skip_serializing_if = "String::is_empty")]
    pub about_said: String,
    #[serde(rename = "bf", default, skip_serializing_if = "String::is_empty")]
    pub about_flight: String,
    /// their words about themselves (iOS PeerAbout): the bio, the link, the moment they said them
    #[serde(rename = "pb", default, skip_serializing_if = "String::is_empty")]
    pub peer_bio: String,
    #[serde(rename = "pl", default, skip_serializing_if = "String::is_empty")]
    pub peer_link: String,
    #[serde(rename = "pa", default)]
    pub peer_about_at: f64,
    /// the person's own order of the list (iOS pinnedChats, archive, MTQuietChats, the manual unread mark): never said to anyone
    #[serde(rename = "pn", default)]
    pub pinned_at: i64,
    /// the moment the person pinned it last (iOS noteOrder at pinning: «the new one to the very top -- the person's deed»): the
    /// list's order keeps it after the pin is taken off, as iOS never talks a chat's moment down
    #[serde(rename = "oa", default, skip_serializing_if = "unset")]
    pub ordered_at: i64,
    #[serde(rename = "ar", default)]
    pub archived: bool,
    #[serde(rename = "mu", default)]
    pub muted: bool,
    #[serde(rename = "mk", default)]
    pub marked: bool,
    /// blocked (iOS blockedChats): nothing of theirs lands, nothing of mine goes
    #[serde(rename = "bl", default)]
    pub blocked: bool,
    /// the letters pinned in the chat, newest last (iOS pinned per chat; PN: for both)
    #[serde(rename = "pins", default, skip_serializing_if = "Vec::is_empty")]
    pub pins: Vec<String>,
    /// THE ANSWER THE LIST'S ROW WEARS (iOS listState «r», «rm», «ra», «rat»): the newest answer to a letter of this chat -- mine or
    /// theirs -- until a letter born after it takes the row back; its withdrawal takes it away. This machine's own, never said.
    #[serde(rename = "rr", default, skip_serializing_if = "Option::is_none")]
    pub row_react: Option<RowReact>,
}

/// One answer as the row wears it: the letter answered, the emoji, whose, the answer's own birth (ms).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct RowReact {
    #[serde(rename = "m")]
    pub mid: String,
    #[serde(rename = "e")]
    pub emoji: String,
    #[serde(rename = "o", default)]
    pub mine: bool,
    pub at: i64,
}

impl Chat {
    /// A letter taken away: its row leaves and its name is kept, so a copy never lands (4000 names, the newest kept).
    pub fn bury(&mut self, mid: &str) {
        self.msgs.retain(|m| m.mid != mid);
        if !self.graves.iter().any(|g| g == mid) {
            self.graves.push(mid.to_string());
            if self.graves.len() > 4000 {
                let over = self.graves.len() - 4000;
                self.graves.drain(..over);
            }
        }
    }
}

/// The book's day, handed in: the computer's UTC day in the program, a hand of the test's own in a test.
pub type DayFn = Arc<dyn Fn() -> i64 + Send + Sync>;
/// The day a node's Date names now, handed in (nodeclock::day_heard in the program): None while no door answered.
pub type NodesFn = Arc<dyn Fn() -> Option<i64> + Send + Sync>;

/// The guard's record as read from the vault, and whether it still waits to be written.
#[derive(Clone, Copy, Default)]
struct GuardKept {
    m: Option<i64>,
    hold: Option<(i64, i64)>,
    done: bool,
    /// the record is there and does not read: it is never written over, and the guard does not withhold
    unreadable: bool,
    /// a change not on the disk yet: written again at the next decision
    dirty: bool,
}

/// The guard's record as a check reads it: M, H and Hc, done.
#[cfg(test)]
pub type GuardRecord = (Option<i64>, Option<(i64, i64)>, bool);

/// What a check counts of the book: its writes, the refused ones, the guard's lines and its askings.
#[cfg(test)]
#[derive(Clone, Default)]
pub struct Seen {
    pub writes: usize,
    pub refused: usize,
    pub lines: Vec<String>,
    pub asks: usize,
}

/// The pipes as this process holds them and the day they were read on (iOS MTPipeBook.cache).
type Held = Option<(i64, BTreeMap<String, PipeKey>)>;

pub struct Book {
    vault: Vault,
    chats: Mutex<Option<Vec<Chat>>>,
    lock: Mutex<()>,
    /// THE NEWEST CHANGE IS THE ONE ON DISK: every change numbered under the feeds' lock, and a change reaches the disk only if
    /// no later one is there already -- two changes written at once once laid the older last, and a letter left the book at the
    /// next start (found 23.10.2026: a film sent and «Sent» was gone from both feeds after the program was stopped)
    changes: std::sync::atomic::AtomicU64,
    on_disk: Mutex<u64>,
    /// the secrets laid aside are looked for once, at the first read of the secrets (`recover_secrets`)
    secrets_back: std::sync::Once,
    day: DayFn,
    nodes: NodesFn,
    /// the clock guard's M, H and Hc as read from the vault (None: not read yet)
    guard: Mutex<Option<GuardKept>>,
    /// the guard's last line in the trace, and its last floor_held: floor_held is said again only when d, c or left change
    guard_said: Mutex<String>,
    held_said: Mutex<String>,
    /// whether a key of the book holds its floor: 0 not known yet, 1 none, 2 one does (the guard is not asked again)
    floored_seen: std::sync::atomic::AtomicU8,
    /// THE PIPES IN MEMORY (iOS MTPipeBook.cache, 2311): read once and floored by the day. With the walk of #38 case 13
    /// (MONTANA_CLOCK_GUARD) the book walks to the floor's day on its first read of each new day, a pipe born in this run with it
    keys: Mutex<Held>,
    /// the receive sides of pipe heals, read once and never replaced by an unreadable record
    heals: Mutex<Option<BTreeMap<String, HealSide>>>,
    heal_lock: Mutex<()>,
    /// a walked book the disk refused, and when: the first read RETRY_WAIT after it writes it again
    refused_at: Mutex<Option<std::time::Instant>>,
    retry_wait: Mutex<std::time::Duration>,
    /// a check's own refusal of the disk (the failed write of case 13)
    #[cfg(test)]
    refuse_writes: std::sync::atomic::AtomicBool,
    /// the book's writes and the guard's lines and askings, for the checks
    #[cfg(test)]
    seen: Mutex<Seen>,
}

/// a moment never set is not written
fn unset(n: &i64) -> bool {
    *n == 0
}

/// A letter's name: t<birth ms>-<UUID> (iOS ChatStore.mintMid).
pub fn mint_mid() -> String {
    static LAST: Mutex<i64> = Mutex::new(0);
    let mut last = LAST.lock().unwrap();
    let mut ms = crate::wire::now_ms();
    if ms <= *last {
        ms = *last + 1;
    }
    *last = ms;
    let r = crate::crypto::random(16).unwrap_or_else(|| vec![0; 16]);
    let h = crate::wire::hex(&r).to_uppercase();
    format!("t{ms}-{}-{}-{}-{}-{}", &h[0..8], &h[8..12], &h[12..16], &h[16..20], &h[20..32])
}

pub fn birth_ms(mid: &str) -> Option<i64> {
    let core = mid.strip_prefix("mid:").unwrap_or(mid);
    let rest = core.strip_prefix('t')?;
    rest.split('-').next()?.parse().ok()
}

/// THE BOOK'S VALUES READ, OR KEPT ASIDE (the owner's word 24.10.2026: no loss of data): a value that is there but does not open
/// (the machine's seal refused) or does not parse is moved aside, never taken for an empty book the next change would write over
/// every feed and every secret; once it opens again it is merged back (`recover`).
fn read_kept<T: serde::de::DeserializeOwned>(vault: &Vault, name: &str) -> Option<T> {
    match vault.get(name).map(|b| serde_json::from_slice::<T>(&b)) {
        Some(Ok(v)) => Some(v),
        Some(Err(_)) => {
            vault.lay_aside(name);
            None
        }
        None => {
            if vault.has(name) {
                vault.lay_aside(name);
            }
            None
        }
    }
}

/// Two books of one person made one: every chat of either, a chat of both keeps the newer book's state and every letter of the
/// two by its name, the graves of both.
fn merged(mut now: Vec<Chat>, old: Vec<Chat>) -> Vec<Chat> {
    for o in old {
        match now.iter_mut().find(|c| c.reference == o.reference) {
            Some(c) => {
                for m in o.msgs {
                    if !c.msgs.iter().any(|x| x.mid == m.mid) && !c.graves.contains(&m.mid) {
                        c.msgs.push(m);
                    }
                }
                for g in o.graves {
                    if !c.graves.contains(&g) {
                        c.graves.push(g);
                    }
                }
                c.msgs.sort_by_key(|m| m.at);
            }
            None => now.push(o),
        }
    }
    now
}

/// Every pipe of a stored book in the form of the day (iOS stored(held(m))); a value that does not read is kept as it lies (no
/// loss).
fn floored_map(raw: &Map<String, Value>, day: i64) -> Map<String, Value> {
    let mut out = raw.clone();
    out.extend(PipeKey::stored(&PipeKey::held(raw, day), day));
    out
}

impl Book {
    pub fn new(vault: Vault) -> Book {
        Book::new_with(vault, Arc::new(crate::wire::day), Arc::new(crate::nodeclock::day_heard))
    }
    /// The book counting its days by the hand given, the nodes' Date naming the same day (a test's own date).
    #[cfg(test)]
    pub fn new_on(vault: Vault, day: DayFn) -> Book {
        let d = day.clone();
        Book::new_with(vault, day, Arc::new(move || Some(d())))
    }
    /// The book counting its days and hearing the nodes' Date by the hands given.
    pub fn new_with(vault: Vault, day: DayFn, nodes: NodesFn) -> Book {
        Book { vault, chats: Mutex::new(None), lock: Mutex::new(()), changes: std::sync::atomic::AtomicU64::new(0), on_disk: Mutex::new(0),
               secrets_back: std::sync::Once::new(), day, nodes, guard: Mutex::new(None), guard_said: Mutex::new(String::new()),
               floored_seen: std::sync::atomic::AtomicU8::new(0), keys: Mutex::new(None), held_said: Mutex::new(String::new()),
               heals: Mutex::new(None), heal_lock: Mutex::new(()), refused_at: Mutex::new(None), retry_wait: Mutex::new(RETRY_WAIT),
               #[cfg(test)]
               refuse_writes: std::sync::atomic::AtomicBool::new(false),
               #[cfg(test)]
               seen: Mutex::new(Seen::default()) }
    }

    /// THE DAY THE FLOOR STANDS ON: this machine's day (iOS 2311). With MONTANA_CLOCK_GUARD the first floor follows the guard's
    /// rule (nodeclock; decision 188, the ruling on #38) -- the device's day, or none while it waits. Letters and labels keep
    /// this machine's own day either way. A book with no pipe does not ask the rule (case 14).
    pub fn floor_day(&self) -> i64 {
        self.floor_day_at(false)
    }
    /// The floor day for a pipe arriving now -- a birth, a restore, a pipe handed by a seat: the rule is asked as for a book that
    /// holds it, so the first pipe of an empty book is the one that asks (#38 case 14).
    pub fn arrival_day(&self) -> i64 {
        self.floor_day_at(true)
    }
    fn floor_day_at(&self, arriving: bool) -> i64 {
        let device = (self.day)();
        if crate::rules::get().clock_guard { self.guarded(device, arriving) } else { device }
    }
    /// Whether the book walks to its floor day on the first read of each new day (#38 case 13): with the guard.
    pub fn walks() -> bool {
        crate::rules::get().clock_guard
    }

    /// THE CLOCK GUARD BEFORE THE FIRST FLOOR (nodeclock::decide, the rule of #38): M raised by the Date a door named, H and Hc kept
    /// and let go as the rule says, the rule's line said when it changes; the device's day d while the floor may stand on it, none
    /// (i64::MIN) while it waits. Once a key of the book holds its floor the rule is not asked again; a book with no pipe does not
    /// ask it at all -- M is still raised, H and Hc are not started -- until a pipe arrives, and a new pipe never resets H (#38
    /// case 14, the ruling of 25.10.2026 19:23 UTC, point 3).
    fn guarded(&self, d: i64, arriving: bool) -> i64 {
        use crate::pipekey::ERASE_DAY;
        let mut g = self.guard.lock().unwrap_or_else(|e| e.into_inner());
        let st = g.get_or_insert_with(|| self.read_guard());
        let heard = (self.nodes)();
        let asked = arriving || self.has_pipe();
        if st.unreadable {
            // a record that does not read is never written over, and the guard does not withhold: rule 2 (22:04 UTC, case 18)
            if d >= ERASE_DAY && asked && !self.floored_already() {
                self.say(format!("floor_free d={d} c=none"));
            }
            return d;
        }
        // M is raised by what a node's Date named, never lowered
        if let Some(n) = heard.filter(|n| st.m.is_none_or(|m| *n > m)) {
            st.m = Some(n);
            st.dirty = true;
        }
        let c = match (st.m, heard) {
            (Some(m), Some(n)) => Some(m.max(n)),
            (m, n) => m.or(n),
        };
        let verdict = if st.done || (d >= ERASE_DAY && self.floored_already()) {
            // rule 4: once the first floor applied the rule is not asked again
            if d >= ERASE_DAY && !st.done {
                st.done = true;
                st.hold = None;
                st.dirty = true;
            }
            None
        } else if !asked {
            None // a book with no pipe does not ask the rule (case 14)
        } else {
            #[cfg(test)]
            {
                self.seen.lock().unwrap().asks += 1;
            }
            let (verdict, next) = crate::nodeclock::decide(d, c, st.hold);
            if next != st.hold {
                st.hold = next;
                st.dirty = true;
            }
            if verdict.floors() && d >= ERASE_DAY {
                st.done = true; // kept beside the book even when the book's own write is refused (case 15)
                st.hold = None;
                st.dirty = true;
            }
            Some(verdict)
        };
        if st.dirty && self.vault.set(GUARD_RECORD, Self::guard_form(st).as_bytes()) {
            st.dirty = false; // a refused write is tried again at the next decision
        }
        match verdict {
            None if st.done || d < ERASE_DAY || asked => d,
            None => i64::MIN,
            Some(v) => {
                if let Some(line) = v.line(d, c) {
                    let held = matches!(v, crate::nodeclock::Guard::Held { .. });
                    let mut last = self.held_said.lock().unwrap_or_else(|e| e.into_inner());
                    if !held || *last != line {
                        if held {
                            *last = line.clone();
                        }
                        drop(last);
                        self.say(line);
                    }
                }
                if v.floors() || v == crate::nodeclock::Guard::Before { d } else { i64::MIN }
            }
        }
    }
    /// The guard's record from the vault: none yet, the record, or a record that does not read.
    fn read_guard(&self) -> GuardKept {
        if !self.vault.has(GUARD_RECORD) {
            return GuardKept::default();
        }
        let Some(v) = self.vault.get(GUARD_RECORD).and_then(|b| serde_json::from_slice::<Value>(&b).ok()).filter(|v| v.is_object()) else {
            crate::trace_line("floor_guard unreadable -- kept as it lies");
            return GuardKept { unreadable: true, ..Default::default() };
        };
        let hold = v["h"].as_i64().zip(v["hc"].as_i64());
        GuardKept { m: v["m"].as_i64(), hold, done: v["done"].as_bool().unwrap_or(false), unreadable: false, dirty: false }
    }
    fn guard_form(st: &GuardKept) -> String {
        let mut o = serde_json::json!({ "done": st.done });
        if let Some(m) = st.m {
            o["m"] = serde_json::json!(m);
        }
        if let Some((h, hc)) = st.hold {
            o["h"] = serde_json::json!(h);
            o["hc"] = serde_json::json!(hc);
        }
        o.to_string()
    }
    /// One line of the guard in the trace.
    fn say(&self, line: String) {
        crate::trace_line(&line);
        #[cfg(test)]
        self.seen.lock().unwrap().lines.push(line.clone());
        *self.guard_said.lock().unwrap_or_else(|e| e.into_inner()) = line;
    }
    /// THE DAY A RESTORE FLOORS BY (#38, 22:04 UTC, case 16): the restore asks the guard as a pipe arriving -- while the first
    /// floor waits the restored keys wait with the book; otherwise max(d, M). Without the guard, the day of the book (iOS 2311).
    pub fn restore_day(&self) -> i64 {
        let day = self.arrival_day();
        if !crate::rules::get().clock_guard || day == i64::MIN {
            return day;
        }
        let m = self.guard.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|st| st.m);
        m.map_or(day, |m| day.max(m))
    }
    /// The guard's record as a check reads it: (M, H and Hc, done), None while there is none.
    #[cfg(test)]
    pub fn guard_record(vault: &Vault) -> Option<GuardRecord> {
        let v: Value = serde_json::from_slice(&vault.get(GUARD_RECORD)?).ok()?;
        Some((v["m"].as_i64(), v["h"].as_i64().zip(v["hc"].as_i64()), v["done"].as_bool().unwrap_or(false)))
    }
    /// The wait before a refused write is tried again (a check shortens it).
    #[cfg(test)]
    pub fn set_retry_wait(&self, wait: std::time::Duration) {
        *self.retry_wait.lock().unwrap() = wait;
    }
    /// The guard's last line (a check reads it as the trace would show it).
    #[cfg(test)]
    pub fn guard_line(&self) -> String {
        self.guard_said.lock().unwrap().clone()
    }
    /// Whether the book holds a pipe: in memory once it is read, on the disk before.
    fn has_pipe(&self) -> bool {
        if let Some((_, m)) = self.keys.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
            return m.values().any(|k| k.is_whole());
        }
        self.vault.get(SECRETS).and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .is_some_and(|v| v.as_object().is_some_and(|m| !m.is_empty()))
    }
    /// Whether a key of the book holds its floor: in memory once it is read, on the disk before.
    fn floored_already(&self) -> bool {
        use std::sync::atomic::Ordering;
        match self.floored_seen.load(Ordering::SeqCst) {
            2 => return true,
            1 => return false,
            _ => {}
        }
        let held = self.keys.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(|(_, m)| m.values().any(|k| k.floor_day().is_some()));
        let seen = held.unwrap_or_else(|| {
            self.vault.get(SECRETS).and_then(|b| serde_json::from_slice::<Value>(&b).ok()).is_some_and(|v| {
                v.as_object().is_some_and(|m| m.values().filter_map(|x| x.as_str()).any(|s| PipeKey::from_sealed_b64(s, i64::MIN).is_some_and(|k| k.floor_day().is_some())))
            })
        });
        self.floored_seen.store(if seen { 2 } else { 1 }, Ordering::SeqCst);
        seen
    }

    fn secrets(&self, day: i64) -> Map<String, Value> {
        self.secrets_back.call_once(|| self.recover_secrets(day));
        self.secrets_now()
    }
    /// The pipes as the disk holds them.
    fn secrets_now(&self) -> Map<String, Value> {
        match read_kept::<Value>(&self.vault, SECRETS) {
            Some(Value::Object(o)) => o,
            _ => Map::new(),
        }
    }
    /// The pipes written, every one in the form of the day (iOS store: the floor applies to every write), under the one entry
    /// every build reads.
    fn write_pipes(&self, raw: &Map<String, Value>, day: i64) -> bool {
        #[cfg(test)]
        {
            if self.refuse_writes.load(std::sync::atomic::Ordering::SeqCst) {
                self.seen.lock().unwrap().refused += 1;
                return false;
            }
            self.seen.lock().unwrap().writes += 1;
        }
        let out = floored_map(raw, day);
        let floor_in = out.values().filter_map(|v| v.as_str()).any(|s| PipeKey::from_sealed_b64(s, i64::MIN).is_some_and(|k| k.floor_day().is_some()));
        if floor_in {
            self.floored_seen.store(2, std::sync::atomic::Ordering::SeqCst);
        }
        let body = Value::Object(out).to_string();
        let ok = self.vault.set(SECRETS, body.as_bytes());
        if !ok {
            crate::trace_line(&format!("pipes_write_refused n={}", raw.len()));
        }
        ok
    }

    /// A BOOK LAID ASIDE COMES BACK (read_kept): the secrets and the feeds that open again are merged into the book that stands,
    /// written in the day's form, and only then let go; one that still does not open stays aside for the next start. The secrets
    /// are read under the caller's lock, if it holds one: this takes none.
    fn recover_secrets(&self, day: i64) {
        for name in self.vault.asides(SECRETS) {
            let Some(Value::Object(old)) = self.vault.get(&name).and_then(|b| serde_json::from_slice::<Value>(&b).ok()) else { continue };
            let mut s = self.secrets_now();
            for (k, v) in old {
                s.entry(k).or_insert(v);
            }
            if self.write_pipes(&s, day) {
                self.vault.delete(&name);
            }
        }
    }
    fn recover(&self, chats: &mut Vec<Chat>) {
        for name in self.vault.asides(CHATS) {
            let Some(old) = self.vault.get(&name).and_then(|b| serde_json::from_slice::<Vec<Chat>>(&b).ok()) else { continue };
            *chats = merged(std::mem::take(chats), old);
            if serde_json::to_vec(&*chats).is_ok_and(|b| self.vault.set(CHATS, &b)) {
                self.vault.delete(&name);
            }
        }
    }

    /// THE PIPES AS THIS PROCESS HOLDS THEM (iOS MTPipeBook.load): read once and floored by the floor's day (floor_day); when that
    /// day moves on, floored again. A disk holding more than the day allows -- the established secret from the erase day, a floor
    /// of an earlier day -- is written again in the day's form, once; the archive's heads follow by their own marks
    /// (Desk::floor_heads).
    /// THE WALK TO THE DAY (#38 case 13, decision 188; with MONTANA_CLOCK_GUARD): on the first read of a new floor day the book is
    /// brought to that day before it is handed out -- the guard asked first, so the floor may still wait -- and written at once
    /// (the vault writes the new sealed form beside the old and then replaces it); a write the disk refuses leaves the walked book
    /// in memory, handed out as walked, and the next read writes it again. A second read of the same day changes and writes
    /// nothing, and no floor day ever goes back. Without the guard (iOS 2311) the book stays as it was read at the start.
    fn with_pipes<T>(&self, f: impl FnOnce(&BTreeMap<String, PipeKey>) -> T) -> T {
        let day = self.floor_day();
        let walks = Self::walks();
        let stands = |g: &Held| g.as_ref().is_some_and(|(d, _)| *d == day || !walks);
        let wait = *self.retry_wait.lock().unwrap_or_else(|e| e.into_inner());
        let due = || self.refused_at.lock().unwrap_or_else(|e| e.into_inner()).is_some_and(|at| at.elapsed() >= wait);
        {
            let g = self.keys.lock().unwrap_or_else(|e| e.into_inner());
            if stands(&g) && !due() {
                return f(&g.as_ref().unwrap().1);
            }
        }
        let _w = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        let mut g = self.keys.lock().unwrap_or_else(|e| e.into_inner());
        if stands(&g) && due() {
            let raw = self.secrets(day);
            let written = !PipeKey::lags(&raw, day) || self.write_pipes(&raw, day);
            *self.refused_at.lock().unwrap_or_else(|e| e.into_inner()) = (!written).then(std::time::Instant::now);
        }
        if !stands(&g) {
            let raw = self.secrets(day);
            // what the disk holds, unfloored, under what this process holds
            let mut before = PipeKey::held(&raw, i64::MIN);
            if let Some((_, held)) = g.take() {
                before.extend(held);
            }
            // the confirmation of an unanswered introduction is kept before the floor erases what it is made of (D1)
            crate::meeting::keep_confirms(&self.vault, &before);
            let now: BTreeMap<String, PipeKey> = before.iter().map(|(r, k)| (r.clone(), k.floored(day))).collect();
            if now.values().any(|k| k.floor_day().is_some()) {
                self.floored_seen.store(2, std::sync::atomic::Ordering::SeqCst);
            }
            if PipeKey::lags(&raw, day) {
                if self.write_pipes(&raw, day) {
                    *self.refused_at.lock().unwrap_or_else(|e| e.into_inner()) = None;
                    let heads = now.iter().filter(|(r, k)| !k.holds_origin() && k.reference().as_deref() == Some(r.as_str())).count();
                    crate::trace_line(&format!("pipes_floored n={} heads={heads}", now.len()));
                } else if walks {
                    *self.refused_at.lock().unwrap_or_else(|e| e.into_inner()) = Some(std::time::Instant::now());
                }
            }
            *g = Some((day, now));
        }
        f(&g.as_ref().unwrap().1)
    }

    /// THE BIRTH OF A CORRESPONDENCE, OR ITS RETURN (iOS MTPipeBook.establish): filed under its local name, which the key derives
    /// from its own bytes -- never one handed in. A name already held answers only for the very pipe held, the floor included (iOS
    /// remember, MTPipe.swift:273). With MONTANA_D3_SAME_PIPE (D3, decision 188, the ruling on #37) it answers for the same pipe in any
    /// form, proven by what it derives (PipeKey::same_pipe), and of two forms the one further down the walk stays (a floor over
    /// the secret, the later floor): the name of another pipe is refused either way. What is written is the day's form; a pipe
    /// born now joins the book as the book stands (iOS), or walked with it to its floor day (#38 case 13). The name.
    pub fn establish(&self, key: &PipeKey) -> Option<String> {
        let r = key.reference()?;
        self.with_pipes(|_| ()); // the book stands in memory before a birth
        let day = self.arrival_day();
        let _w = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        let mut raw = self.secrets(day);
        let cached = self.keys.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|(_, m)| m.get(&r).cloned());
        let have = cached.or_else(|| raw.get(&r).and_then(|v| v.as_str()).and_then(|s| PipeKey::from_sealed_b64(s, day)));
        let rules = crate::rules::get();
        let keep = match &have {
            Some(h) if rules.d3_same_pipe && !h.same_pipe(key) => {
                crate::trace_line(&format!("pipes_establish refused ref={} -- another pipe under this name", &r[..10]));
                return None;
            }
            Some(h) if rules.d3_same_pipe => PipeKey::forward(h, key),
            // iOS 2311 remember: the pipe held must be the one met, its form included; nothing is written
            Some(h) => return (h == key).then_some(r),
            None => key.clone(),
        };
        let sealed = keep.sealed_b64(day);
        if raw.get(&r).and_then(|v| v.as_str()) != Some(sealed.as_str()) || PipeKey::lags(&raw, day) {
            raw.insert(r.clone(), Value::String(sealed));
            if !self.write_pipes(&raw, day) {
                // a birth the disk refused is not held (iOS store: the cache is not replaced); a pipe that stood still stands
                return have.map(|_| r);
            }
        }
        if let Some((_, m)) = self.keys.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
            m.insert(r.clone(), if Self::walks() { keep.floored(day) } else { keep });
        }
        Some(r)
    }

    fn heal_map(&self) -> Option<BTreeMap<String, HealSide>> {
        let mut cached = self.heals.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(map) = cached.as_ref() {
            return Some(map.clone());
        }
        let read = if !self.vault.has(HEALS) {
            Some(BTreeMap::new())
        } else {
            self.vault.get(HEALS).and_then(|bytes| serde_json::from_slice::<BTreeMap<String, HealSide>>(&bytes).ok())
        };
        match read {
            Some(map) => {
                *cached = Some(map.clone());
                Some(map)
            }
            None => {
                crate::trace_line("pipes_heal_unreadable -- kept as it lies");
                None
            }
        }
    }

    fn store_heals(&self, map: &BTreeMap<String, HealSide>) -> bool {
        let Ok(bytes) = serde_json::to_vec(map) else { return false };
        if !self.vault.set(HEALS, &bytes) {
            crate::trace_line(&format!("pipes_heal_write_refused n={}", map.len()));
            return false;
        }
        *self.heals.lock().unwrap_or_else(|e| e.into_inner()) = Some(map.clone());
        true
    }

    fn healing_for(&self, r: &str) -> Option<crate::pipeheal::Healing> {
        self.heal_map().and_then(|map| map.get(r).and_then(|side| side.healing.clone()))
    }

    /// The pipe of a correspondence as its one owner holds it: it answers what the pipe derives and hands out no byte of it.
    pub fn key(&self, r: &str) -> Option<PipeKey> {
        let healing = self.healing_for(r);
        self.with_pipes(|m| m.get(r).filter(|k| k.is_whole()).cloned()).map(|key| key.with_healing(healing))
    }
    /// A correspondence this machine can answer for.
    pub fn holds(&self, r: &str) -> bool {
        self.with_pipes(|m| m.get(r).is_some_and(|k| k.is_whole()))
    }
    pub fn refs(&self) -> Vec<String> {
        self.with_pipes(|m| m.iter().filter(|(_, k)| k.is_whole()).map(|(r, _)| r.clone()).collect())
    }
    /// Every pipe held, by its name, with its heal in flight attached.
    pub fn pipes(&self) -> Vec<(String, PipeKey)> {
        self.with_pipes(|m| m.iter().filter(|(_, k)| k.is_whole()).map(|(r, k)| (r.clone(), k.clone())).collect::<Vec<_>>())
            .into_iter().map(|(r, key)| {
                let healing = self.healing_for(&r);
                (r, key.with_healing(healing))
            }).collect()
    }

    fn replace_pipe(&self, r: &str, key: PipeKey) -> bool {
        let day = self.floor_day();
        let key = key.floored(day);
        let _write = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        let mut raw = self.secrets(day);
        raw.insert(r.to_string(), Value::String(key.sealed_b64(day)));
        if !self.write_pipes(&raw, day) {
            return false;
        }
        if let Some((_, map)) = self.keys.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
            map.insert(r.to_string(), key);
        }
        self.floored_seen.store(2, std::sync::atomic::Ordering::SeqCst);
        true
    }

    /// A pipe letter opened under a walk: the heal hears it, then reads the letter's ninth field.
    /// Returns whether the pipe was rebased, so the postman can append its new archive head.
    pub fn heal_read(&self, r: &str, plain: &[u8], walk: HealWalk, window: i64) -> bool {
        let part = crate::wire::letter_heal(plain).and_then(|field| HealPart::parse(&field));
        if part.is_none() && walk == HealWalk::Old {
            let waiting = self.heal_map().and_then(|map| map.get(r).and_then(|side| side.healing.clone()))
                .is_some_and(|healing| !healing.holds_old);
            if !waiting {
                return false;
            }
        }
        let _step = self.heal_lock.lock().unwrap_or_else(|e| e.into_inner());
        if !self.with_pipes(|map| map.contains_key(r)) {
            return false;
        }
        let Some(mut map) = self.heal_map() else { return false };
        let mut side = map.get(r).cloned().unwrap_or_default();
        let before = side.clone();
        if let Some(healing) = &mut side.healing {
            healing.opened(walk);
        }
        let mut rebased = false;
        if side.healing.as_ref().is_some_and(|healing| !healing.holds_old) {
            let pipe = self.with_pipes(|map| map.get(r).cloned());
            let next = pipe.and_then(|pipe| pipe.with_healing(side.healing.clone()).rebased());
            if let Some(next) = next {
                if self.replace_pipe(r, next) {
                    side.healing = None;
                    side.rebased = true;
                    rebased = true;
                    crate::trace_line("pipe_healed");
                }
            }
        }
        if let Some(part) = part {
            if let Some(pipe) = self.with_pipes(|map| map.get(r).cloned()) {
                side.received(&part, window.div_euclid(crate::pipekey::MINUTES_PER_DAY), &pipe);
            }
        }
        if side != before {
            map.insert(r.to_string(), side);
            self.store_heals(&map);
        }
        rebased
    }

    /// ANOTHER PERSON'S PIPES, READ AND NEVER WRITTEN (a seat's vault, seats::pair_with): both entries, floored by the day in hand,
    /// the disk left as it lies -- no rewrite, no aside.
    pub fn peek(vault: &Vault, day: i64) -> Vec<PipeKey> {
        let m = match vault.get(SECRETS).and_then(|b| serde_json::from_slice::<Value>(&b).ok()) {
            Some(Value::Object(m)) => m,
            _ => Map::new(),
        };
        PipeKey::held(&m, day).into_values().filter(|k| k.is_whole()).collect()
    }
    fn loaded(&self) -> std::sync::MutexGuard<'_, Option<Vec<Chat>>> {
        let mut g = self.chats.lock().unwrap();
        if g.is_none() {
            let mut chats: Vec<Chat> = read_kept(&self.vault, CHATS).unwrap_or_default();
            self.recover(&mut chats);
            *g = Some(chats);
        }
        g
    }

    /// The feeds as a change left them (`n`, numbered under their lock), laid on disk unless a later change is there already.
    fn keep(&self, n: u64, chats: &[Chat]) {
        let mut on_disk = self.on_disk.lock().unwrap();
        if n <= *on_disk {
            return;
        }
        if let Ok(b) = serde_json::to_vec(chats) {
            if self.vault.set(CHATS, &b) {
                *on_disk = n;
            }
        }
    }
    fn number(&self) -> u64 {
        self.changes.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
    }

    pub fn all(&self) -> Vec<Chat> {
        self.loaded().clone().unwrap_or_default()
    }

    pub fn chat(&self, r: &str) -> Option<Chat> {
        self.loaded().as_ref()?.iter().find(|c| c.reference == r).cloned()
    }

    /// The name a chat is shown by (iOS displayName: my own word for them first, else the name they gave), its letters left
    /// where they are -- many rows read it at once (the wall's bylines).
    pub fn shown_name(&self, r: &str) -> Option<String> {
        self.loaded().as_ref()?.iter().find(|c| c.reference == r).map(|c| if c.alias.trim().is_empty() { c.name.clone() } else { c.alias.clone() })
    }

    /// The name a correspondent gave themselves (iOS MTNameBook.declared) -- never my own word for them.
    pub fn given_name(&self, r: &str) -> Option<String> {
        self.loaded().as_ref()?.iter().find(|c| c.reference == r).map(|c| c.name.clone())
    }

    /// The chat stands (or its empty name takes the one given); `perm` -- met through a permanent link.
    pub fn open(&self, r: &str, name: &str, perm: bool) {
        self.edit(r, |c| {
            if c.name.is_empty() && !name.is_empty() {
                c.name = name.to_string();
            }
            c.perm |= perm;
        });
    }

    /// One change to one chat, made and kept whole; the chat is born if it is not there.
    pub fn edit(&self, r: &str, f: impl FnOnce(&mut Chat)) {
        let mut g = self.loaded();
        let chats = g.as_mut().unwrap();
        let i = match chats.iter().position(|c| c.reference == r) {
            Some(i) => i,
            None => {
                chats.push(Chat { reference: r.to_string(), ..Default::default() });
                chats.len() - 1
            }
        };
        f(&mut chats[i]);
        let (n, snapshot) = (self.number(), chats.clone());
        drop(g);
        self.keep(n, &snapshot);
    }

    /// One correspondence leaves whole: its secret and its feed (after my erasure for both was heard); the blocks another
    /// device of the person lays for it are not filed here again (iOS deletedCold).
    pub fn drop_chat(&self, r: &str) {
        let day = self.floor_day();
        {
            let _g = self.lock.lock().unwrap();
            let mut gone: Map<String, Value> = self.vault.get(crate::twin::GONE).and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
            gone.insert(crate::wire::hex(&crate::wire::sha(&[r.as_bytes()])), Value::Bool(true));
            while gone.len() > 4000 {
                let k = gone.keys().next().cloned().unwrap();
                gone.remove(&k);
            }
            self.vault.set(crate::twin::GONE, Value::Object(gone).to_string().as_bytes());
            let mut s = self.secrets(day);
            if s.remove(r).is_some() {
                self.write_pipes(&s, day);
            }
            if let Some((_, m)) = self.keys.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
                m.remove(r);
            }
            // an introduction still waiting dies with its pipe: its kept confirmation is never another pipe's (D1)
            crate::meeting::first_done(&self.vault, r);
        }
        if let Some(mut map) = self.heal_map() {
            if map.remove(r).is_some() {
                self.store_heals(&map);
            }
        }
        let mut g = self.loaded();
        let chats = g.as_mut().unwrap();
        chats.retain(|c| c.reference != r);
        let (n, snapshot) = (self.number(), chats.clone());
        drop(g);
        self.keep(n, &snapshot);
    }

    /// The identity left: every in-memory owner of its pipes, heals, feeds and floor guard lets go.
    /// The sealed store is the seat's and stays untouched, as iOS MTPipeBook.forgetAll.
    pub fn forget_all(&self) {
        let _heal = self.heal_lock.lock().unwrap_or_else(|e| e.into_inner());
        let _write = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        *self.keys.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *self.heals.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *self.chats.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *self.guard.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *self.guard_said.lock().unwrap_or_else(|e| e.into_inner()) = String::new();
        *self.held_said.lock().unwrap_or_else(|e| e.into_inner()) = String::new();
        *self.refused_at.lock().unwrap_or_else(|e| e.into_inner()) = None;
        self.floored_seen.store(0, std::sync::atomic::Ordering::SeqCst);
        crate::pipekey::forget_walks();
    }

    #[cfg(test)]
    pub fn caches_loaded(&self) -> (bool, bool, bool) {
        (
            self.keys.lock().unwrap_or_else(|e| e.into_inner()).is_some(),
            self.heals.lock().unwrap_or_else(|e| e.into_inner()).is_some(),
            self.chats.lock().unwrap_or_else(|e| e.into_inner()).is_some(),
        )
    }
}

/// WHO A SEAT'S FOLDER BELONGS TO, as the book can tell : None when the folder holds no person at all;
/// Some(Some(twin)) when a store stands and the words' name for its owner lies there whole; Some(None) when a store
/// stands and nobody can be named. Only a person's own keys count: a device's marks and the seats' own book are not
/// one, so the drawer's folder is not mistaken for a person's.
pub fn store_owner(vault: &Vault) -> Option<Option<String>> {
    let stored = crate::seed::Seed::stored_twin(vault);
    let derived = vault.get(crate::seed::MN).and_then(|raw| String::from_utf8(raw).ok()).and_then(|words| crate::seed::twin_of(&words));
    let twin = match (derived, stored) {
        (Some(from_words), Some(stored)) if from_words == stored => Some(from_words),
        (Some(_), Some(_)) => None, // MN and TWIN name two people: the owner cannot be told safely
        (Some(from_words), None) => Some(from_words),
        (None, stored) => stored, // MN is missing or damaged: the stored twin is the owner's remaining name
    };
    // a person's own marks beside the book's: a face, the drafts, the flow of presence, the card's stamp, the mirror's
    let own = [
        SECRETS, CHATS, HEALS, GUARD_RECORD, "selfFace", "draftsMap", "moneyFlow", "cardFaceUp", "archiveFilled", "passwords", "vpnBook", "coinLedger", "coin3",
    ];
    let holds = twin.is_some() || vault.has(crate::seed::MN) || own.iter().any(|name| vault.has(name));
    holds.then_some(twin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unreadable_book_is_laid_aside_never_written_over_and_comes_back_when_it_opens() {
        let d = std::env::temp_dir().join(format!("montana-book-aside-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let v = Vault::at(&d).unwrap();
        let b = Book::new(v.clone());
        let secret = vec![5u8; 32];
        let r = b.establish(&crate::pipekey::PipeKey::new(&secret)).unwrap();
        let mid = mint_mid();
        b.edit(&r, |c| c.msgs.push(Msg { mid: mid.clone(), text: "старое".into(), mine: true, at: birth_ms(&mid).unwrap(), ..Default::default() }));
        let (good_chats, good_secrets) = (v.get(CHATS).unwrap(), v.get(SECRETS).unwrap());
        // the machine's seal refuses the feeds, the secrets do not parse: a start reads nothing -- and moves both aside
        std::fs::write(d.join(format!("{CHATS}.sealed")), b"not sealed by this machine").unwrap();
        v.set(SECRETS, b"{broken").then_some(()).unwrap();
        let b2 = Book::new(v.clone());
        assert!(b2.all().is_empty() && !b2.holds(&r), "nothing readable stands");
        assert_eq!((v.asides(CHATS).len(), v.asides(SECRETS).len()), (1, 1), "both laid aside, not deleted");
        let other = vec![6u8; 32];
        let r2 = b2.establish(&crate::pipekey::PipeKey::new(&other)).unwrap();
        b2.edit(&r2, |c| c.name = "новый".into());
        // the bytes open again (the seal came back): the next start merges them into the book that stands, and lets them go
        for (name, good) in [(v.asides(CHATS)[0].clone(), good_chats), (v.asides(SECRETS)[0].clone(), good_secrets)] {
            assert!(v.set(&name, &good));
        }
        let b3 = Book::new(v.clone());
        assert!(b3.key(&r).unwrap().same_pipe(&PipeKey::new(&secret)));
        assert!(b3.key(&r2).unwrap().same_pipe(&PipeKey::new(&other)));
        assert!(b3.chat(&r).unwrap().msgs.iter().any(|m| m.mid == mid), "the old letter came back");
        assert_eq!(b3.chat(&r2).unwrap().name, "новый", "the newer book's chat stands");
        assert!(v.asides(CHATS).is_empty() && v.asides(SECRETS).is_empty(), "merged, then let go");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_pipe_is_filed_by_its_secret_and_its_feed_survives_a_restart() {
        let d = std::env::temp_dir().join(format!("montana-book-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let b = Book::new(Vault::at(&d).unwrap());
        let secret = vec![7u8; 32];
        let r = b.establish(&crate::pipekey::PipeKey::new(&secret)).unwrap();
        assert_eq!(Some(r.clone()), PipeKey::new(&secret).reference());
        b.open(&r, "Боб", true);
        let mid = mint_mid();
        b.edit(&r, |c| c.msgs.push(Msg { mid: mid.clone(), text: "привет".into(), mine: true, at: birth_ms(&mid).unwrap(), state: 1, ..Default::default() }));
        let again = Book::new(Vault::at(&d).unwrap());
        assert!(again.key(&r).unwrap().same_pipe(&PipeKey::new(&secret)));
        let c = again.chat(&r).unwrap();
        assert_eq!((c.name.as_str(), c.perm, c.msgs.len()), ("Боб", true, 1));
        assert!(mid.starts_with('t') && birth_ms(&mid).unwrap() > 1_700_000_000_000);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn changes_made_at_once_are_all_on_disk_after_a_restart() {
        let d = std::env::temp_dir().join(format!("montana-book-race-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let b = std::sync::Arc::new(Book::new(Vault::at(&d).unwrap()));
        let hands: Vec<_> = (0..8).map(|t| {
            let b = b.clone();
            std::thread::spawn(move || {
                for i in 0..25 {
                    let r = format!("chat{}", t % 3);
                    b.edit(&r, |c| c.msgs.push(Msg { mid: format!("m{t}-{i}"), text: "письмо".into(), mine: true, ..Default::default() }));
                }
            })
        }).collect();
        hands.into_iter().for_each(|h| h.join().unwrap());
        // the program stopped and started again: the disk holds every letter of every hand
        let again = Book::new(Vault::at(&d).unwrap());
        let n: usize = again.all().iter().map(|c| c.msgs.len()).sum();
        assert_eq!(n, 8 * 25, "no change lost to an older one written last");
        let left = std::fs::read_dir(&d).unwrap().flatten().filter(|e| e.path().extension().is_some_and(|x| x.to_string_lossy().starts_with("tmp"))).count();
        assert_eq!(left, 0, "no file laid aside is left behind");
        let _ = std::fs::remove_dir_all(&d);
    }

    use crate::pipekey::{ERASE_DAY, FLOOR_LAG};
    use crate::rules::{self, Rules};
    use std::sync::atomic::{AtomicI64, Ordering};

    /// A book whose day is a cell of the test's own: never the computer's clock.
    fn on_day(v: &Vault, d: i64) -> (Book, Arc<AtomicI64>) {
        let day = Arc::new(AtomicI64::new(d));
        let cell = day.clone();
        (Book::new_on(v.clone(), Arc::new(move || cell.load(Ordering::SeqCst))), day)
    }
    fn dir(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("montana-book-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }
    fn s(n: u8) -> Vec<u8> {
        (0..32).map(|i| i ^ n).collect()
    }
    /// Whether the disk's book holds the secret: its bytes or their base64.
    fn disk_holds(v: &Vault, secret: &[u8]) -> bool {
        let b = v.get(SECRETS).unwrap_or_default();
        b.windows(32).any(|w| w == secret) || String::from_utf8_lossy(&b).contains(&crate::wire::b64(secret))
    }
    /// The two settings every proposal is tested under: iOS 2311 (every switch off) and every switch on.
    const BOTH: [Rules; 2] = [Rules::IOS, Rules::ALL];

    /// FROM THE ERASE DAY THE BOOK READS ITSELF FLOORED AND WRITES ITSELF AGAIN ONCE (iOS MTPipeBook.load + floorStore): before it
    /// the disk keeps the 32 bytes; a start on 03.11 holds the floor of 01.11 and the disk is rewritten, once, under the one
    /// entry every build reads (iOS); a read after it changes nothing; the names, the feeds and what each pipe keeps for good
    /// stand.
    #[test]
    fn the_book_floors_by_its_day_and_rewrites_itself_once_when_it_lags() {
        for r in BOTH {
            rules::with(r, || {
                let d = dir(&format!("lag-{}", r.clock_guard));
                let v = Vault::at(&d).unwrap();
                let (b, _) = on_day(&v, ERASE_DAY - 1);
                let (a, c) = (PipeKey::new(&s(1)), PipeKey::new(&s(2)));
                let (ra, rc) = (b.establish(&a).unwrap(), b.establish(&c).unwrap());
                b.open(&ra, "Боб", false);
                assert!(disk_holds(&v, &s(1)) && b.key(&ra).unwrap().holds_origin(), "before the erase day the secret is kept");
                let (b2, _) = on_day(&v, ERASE_DAY);
                let k = b2.key(&ra).unwrap();
                assert_eq!((k.holds_origin(), k.floor_day()), (false, Some(ERASE_DAY - FLOOR_LAG)));
                assert!(!disk_holds(&v, &s(1)) && !disk_holds(&v, &s(2)), "the disk is written again in the floor's form");
                let once = v.get(SECRETS).unwrap();
                assert!(!PipeKey::lags(&serde_json::from_slice(&once).unwrap(), ERASE_DAY));
                let (b3, _) = on_day(&v, ERASE_DAY);
                assert_eq!(b3.refs().len(), 2);
                assert_eq!(v.get(SECRETS).unwrap(), once, "a start on the same day writes nothing");
                assert!(k.same_pipe(&a) && k.kept() == a.kept() && b3.key(&rc).unwrap().same_pipe(&c));
                assert_eq!(b3.chat(&ra).unwrap().name, "Боб", "the feed is not touched");
                let _ = std::fs::remove_dir_all(&d);
            });
        }
    }

    /// A PIPE BORN ON OR AFTER THE ERASE DAY: the disk takes its floor from the birth either way; the book takes it as the book
    /// stands -- iOS 2311's book as read at the start (MTPipe.swift:255-263), or the walked book of #38 case 13, which follows
    /// the floor day as it moves on; its first letter's confirmation is the meeting's (D1).
    #[test]
    fn a_newborn_pipe_on_or_after_the_erase_day_joins_the_book_as_it_stands() {
        for r in BOTH {
            rules::with(r, || {
                let d = dir(&format!("newborn-{}", Book::walks()));
                let v = Vault::at(&d).unwrap();
                let (b, day) = on_day(&v, ERASE_DAY + 1);
                let n = PipeKey::new(&s(3));
                let rn = b.establish(&n).unwrap();
                crate::meeting::remember_first(&v, &rn, &n, &[7; 1088], &[1; 32]);
                let held = b.key(&rn).unwrap();
                assert_eq!(held.holds_origin(), !Book::walks(), "the book as read (iOS) or walked");
                assert!(held.same_pipe(&n) || held == n);
                assert_eq!(crate::meeting::confirm(&v, &rn, &held, &[7; 1088]), n.first_confirm(&[7; 1088]), "its first letter confirmed");
                assert!(!disk_holds(&v, &s(3)), "the disk takes the floor from the birth");
                let (again, _) = on_day(&v, ERASE_DAY + 1);
                assert_eq!(again.key(&rn).unwrap().floor_day(), Some(ERASE_DAY + 1 - FLOOR_LAG), "a restart reads the floor");
                day.store(ERASE_DAY + 2, Ordering::SeqCst);
                let turned = b.key(&rn).unwrap();
                assert_eq!(turned.floor_day(), Book::walks().then_some(ERASE_DAY), "the day turned: iOS keeps the run's book");
                let _ = std::fs::remove_dir_all(&d);
            });
        }
    }

    /// A PROCESS ACROSS MIDNIGHT: a book read on 02.11 that lives on into 03.11 walks to the new day on its first read of it and
    /// writes the disk before it hands itself out (#38 case 13), the confirmation of an introduction still waiting kept first
    /// (D1); iOS 2311's book stays as read until the next start.
    #[test]
    fn a_book_living_across_the_erase_day_floors_at_its_next_start_or_when_the_day_turns() {
        for r in BOTH {
            rules::with(r, || {
                let d = dir(&format!("midnight-{}", Book::walks()));
                let v = Vault::at(&d).unwrap();
                let (b, day) = on_day(&v, ERASE_DAY - 1);
                let a = PipeKey::new(&s(4));
                let ra = b.establish(&a).unwrap();
                let ct = vec![7u8; 1088];
                // an introduction recorded by an earlier build: no confirmation beside it
                v.set("pipeFirst", serde_json::json!({ ra.clone(): { "ct": crate::wire::b64(&ct), "inv": crate::wire::b64(&[1u8; 32]) } }).to_string().as_bytes());
                assert!(b.key(&ra).unwrap().holds_origin());
                day.store(ERASE_DAY, Ordering::SeqCst);
                let k = b.key(&ra).unwrap();
                assert_eq!(k.floor_day(), Book::walks().then_some(ERASE_DAY - FLOOR_LAG));
                assert_eq!(disk_holds(&v, &s(4)), !Book::walks(), "the disk follows the day, or waits for the next start (iOS)");
                let (next, _) = on_day(&v, ERASE_DAY);
                let f = next.key(&ra).unwrap();
                assert_eq!(f.floor_day(), Some(ERASE_DAY - FLOOR_LAG), "the next start floors either way");
                assert!(!disk_holds(&v, &s(4)));
                let kept = if r.d1_keep_confirm { a.first_confirm(&ct) } else { String::new() };
                assert_eq!(crate::meeting::confirm(&v, &ra, &f, &ct), kept, "the confirmation kept before the floor (D1), or none (iOS)");
                let _ = std::fs::remove_dir_all(&d);
            });
        }
    }

    /// A NAME COUNTS ONLY FOR ITS PIPE. iOS 2311 (remember, MTPipe.swift:273): the very pipe held, its form included -- the same
    /// pipe again before the floor is the one chat; another form of it (its floor from another installation, the 32 bytes of an
    /// old archive after the floor) is refused. D3 (MONTANA_D3_SAME_PIPE, #37, decision 188): the same pipe in any form is
    /// the one chat, the forward form kept and never rolled back. Either way another pipe carrying this name is refused, and two
    /// people of one name are two pipes, no duplicates.
    #[test]
    fn a_name_counts_for_its_pipe_as_ios_compares_it_or_by_what_it_derives() {
        for r in BOTH {
            rules::with(r, || {
                let d = dir(&format!("d3-{}", r.d3_same_pipe));
                let v = Vault::at(&d).unwrap();
                let (b, day) = on_day(&v, ERASE_DAY - 1);
                let a = PipeKey::new(&s(5));
                let ra = b.establish(&a).unwrap();
                assert_eq!(b.establish(&a).as_deref(), Some(ra.as_str()), "a repeated meeting of the same pipe is the same chat");
                assert_eq!(b.refs().len(), 1, "no duplicate");
                // the person's other installation hands the floor of the same pipe
                let floor = a.floored(ERASE_DAY);
                if r.d3_same_pipe {
                    assert_eq!(b.establish(&floor).as_deref(), Some(ra.as_str()));
                    assert_eq!(b.key(&ra).unwrap().floor_day(), Some(ERASE_DAY - FLOOR_LAG), "taken forward");
                    assert_eq!(b.establish(&a).as_deref(), Some(ra.as_str()), "an old archive's 32 bytes: the same chat");
                    assert_eq!(b.key(&ra).unwrap().floor_day(), Some(ERASE_DAY - FLOOR_LAG), "never rolled back to the secret");
                    assert!(!disk_holds(&v, &s(5)));
                } else {
                    assert_eq!(b.establish(&floor), None, "iOS: another form of the pipe is not the pipe held");
                    assert!(b.key(&ra).unwrap().holds_origin());
                }
                // another pipe under this pipe's name: refused, the pipe that stands is untouched
                let mut raw = crate::wire::unb64(&PipeKey::new(&s(6)).floored(ERASE_DAY).sealed_b64(ERASE_DAY)).unwrap();
                raw[crate::pipekey::FLOOR_FORM_SIZE - 16..].copy_from_slice(&crate::wire::unhex(&ra).unwrap());
                let impostor = PipeKey::from_sealed_b64(&crate::wire::b64(&raw), ERASE_DAY).unwrap();
                assert_eq!(impostor.reference().as_deref(), Some(ra.as_str()));
                assert_eq!(b.establish(&impostor), None, "a name alone is not a pipe");
                assert!(b.key(&ra).unwrap().same_pipe(&a));
                // two people who gave themselves one name: two pipes, two chats
                let (x, y) = (PipeKey::new(&s(7)), PipeKey::new(&s(8)));
                let (rx, ry) = (b.establish(&x).unwrap(), b.establish(&y).unwrap());
                b.open(&rx, "Боб", false);
                b.open(&ry, "Боб", false);
                assert_ne!(rx, ry);
                assert_eq!(b.refs().len(), 3);
                // after the erase day, a fresh start: a repeated meeting of x by its 32 bytes is the same chat with D3, refused by iOS
                day.store(ERASE_DAY + 1, Ordering::SeqCst);
                let (after, _) = on_day(&v, ERASE_DAY + 1);
                assert_eq!(after.establish(&x).is_some(), r.d3_same_pipe, "iOS 2311 does not know the earlier pipe in its old form");
                assert_eq!(after.establish(&x.floored(ERASE_DAY + 1)).as_deref(), Some(rx.as_str()), "its own floor of the day: the same");
                assert_eq!(after.refs().len(), 3, "no duplicates after the floor");
                // a floor of x differing in any one kept quantity but its name: refused either way, x untouched (#37)
                for i in 0..7 {
                    let mut form = crate::wire::unb64(&x.floored(ERASE_DAY + 1).sealed_b64(ERASE_DAY + 1)).unwrap();
                    form[9 + 32 * i] ^= 0x01;
                    let odd = PipeKey::from_sealed_b64(&crate::wire::b64(&form), ERASE_DAY + 1).unwrap();
                    assert_eq!(odd.reference().as_deref(), Some(rx.as_str()));
                    assert_eq!(after.establish(&odd), None, "field {i} differs: refused");
                }
                assert!(after.key(&rx).unwrap().same_pipe(&x));
                // THE ROAD OF AN ARCHIVE HEAD (iOS establish(pipe:) from init?(archiveHead:), MTPipeKey.swift:276-281): a head another
                // device floored on 03.11, read on 04.11, is floored by the day and is the pipe held; a head floored on a later day than
                // the book's own is the same pipe further down -- taken forward with D3, refused by iOS's equality
                let head = String::from_utf8(x.floored(ERASE_DAY).archive_head()).unwrap();
                let read = PipeKey::from_archive_head(&head, after.floor_day()).unwrap();
                assert_eq!(after.establish(&read).as_deref(), Some(rx.as_str()), "a head of another day, floored by this one: the same");
                let later = String::from_utf8(x.floored(ERASE_DAY + 4).archive_head()).unwrap();
                let ahead = PipeKey::from_archive_head(&later, after.floor_day()).unwrap();
                assert_eq!(after.establish(&ahead).is_some(), r.d3_same_pipe);
                assert_eq!(after.key(&rx).unwrap().floor_day(), Some(if r.d3_same_pipe { ERASE_DAY + 2 } else { ERASE_DAY - 1 }));
                let _ = std::fs::remove_dir_all(&d);
            });
        }
    }

    /// A pipe dropped takes its introduction's record with it (a kept confirmation is never another pipe's); another person's
    /// vault is read and never written (seats::pair_with).
    #[test]
    fn a_dropped_pipe_takes_its_introduction_and_another_vault_is_only_read() {
        let d = dir("drop");
        let v = Vault::at(&d.join("a")).unwrap();
        let (b, _) = on_day(&v, ERASE_DAY - 1);
        let a = PipeKey::new(&s(9));
        let r = b.establish(&a).unwrap();
        crate::meeting::remember_first(&v, &r, &a, &[7; 1088], &[1; 32]);
        assert!(crate::meeting::first(&v, &r).is_some());
        b.drop_chat(&r);
        assert!(crate::meeting::first(&v, &r).is_none() && !b.holds(&r));
        let theirs = Vault::at(&d.join("b")).unwrap();
        let (tb, _) = on_day(&theirs, ERASE_DAY - 1);
        let t = PipeKey::new(&s(10));
        tb.establish(&t).unwrap();
        let before = theirs.get(SECRETS).unwrap();
        let seen = Book::peek(&theirs, ERASE_DAY);
        assert!(seen.len() == 1 && seen[0].same_pipe(&t) && !seen[0].holds_origin(), "read in the form of the day");
        assert_eq!(theirs.get(SECRETS).unwrap(), before, "and never written");
        let _ = std::fs::remove_dir_all(&d);
    }

    /// A book laid aside with floors in it comes back merged and in the form of the day: nothing lost, no secret kept.
    #[test]
    fn a_book_laid_aside_with_floors_comes_back_in_the_form_of_the_day() {
        for r in BOTH {
            rules::with(r, || {
                let entry = SECRETS;
                let d = dir(&format!("aside-floor-{}", r.clock_guard));
                let v = Vault::at(&d).unwrap();
                let (b, _) = on_day(&v, ERASE_DAY);
                let (a, c) = (PipeKey::new(&s(11)), PipeKey::new(&s(12)));
                let ra = b.establish(&a).unwrap();
                let good = v.get(entry).unwrap();
                v.set(entry, b"{broken").then_some(()).unwrap();
                let (b2, _) = on_day(&v, ERASE_DAY);
                assert!(!b2.holds(&ra) && v.asides(entry).len() == 1);
                let rc = b2.establish(&c).unwrap();
                assert!(v.set(&v.asides(entry)[0].clone(), &good));
                let (b3, _) = on_day(&v, ERASE_DAY + 2);
                assert_eq!(b3.key(&ra).unwrap().floor_day(), Some(ERASE_DAY), "merged back and floored by the day");
                assert!(b3.key(&rc).unwrap().same_pipe(&c));
                assert!(v.asides(entry).is_empty() && !disk_holds(&v, &s(11)) && !disk_holds(&v, &s(12)));
                let _ = std::fs::remove_dir_all(&d);
            });
        }
    }

    /// A book whose day and whose nodes' Date day are the test's own.
    fn at(v: &Vault, d: i64, heard: Option<i64>) -> Book {
        Book::new_with(v.clone(), Arc::new(move || d), Arc::new(move || heard))
    }
    const GUARD: Rules = Rules { clock_guard: true, ..Rules::IOS };
    /// A vault with one pipe written before the erase day (its 32 bytes on the disk), and that pipe's name.
    fn with_pipe(name: &str, n: u8) -> (std::path::PathBuf, Vault, String) {
        let d = dir(name);
        let v = Vault::at(&d).unwrap();
        let r = at(&v, ERASE_DAY - 1, None).establish(&PipeKey::new(&s(n))).unwrap();
        (d, v, r)
    }
    /// The floor's day a book on d holds the pipe at (None: not floored).
    fn floor_of(v: &Vault, r: &str, d: i64, heard: Option<i64>) -> (Option<i64>, String) {
        let b = at(v, d, heard);
        let f = b.key(r).unwrap().floor_day();
        (f, b.guard_line())
    }
    fn hold(v: &Vault) -> Option<(i64, i64)> {
        Book::guard_record(v).and_then(|x| x.1)
    }
    fn m_of(v: &Vault) -> Option<i64> {
        Book::guard_record(v).and_then(|x| x.0)
    }

    /// THE CLOCK GUARD'S TWELVE CHECKS ON THE BOOK (#38, the rule nodeclock::decide; decision 188), with
    /// MONTANA_CLOCK_GUARD on: what floors, what waits, what M, H and Hc the vault keeps beside the book across restarts, and the
    /// rule's lines. With the switch off every case floors on d, as iOS 2311 (the next test).
    #[test]
    fn the_clock_guard_holds_its_twelve_checks_on_the_book() {
        rules::with(GUARD, || {
            // 1: d 20760, c 20760 -- the floor on 20760 (F 20758)
            let (d1, v1, r1) = with_pipe("g1", 31);
            assert_eq!(floor_of(&v1, &r1, 20760, Some(20760)), (Some(20758), String::new()));
            assert_eq!(hold(&v1), None);
            // 2: d 20800, c 20760, nothing held -- withheld; H 20800, Hc 20760, kept beside the book
            let held = |name: &str| {
                let (d, v, r) = with_pipe(name, 32);
                let (f, line) = floor_of(&v, &r, 20800, Some(20760));
                assert_eq!((f, line.as_str()), (None, "floor_held d=20800 c=20760 since=20800 left=4"));
                assert_eq!((hold(&v), m_of(&v)), (Some((20800, 20760)), Some(20760)), "M, H, Hc beside the book");
                assert!(disk_holds(&v, &s(32)), "withheld: the disk keeps the pipe as it was");
                (d, v, r)
            };
            let (d2, v2, r2) = held("g2");
            // 3 and 12: a restart on 20801, no answer, M 20760 -- withheld, left 3, on each of three loads
            for _ in 0..3 {
                assert_eq!(floor_of(&v2, &r2, 20801, None), (None, "floor_held d=20801 c=20760 since=20800 left=3".to_string()));
            }
            // 6: the clock set to 20750 -- nothing floors, H kept; back to 20802 with c 20760 -- withheld, left 2
            assert_eq!(floor_of(&v2, &r2, 20750, Some(20760)).0, None);
            assert_eq!(hold(&v2), Some((20800, 20760)));
            assert_eq!(floor_of(&v2, &r2, 20802, Some(20760)), (None, "floor_held d=20802 c=20760 since=20800 left=2".to_string()));
            // 11: d 20798, c 20762 -- withheld, left min(6, 2) = 2
            assert_eq!(floor_of(&v2, &r2, 20798, Some(20762)), (None, "floor_held d=20798 c=20762 since=20800 left=2".to_string()));
            // 4: d 20804 -- the floor on 20804 by the device's count
            let (d4, v4, r4) = held("g4");
            assert_eq!(floor_of(&v4, &r4, 20804, None), (Some(20802), "floor_ceiling d=20804 c=20760 since=20800 by=device".to_string()));
            // 5: the clock set right while waiting, d 20761, c 20761 -- the floor on 20761, H let go
            let (d5, v5, r5) = held("g5");
            assert_eq!(floor_of(&v5, &r5, 20761, Some(20761)), (Some(20759), "floor_free d=20761 c=20761".to_string()));
            assert_eq!(hold(&v5), None);
            // 10: the clock set back to 20790 while waiting, the nodes confirm 20764 -- the floor on 20790 by the confirmed day
            let (d10, v10, r10) = held("g10");
            assert_eq!(floor_of(&v10, &r10, 20790, Some(20764)), (Some(20788), "floor_ceiling d=20790 c=20764 since=20800 by=confirmed".to_string()));
            // 7: doors that disagree (20760 and 20790), d 20790 -- c 20790, the floor on 20790, nothing held
            let (d7, v7, r7) = with_pipe("g7", 33);
            let c7 = crate::nodeclock::highest(&[20760 * 86_400 + 1, 20790 * 86_400 + 1]);
            assert_eq!(floor_of(&v7, &r7, 20790, c7), (Some(20788), String::new()));
            assert_eq!(hold(&v7), None);
            // 8: no M ever, no answer, d 20800 -- the floor on 20800, nothing held
            let (d8, v8, r8) = with_pipe("g8", 34);
            assert_eq!(floor_of(&v8, &r8, 20800, None), (Some(20798), String::new()));
            assert_eq!((hold(&v8), m_of(&v8)), (None, None));
            // 9: a book already holding its floor of 20758, d 20800 (c 20760) -- the walk goes on, the guard is not asked
            let (d9, v9, r9) = with_pipe("g9", 35);
            assert_eq!(floor_of(&v9, &r9, 20760, Some(20760)).0, Some(20758));
            assert_eq!(floor_of(&v9, &r9, 20800, Some(20760)), (Some(20798), String::new()));
            assert_eq!(hold(&v9), None);
            for d in [d1, d2, d4, d5, d10, d7, d8, d9] {
                let _ = std::fs::remove_dir_all(&d);
            }
        });
    }

    /// THE SAME CASES WITH THE SWITCH OFF (iOS 2311): every one floors on the device's day; nothing is kept for the guard.
    #[test]
    fn with_the_guard_off_every_case_floors_on_the_devices_day() {
        rules::with(Rules::IOS, || {
            for (i, (d, heard)) in [(20760, Some(20760)), (20800, Some(20760)), (20801, None), (20804, None), (20761, Some(20761)),
                                    (20750, Some(20760)), (20790, Some(20790)), (20800, None), (20790, Some(20764)), (20798, Some(20762))]
                .into_iter().enumerate()
            {
                let (dd, v, r) = with_pipe(&format!("goff{i}"), 40 + i as u8);
                let (f, line) = floor_of(&v, &r, d, heard);
                assert_eq!(f, (d >= ERASE_DAY).then_some(d - FLOOR_LAG), "case {i}: the floor on d, as iOS");
                assert!(line.is_empty() && Book::guard_record(&v).is_none());
                let _ = std::fs::remove_dir_all(&dd);
            }
        });
    }

    /// The vault of a shared case's start: the pipe on the disk as the start names it, and the guard's record beside the book.
    fn shared_start(v: &Vault, start: &str, pipe: &PipeKey) {
        if start == "fresh_empty_book" {
            return;
        }
        let r = pipe.reference().unwrap();
        let form = if start == "floored_20758" { pipe.floored(20760).sealed_b64(20760) } else { pipe.sealed_b64(i64::MIN) };
        assert!(v.set(SECRETS, serde_json::json!({ r: form }).to_string().as_bytes()));
        match start {
            "after_case_2" => assert!(v.set(GUARD_RECORD, br#"{"m":20760,"h":20800,"hc":20760,"done":false}"#)),
            "guard_record_unreadable" => assert!(v.set(GUARD_RECORD, b"{not a record")),
            _ => assert!(["fresh", "fresh_with_pipe", "floored_20758"].contains(&start), "a start the table names: {start}"),
        }
    }
    /// A whole copy of the pipe's folder (its own form and a letter), as a restore by the words lays it back.
    fn whole_copy(pipe: &PipeKey, root: &std::path::Path) -> (crate::history::Archive, Vec<crate::history::Record>) {
        let words = crate::seed::generate().unwrap();
        let a = crate::history::Archive::new(&root.join("copy"), crate::history::keys(&words).unwrap(), [1; 16]);
        let r = pipe.reference().unwrap();
        assert!(a.append(&r, crate::history::HEAD_DIR, 100, &pipe.archive_head()));
        assert!(a.append(&r, 1, 101, "привет".as_bytes()));
        let records = a.blocks().into_iter().map(|b| crate::history::Record { kind: crate::history::KIND_BLOCK, name: String::new(), data: b }).collect();
        (crate::history::Archive::new(&root.join("laid"), crate::history::keys(&words).unwrap(), [2; 16]), records)
    }

    /// THE SHARED TABLE OF #38 (cases 1-18, decision 188 and the rulings of 19:23 and 22:04 UTC; the same file the other ports
    /// read, byte for byte: pipekey/clock-guard-cases.json), on the book with the switches decision 188 sets: each step one read
    /// of the book in a run of its own or the run before, its result, the floor day, the guard's record beside the book (M, H, Hc,
    /// done), the lines it wrote, whether it wrote the disk and whether it asked the guard.
    #[test]
    fn the_shared_clock_guard_cases_hold_on_the_book() {
        let table: Value = serde_json::from_str(include_str!("pipekey/clock-guard-cases.json")).unwrap();
        let k = &table["constants"];
        assert_eq!((k["eraseDay"].as_i64(), k["N"].as_i64(), k["floorLag"].as_i64()),
                   (Some(ERASE_DAY), Some(crate::nodeclock::BOX_DAYS), Some(FLOOR_LAG)));
        let cases = table["cases"].as_array().unwrap();
        assert_eq!(cases.iter().map(|c| c["id"].as_i64().unwrap()).collect::<Vec<_>>(), (1..=18).collect::<Vec<_>>());
        rules::with(Rules::DECIDED, || {
            for case in cases {
                let id = case["id"].as_i64().unwrap();
                let d = dir(&format!("shared-{id}"));
                let v = Vault::at(&d.join("v")).unwrap();
                let secret = s(60 + id as u8);
                let pipe = PipeKey::new(&secret);
                let r = pipe.reference().unwrap();
                let start = case["start"].as_str().unwrap();
                shared_start(&v, start, &pipe);
                let record_before = v.get(GUARD_RECORD);
                let (day, heard) = (Arc::new(AtomicI64::new(0)), Arc::new(Mutex::new(None::<i64>)));
                let mut book: Option<Book> = None;
                for (i, step) in case["steps"].as_array().unwrap().iter().enumerate() {
                    let at = format!("case {id} step {}", i + 1);
                    day.store(step["d"].as_i64().unwrap(), Ordering::SeqCst);
                    *heard.lock().unwrap() = step["heard"].as_array().unwrap().iter().filter_map(|h| h.as_i64()).max();
                    if i == 0 || step["restart"].as_bool() == Some(true) {
                        let (dc, hc) = (day.clone(), heard.clone());
                        book = Some(Book::new_with(v.clone(), Arc::new(move || dc.load(Ordering::SeqCst)), Arc::new(move || *hc.lock().unwrap())));
                    }
                    let b = book.as_ref().unwrap();
                    if step["book_write_fails"].as_bool() == Some(true) {
                        b.refuse_writes.store(true, Ordering::SeqCst);
                    }
                    let before = b.seen.lock().unwrap().clone();
                    let key = match step["event"].as_str() {
                        Some("read_empty_book") => {
                            assert!(b.refs().is_empty(), "{at}: the book holds no pipe");
                            None
                        }
                        Some("first_pipe_arrives") => {
                            assert_eq!(b.establish(&pipe).as_deref(), Some(r.as_str()), "{at}: the first pipe");
                            b.key(&r)
                        }
                        Some("cold_start_restore_of_a_whole_copy") => {
                            let (laid, records) = whole_copy(&pipe, &d);
                            let store = crate::media::Store::at(&d.join("m")).unwrap();
                            crate::history::restore(&laid, b, &store, None, &records);
                            b.key(&r)
                        }
                        None => b.key(&r),
                        Some(other) => panic!("{at}: an event the table does not name: {other}"),
                    };
                    let after = b.seen.lock().unwrap().clone();
                    let e = &step["expect"];
                    let lines: Vec<String> = e["trace"].as_array().unwrap().iter().map(|l| l.as_str().unwrap().to_string()).collect();
                    assert_eq!(after.lines[before.lines.len()..].to_vec(), lines, "{at}: the lines");
                    let rec = Book::guard_record(&v);
                    match (e["result"].as_str().unwrap(), &key) {
                        ("floor" | "walk" | "unchanged", Some(k)) => assert_eq!(k.floor_day(), e["F"].as_i64(), "{at}: the floor day"),
                        ("held" | "none", Some(k)) => assert!(k.holds_origin(), "{at}: no floor"),
                        ("none", None) => {}
                        (other, _) => panic!("{at}: a result the table does not name here: {other}"),
                    }
                    if let Some(h) = e.get("H") {
                        assert_eq!(rec.and_then(|x| x.1).map(|x| x.0), h.as_i64(), "{at}: H");
                    }
                    if let Some(h) = e.get("Hc") {
                        assert_eq!(rec.and_then(|x| x.1).map(|x| x.1), h.as_i64(), "{at}: Hc");
                    }
                    if let Some(m) = e.get("M") {
                        assert_eq!(rec.and_then(|x| x.0), m.as_i64(), "{at}: M");
                    }
                    if let Some(done) = e.get("done") {
                        assert_eq!(rec.map(|x| x.2), done.as_bool(), "{at}: done");
                    }
                    if let Some(kept) = e.get("record") {
                        assert_eq!(kept.as_str(), Some("not overwritten"));
                        assert_eq!(v.get(GUARD_RECORD), record_before, "{at}: the record lies as it lay");
                    }
                    if let Some(restore) = e.get("restore") {
                        assert_eq!(restore.as_str(), Some("the restored key stays whole, as the book"));
                        assert!(key.as_ref().is_some_and(|k| k.holds_origin()), "{at}: the restored key whole");
                    }
                    match (e["memory"].as_str(), &key) {
                        (Some("whole"), Some(k)) => assert!(k.holds_origin(), "{at}: the book in memory whole"),
                        (Some("floored_20758"), Some(k)) => assert_eq!(k.floor_day(), Some(20758), "{at}: the book in memory walked"),
                        (None, _) => {}
                        (Some(other), _) => panic!("{at}: {other}"),
                    }
                    match e["disk"].as_str() {
                        Some("whole") => assert!(disk_holds(&v, &secret), "{at}: the disk whole"),
                        Some("floored_20758_written_before_hand_out") => {
                            assert!(!disk_holds(&v, &secret), "{at}: written before the book was handed out");
                            let on_disk = Book::peek(&v, i64::MIN);
                            assert_eq!(on_disk.iter().map(|k| k.floor_day()).collect::<Vec<_>>(), [Some(20758)], "{at}");
                        }
                        Some(other) => panic!("{at}: {other}"),
                        None => {}
                    }
                    if e["disk_write"].as_bool() == Some(false) {
                        assert_eq!(after.writes, before.writes, "{at}: nothing written");
                    }
                    if e["guard_asked"].as_bool() == Some(false) {
                        assert_eq!(after.asks, before.asks, "{at}: the guard not asked");
                    }
                }
                let _ = std::fs::remove_dir_all(&d);
            }
        });
    }

    /// #38 CASE 13 AND POINT 6 (22:04 UTC), A REFUSED WRITE: the book walks to the floor day on its first read of the day, and when
    /// the disk refuses the write the walked book is handed out all the same; the write is tried again at the first read at least
    /// sixty seconds after the refusal, not before; once written, a read of the same day writes nothing; nothing whole is written
    /// back, and a clock set back moves no floor day back.
    #[test]
    fn a_refused_write_keeps_the_walked_book_and_a_read_a_minute_later_writes_it() {
        use std::sync::atomic::Ordering::SeqCst;
        rules::with(Rules::DECIDED, || {
            let d = dir("refused");
            let v = Vault::at(&d).unwrap();
            let (day, heard) = (Arc::new(AtomicI64::new(20759)), Arc::new(Mutex::new(Some(20759i64))));
            let (dc, hc) = (day.clone(), heard.clone());
            let b = Book::new_with(v.clone(), Arc::new(move || dc.load(SeqCst)), Arc::new(move || *hc.lock().unwrap()));
            let a = PipeKey::new(&s(80));
            let r = b.establish(&a).unwrap();
            assert!(b.key(&r).unwrap().holds_origin() && disk_holds(&v, &s(80)));
            b.refuse_writes.store(true, SeqCst);
            day.store(20760, SeqCst);
            *heard.lock().unwrap() = Some(20760);
            let first = b.seen.lock().unwrap().clone();
            assert_eq!(b.key(&r).unwrap().floor_day(), Some(20758), "handed out walked though the disk refused");
            let refused = b.seen.lock().unwrap().clone();
            assert_eq!((refused.writes, refused.refused), (first.writes, first.refused + 1), "the disk refused it once");
            assert!(disk_holds(&v, &s(80)));
            assert_eq!(b.key(&r).unwrap().floor_day(), Some(20758), "the next read: still walked");
            assert_eq!(b.seen.lock().unwrap().refused, refused.refused, "and not tried again within the minute");
            b.refuse_writes.store(false, SeqCst);
            b.set_retry_wait(std::time::Duration::ZERO); // the minute has passed
            assert_eq!(b.key(&r).unwrap().floor_day(), Some(20758));
            assert!(!disk_holds(&v, &s(80)), "written at the first read after the wait");
            let settled = b.seen.lock().unwrap().writes;
            assert_eq!(settled, refused.writes + 1);
            let other = PipeKey::new(&s(81));
            let ro = b.establish(&other).unwrap();
            assert!(!disk_holds(&v, &s(81)) && b.key(&ro).unwrap().floor_day() == Some(20758), "a pipe born now: the day's form");
            let after = b.seen.lock().unwrap().writes;
            assert_eq!((b.key(&r).unwrap().floor_day(), b.key(&ro).unwrap().floor_day()), (Some(20758), Some(20758)));
            assert_eq!(b.seen.lock().unwrap().writes, after, "a second read of the day writes nothing");
            day.store(20755, SeqCst);
            assert_eq!(b.key(&r).unwrap().floor_day(), Some(20758), "a clock set back moves no floor day back");
            assert_eq!(b.seen.lock().unwrap().writes, after);
            let _ = std::fs::remove_dir_all(&d);
        });
    }

    /// #38 CASE 13: a pipe born while the floor waits stays whole until the floor applies; then, in the same run, the walk takes it
    /// with every other pipe of the book.
    #[test]
    fn a_pipe_born_while_the_floor_waits_follows_it_when_it_applies() {
        use std::sync::atomic::Ordering::SeqCst;
        rules::with(Rules::DECIDED, || {
            let d = dir("born-waiting");
            let v = Vault::at(&d).unwrap();
            let a = PipeKey::new(&s(82));
            shared_start(&v, "after_case_2", &a);
            let ra = a.reference().unwrap();
            let day = Arc::new(AtomicI64::new(20801));
            let dc = day.clone();
            let b = Book::new_with(v.clone(), Arc::new(move || dc.load(SeqCst)), Arc::new(|| None));
            let n = PipeKey::new(&s(83));
            let rn = b.establish(&n).unwrap();
            assert!(b.key(&ra).unwrap().holds_origin() && b.key(&rn).unwrap().holds_origin(), "the floor waits");
            assert!(disk_holds(&v, &s(83)));
            day.store(20804, SeqCst);
            assert_eq!((b.key(&ra).unwrap().floor_day(), b.key(&rn).unwrap().floor_day()), (Some(20802), Some(20802)));
            assert!(!disk_holds(&v, &s(82)) && !disk_holds(&v, &s(83)));
            assert_eq!(hold(&v), None, "H and Hc let go when the floor applies");
            assert_eq!(Book::guard_record(&v).map(|x| x.2), Some(true), "the first floor kept in the guard's record");
            let _ = std::fs::remove_dir_all(&d);
        });
    }

    #[test]
    fn the_active_receive_side_heals_a_pipe_and_keeps_it_after_a_restart() {
        use crate::pipeheal::{Healing, Role};
        use crate::pipekey::{MINUTES_PER_DAY, WALK_ANCHOR_DAY};
        use crate::wire::{open_boxed_walked, with_author_tag, with_heal};
        let dir = dir("heal-receive");
        let vault = Vault::at(&dir).unwrap();
        let answered = WALK_ANCHOR_DAY + 1;
        let today = answered + 1;
        let book = Book::new_on(vault.clone(), Arc::new(move || today));
        let pipe = PipeKey::new(&s(90));
        let r = book.establish(&pipe).unwrap();
        let (pk, sk) = crate::crypto::mlkem_keypair(&crate::crypto::random(64).unwrap()).unwrap();
        let offer = HealPart::Offer(pk.clone());
        let plain = with_heal(&with_author_tag(b"mid:offer\0hello\0", ""), &offer.field());
        assert!(!book.heal_read(&r, &plain, HealWalk::Old, answered * MINUTES_PER_DAY + 8));
        assert!(book.heal_map().unwrap().get(&r).and_then(|side| side.owed.clone()).is_some_and(|owed| owed.pk == pk));

        let mut side = book.heal_map().unwrap().get(&r).cloned().unwrap();
        let answer = side.carry(answered).unwrap();
        let HealPart::Answer { ref ct, .. } = answer else { panic!("answer") };
        let ss = crate::crypto::mlkem_decaps(&sk, ct).unwrap();
        side.sent(&answer, answered, &pipe);
        let mut map = book.heal_map().unwrap();
        map.insert(r.clone(), side);
        assert!(book.store_heals(&map));

        let heal = pipe.heal(answered, &ss, &pk, ct).unwrap();
        let offering = pipe.with_healing(Some(Healing::new(Role::Offering, heal)));
        let window = today * MINUTES_PER_DAY + 9;
        let letter = with_author_tag(b"mid:healed\0under the healed walk\0", "");
        let sealed = crate::wire::seal(&offering.body_key(window), &letter).unwrap();
        let opened = open_boxed_walked(&sealed, &book.key(&r).unwrap(), window * 60).unwrap();
        assert_eq!(opened.walk, HealWalk::Healed);
        assert!(book.heal_read(&r, &opened.plain, opened.walk, opened.window));
        let rebased = book.key(&r).unwrap();
        assert_eq!(rebased.floor_day(), Some(today));
        assert_eq!(rebased.body_key(window), offering.body_key(window));
        assert_eq!(rebased.body_keys(window).len(), 1);

        let restarted = Book::new_on(vault.clone(), Arc::new(move || today));
        let held = restarted.key(&r).unwrap();
        assert_eq!(held.body_key(window), offering.body_key(window));
        let late = crate::wire::seal(&offering.body_key(window + 1), &letter).unwrap();
        assert!(open_boxed_walked(&late, &held, (window + 1) * 60).is_some());
        let side = restarted.heal_map().unwrap().get(&r).cloned().unwrap();
        assert!(side.rebased && side.healing.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_unreadable_pipe_heal_record_is_not_replaced() {
        let dir = dir("heal-unreadable");
        let vault = Vault::at(&dir).unwrap();
        assert!(vault.set(HEALS, b"not json"));
        let before = vault.get(HEALS).unwrap();
        let book = Book::new_on(vault.clone(), Arc::new(|| crate::pipekey::WALK_ANCHOR_DAY + 2));
        let r = book.establish(&PipeKey::new(&s(91))).unwrap();
        assert!(book.key(&r).is_some());
        assert!(!book.heal_read(&r, b"mid\0hello\0", HealWalk::Old, (crate::pipekey::WALK_ANCHOR_DAY + 2) * crate::pipekey::MINUTES_PER_DAY));
        assert_eq!(vault.get(HEALS).unwrap(), before);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
