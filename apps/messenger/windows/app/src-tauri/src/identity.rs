//! The doors of the onboarding (iOS MontanaOnboardingView, MontanaSettings.swift 2380-2960; ContentView 438-468), as Tauri
//! commands. The words cross into the window only where the person must read or type them -- the words page right after
//! the birth, the restore field -- and are never kept there; "Copy" copies from this side.

use crate::clipboard;
use crate::profile::{callsign_of, kept_name, Profile, TERMS_VERSION};
use crate::seed::{self, Refused, Seed};
use serde::Serialize;
use std::sync::Mutex;

pub struct Person {
    pub seed: Seed,
    pub profile: Profile,
    /// the language of the person's system, for the call sign
    pub lang: String,
    lock: Mutex<()>,
}

impl Person {
    pub fn new(seed: Seed, profile: Profile, lang: String) -> Person {
        Person { seed, profile, lang, lock: Mutex::new(()) }
    }
}

#[derive(Serialize)]
pub struct State {
    /// a person sits here (iOS MontanaSeed.hasSeed)
    pub seated: bool,
    /// the name shown: the person's, or the call sign
    pub name: String,
    pub callsign: String,
    pub terms_ok: bool,
    /// my face's version (0: none): the window draws it when it changes
    pub face: i64,
}

#[derive(Serialize, Debug)]
#[serde(tag = "kind", content = "count")]
pub enum Failure {
    /// "Exactly 24 words required (entered N)"
    WordCount(usize),
    /// "Invalid phrase. Check the words and their order."
    Invalid,
    /// "The identity could not be stored on this device"
    Storage,
    /// The folder of this seat already holds another person's sealed store (or one nobody can name): nothing was
    /// written, nothing was deleted. The person is asked to use another seat, not to overwrite this one.
    SeatHeld,
    /// the core refused for a reason of its own
    Core(i32),
}

pub fn state(p: &Person) -> State {
    let s = p.profile.read();
    let callsign = if p.seed.has_seed() { p.seed.twin().map(|t| callsign_of(&t, &p.lang)).unwrap_or_default() } else { String::new() };
    State {
        seated: p.seed.has_seed(),
        name: if s.user_name.is_empty() { callsign.clone() } else { s.user_name },
        callsign,
        terms_ok: s.terms_accepted_version >= TERMS_VERSION,
        face: 0,
    }
}

/// "Create": the core draws the words, they are kept, read back, and handed to the words page once.
/// THE SEAT TAKES THE WORDS FIRST : a folder that holds another person's store refuses before anything is
/// written, so a refused create does not even forget the profile that stood there.
pub fn create(p: &Person) -> Result<Vec<String>, Failure> {
    let _g = p.lock.lock().unwrap();
    let words = seed::generate().map_err(|e| match e { Refused::Core(c) => Failure::Core(c), _ => Failure::Invalid })?;
    p.seed.enter(&words).map_err(|e| match e {
        Refused::SeatHeld => Failure::SeatHeld,
        Refused::Core(c) => Failure::Core(c),
        _ => Failure::Storage,
    })?;
    p.profile.forget();
    let _ = p.seed.twin();
    Ok(words.split(' ').map(str::to_string).collect())
}

/// "Restore": lower case, split on spaces, new lines and commas; exactly 24 words; the core opens keys from them or
/// they are refused (iOS MontanaSettings 2589-2640).
pub fn restore(p: &Person, typed: &str) -> Result<(), Failure> {
    let _g = p.lock.lock().unwrap();
    let words: Vec<String> = typed
        .to_lowercase()
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect();
    if words.len() != 24 {
        return Err(Failure::WordCount(words.len()));
    }
    match p.seed.enter(&words.join(" ")) {
        Ok(()) => {
            let _ = p.seed.twin();
            Ok(())
        }
        Err(Refused::Storage) => Err(Failure::Storage),
        Err(Refused::SeatHeld) => Err(Failure::SeatHeld),
        Err(_) => Err(Failure::Invalid),
    }
}

/// "Copy" on the words page: the stored words onto the clipboard, out of its history and its cloud.
pub fn copy_words(p: &Person) -> bool {
    p.seed.mnemonic().map(|m| clipboard::copy_secret(&m)).unwrap_or(false)
}

/// The name of the profile step: no crown, at most 64; empty is the call sign (iOS ProfileView onboarding Done).
pub fn set_name(p: &Person, typed: &str) -> bool {
    let mut name = kept_name(typed);
    if name.is_empty() {
        name = p.seed.twin().map(|t| callsign_of(&t, &p.lang)).unwrap_or_default();
    }
    p.profile.update(|s| s.user_name = name)
}

pub fn accept_terms(p: &Person) -> bool {
    p.profile.update(|s| s.terms_accepted_version = TERMS_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::Book;
    use crate::pipekey::PipeKey;
    use crate::vault::Vault;

    fn person(name: &str) -> (Person, std::path::PathBuf) {
        let d = std::env::temp_dir().join(format!("montana-person-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        (Person::new(Seed::new(Vault::at(&d.join("vault")).unwrap()), Profile::at(&d), "ru".into()), d)
    }

    #[test]
    fn create_then_name_then_terms_is_the_whole_way_in() {
        let (p, d) = person("way");
        assert!(!state(&p).seated);
        let words = create(&p).unwrap();
        assert_eq!(words.len(), 24);
        let st = state(&p);
        assert!(st.seated && !st.terms_ok && !st.callsign.is_empty() && st.name == st.callsign);
        assert!(set_name(&p, "👑 Алиса"));
        assert_eq!(state(&p).name, "Алиса");
        assert!(accept_terms(&p));
        assert!(state(&p).terms_ok);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn restore_counts_the_words_and_refuses_a_wrong_phrase() {
        let (p, d) = person("restore");
        let words = seed::generate().unwrap();
        assert!(matches!(restore(&p, "one two three"), Err(Failure::WordCount(3))));
        let wrong: Vec<&str> = words.split(' ').map(|w| if w == words.split(' ').next().unwrap() { "zzzz" } else { w }).collect();
        assert!(matches!(restore(&p, &wrong.join(" ")), Err(Failure::Invalid)));
        restore(&p, &words.to_uppercase().replace(' ', ",\n")).unwrap();
        assert_eq!(p.seed.mnemonic().as_deref(), Some(words.as_str()));
        let _ = std::fs::remove_dir_all(&d);
    }

    // ── THE STORE OF A SEAT IS NOT INHERITED ─────────────────────────────────────────────────────────────────────────
    // The defect (proved red on the code before this change): the words of A are lost (the store stands),
    // identity_state.seated is false, onboarding offers "create"/"restore", and the words of B land in A's folder --
    // the first read of the book then names A's pipes and feed to B. The rule proved here: a seat that holds a person
    // takes only that person's words back.

    #[test]
    fn seat_boundary_create_over_another_persons_store_is_refused_and_keeps_it() {
        let (p, v, d) = seat_boundary_seat("create-held");
        let (_words_a, r) = seat_boundary_seat_with_a(&p, &v);
        p.profile.update(|s| s.user_name = "Artificial A".into());
        v.delete(seed::MN); // the words are lost; the store stays
        assert!(!state(&p).seated);
        let got = create(&p);
        assert!(matches!(got, Err(Failure::SeatHeld)), "a new person must not take another person's seat: {got:?}");
        assert!(v.get(seed::MN).is_none(), "a refused create writes no words");
        assert!(v.has("pipeSecrets") && v.has("chats") && v.has("mt.twin.ref"), "the store of A is untouched");
        assert!(Book::new(v.clone()).key(&r).is_some(), "A's pipe still stands in its own seat");
        assert_eq!(p.profile.read().user_name, "Artificial A", "a refused create changes nothing of the person");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn seat_boundary_restore_of_other_words_into_a_held_seat_is_refused() {
        let (p, v, d) = seat_boundary_seat("restore-other");
        seat_boundary_seat_with_a(&p, &v);
        v.delete(seed::MN);
        let words_b = seed::generate().unwrap();
        let got = restore(&p, &words_b);
        assert!(matches!(got, Err(Failure::SeatHeld)), "the words of B must not take A's store: {got:?}");
        assert!(!state(&p).seated);
        assert!(v.get(seed::MN).is_none() && v.has("pipeSecrets") && v.has("chats"));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn seat_boundary_restore_of_the_same_words_into_their_own_seat_still_works() {
        let (p, v, d) = seat_boundary_seat("restore-own");
        let (words_a, r) = seat_boundary_seat_with_a(&p, &v);
        v.delete(seed::MN); // the words are lost, the store stands
        restore(&p, &words_a).expect("the same person comes back to their own seat");
        assert!(state(&p).seated);
        assert!(Book::new(v.clone()).key(&r).is_some(), "their store comes back with them");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn seat_boundary_an_unknown_owner_refuses_both_create_and_restore() {
        let (p, v, d) = seat_boundary_seat("unknown-owner");
        let (_words_a, _r) = seat_boundary_seat_with_a(&p, &v);
        v.delete(seed::MN);
        v.delete("mt.twin.ref"); // the words and the name of the owner are gone; the store stands
        let got = create(&p);
        assert!(matches!(got, Err(Failure::SeatHeld)), "whom the store belongs to is unknown: {got:?}");
        let words_b = seed::generate().unwrap();
        let got = restore(&p, &words_b);
        assert!(matches!(got, Err(Failure::SeatHeld)), "a store nobody can name takes nobody's words: {got:?}");
        assert!(v.get(seed::MN).is_none() && v.has("pipeSecrets") && v.has("chats"));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn seat_boundary_create_in_an_empty_seat_still_works() {
        let (p, v, d) = seat_boundary_seat("create-empty");
        let words = create(&p).expect("an empty seat takes a new person");
        assert_eq!(words.len(), 24);
        assert!(state(&p).seated && v.has(seed::MN) && v.has("mt.twin.ref"));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn seat_boundary_valid_words_rebuild_a_missing_twin() {
        let (p, v, d) = seat_boundary_seat("missing-twin");
        let (words_a, r) = seat_boundary_seat_with_a(&p, &v);
        v.delete("mt.twin.ref");
        restore(&p, &words_a).expect("valid words name their owner without a stored twin");
        assert!(Book::new(v.clone()).key(&r).is_some());
        assert_eq!(p.seed.twin().as_deref(), seed::twin_of(&words_a).as_deref());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn seat_boundary_conflicting_mnemonic_and_twin_are_refused_without_a_write() {
        let (p, v, d) = seat_boundary_seat("conflicting-owner");
        let (words_a, _r) = seat_boundary_seat_with_a(&p, &v);
        let words_b = seed::generate().unwrap();
        v.set("mt.twin.ref", seed::twin_of(&words_b).unwrap().as_bytes());
        let old_mnemonic = p.seed.mnemonic().unwrap();
        let old_pipes = v.get("pipeSecrets").unwrap();
        let got = restore(&p, &words_b);
        assert!(matches!(got, Err(Failure::SeatHeld)), "MN and TWIN must not name two different people: {got:?}");
        assert_eq!(p.seed.mnemonic().as_deref(), Some(words_a.as_str()));
        assert_eq!(p.seed.mnemonic().unwrap(), old_mnemonic);
        assert_eq!(v.get("pipeSecrets").unwrap(), old_pipes);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn seat_boundary_a_damaged_mnemonic_still_returns_to_its_twin() {
        let (p, v, d) = seat_boundary_seat("damaged-mnemonic");
        let (words_a, r) = seat_boundary_seat_with_a(&p, &v);
        assert!(v.set(seed::MN, b"damaged"));
        restore(&p, &words_a).expect("a damaged MN falls back to the stored twin");
        assert!(Book::new(v.clone()).key(&r).is_some());
        let _ = std::fs::remove_dir_all(&d);
    }

    fn seat_boundary_seat(name: &str) -> (Person, Vault, std::path::PathBuf) {
        let d = std::env::temp_dir().join(format!(
            "montana-seat-{name}-{}-{}",
            std::process::id(),
            crate::wire::hex(&crate::crypto::random(4).unwrap())
        ));
        let _ = std::fs::remove_dir_all(&d);
        let v = Vault::at(&d.join("vault")).unwrap();
        (Person::new(Seed::new(v.clone()), Profile::at(&d), "ru".into()), v, d)
    }

    /// A sits in the seat: their words, one pipe of a correspondence and the chat row.
    fn seat_boundary_seat_with_a(p: &Person, v: &Vault) -> (String, String) {
        let words = seed::generate().unwrap();
        p.seed.enter(&words).unwrap();
        let _ = p.seed.twin();
        let book = Book::new_on(v.clone(), std::sync::Arc::new(|| crate::pipekey::WALK_ANCHOR_DAY + 1));
        let r = book.establish(&PipeKey::new(&[0x71u8; 32])).unwrap();
        book.open(&r, "Artificial A", false);
        (words, r)
    }
}
