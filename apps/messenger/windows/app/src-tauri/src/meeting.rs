//! THE ONE MEETING RESOLVER, the scanner's side (iOS MontanaMeeting + MontanaCard.expand/meet; Android Meeting.kt): a link
//! names an invitation -> the card is fetched from the node under it and opened (or it answers «spent») -> one encapsulation
//! to the card's key, the first secret, the pipe born, the chat standing with the card's name -> the first letter the person
//! writes rides the invitation's box carrying the ciphertext, and every letter after it does too, until the other side
//! answers under the pipe. One link, one conversation: a repeated scan returns to the chat it opened.

use crate::book::Book;
use crate::pipekey::PipeKey;
use crate::card::{perm_bid, rdv_bid, rdv_key, Card, SPENT_MAGIC};
use crate::profile::strip_crown;
use crate::vault::Vault;
use crate::wire::{b64, b64url, unb64, unb64url};
use crate::{crypto, doors};
use serde_json::{json, Map, Value};
use std::sync::Mutex;

const FIRSTS: &str = "pipeFirst"; // ref -> {ct, inv, cf}: the introduction still unanswered
const SCANS: &str = "rdvScanned"; // invitation -> ref (iOS scanBook)
const CARDS: &str = "cardMetByRoot"; // card root -> ref (iOS cardBook)
static GATE: Mutex<()> = Mutex::new(());
/// The record of the introductions is changed under its own lock, never while the book is asked: the book keeps the
/// confirmations from inside its own read (Book::with_pipes -> keep_confirms).
static FIRSTS_LOCK: Mutex<()> = Mutex::new(());

/// THE FIRST LETTER'S CONFIRMATION KEPT FROM THE PIPE'S BIRTH (D1, decision 188 and the chairman's ruling on #36; the switch
/// MONTANA_D1_KEEP_CONFIRM, on by default, «0» turns it off -- off is iOS 2311, rules.rs): an introduction still
/// unanswered once its pipe is floored -- from 03.11.2026 UTC, the book walked to the day (#38 case 13) -- carries the
/// confirmation computed from the secret at the pipe's birth, kept in the introduction's own sealed record beside its ciphertext
/// and bound to the pipe's name («cr») and that ciphertext (never the secret), used only while the formula gives its vector
/// (confirm_formula_holds), checked against the secret while the secret is held, and gone with the record when the other side
/// answers or the pipe is dropped. iOS 2311 sends an empty confirmation there (MTPipeKey.firstConfirm). `false` is iOS's
/// behaviour -- and, with the book walked to the day from 03.11, it leaves every first letter from that day unconfirmed.
pub fn keep_birth_confirm() -> bool {
    crate::rules::get().d1_keep_confirm
}

/// The confirmation's formula reproduces its vector (secret 00..1f, ct = 07 x 1088 -> 13Y1tBiP47pjtmWHdae8xg, derived from the
/// 2311 formula apart from this code): a kept confirmation is carried only while it does.
fn confirm_formula_holds() -> bool {
    static OK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *OK.get_or_init(|| {
        let ok = PipeKey::new(&(0..32).collect::<Vec<u8>>()).first_confirm(&[7u8; 1088]) == "13Y1tBiP47pjtmWHdae8xg";
        if !ok {
            crate::trace_line("first_conf FAIL -- the confirmation's vector does not reproduce: none is kept or carried");
        }
        ok
    })
}

#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(tag = "kind", content = "chat")]
pub enum Outcome {
    Opened(String),
    Spent,
    Refused,
    Own,
    /// not a Montana invitation at all (iOS «This link is not a Montana invitation.»)
    Foreign,
}

fn jmap(v: &Vault, name: &str) -> Map<String, Value> {
    match v.get(name).and_then(|b| serde_json::from_slice(&b).ok()) {
        Some(Value::Object(o)) => o,
        _ => Map::new(),
    }
}

fn jsave(v: &Vault, name: &str, o: Map<String, Value>) {
    v.set(name, Value::Object(o).to_string().as_bytes());
}

/// The link as a messenger may have mangled it: percent bytes, glued punctuation, a capitalised scheme (iOS normalizeLink).
pub fn normalize(s: &str) -> String {
    let mut raw = s.trim().to_string();
    if raw.contains('%') {
        raw = percent_decode(&raw);
    }
    while raw.ends_with(|c: char| ".,;:!?)]}>»›\"'’”…".contains(c)) {
        raw.pop();
    }
    if let Some(i) = raw.find("://") {
        raw = raw[..i].to_lowercase() + &raw[i..];
    }
    raw
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(x) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(x);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| s.to_string())
}

/// The 32-byte invitation inside a short link, or None (iOS MontanaCard.invite(inShort:)): /r/, /perp/ and /temp/ of
/// montana.quest and montana.xxx (the Business writes there), and montana://r/.
pub fn invite(link: &str) -> Option<Vec<u8>> {
    let t = normalize(link);
    let mut forms: Vec<String> = Vec::new();
    for h in ["montana.quest", "montana.xxx", "www.montana.xxx"] {
        for k in ["r", "perp", "temp"] {
            forms.push(format!("https://{h}/{k}/"));
        }
    }
    forms.push("montana://r/".into());
    let form = forms.iter().find(|f| t.starts_with(f.as_str()))?;
    let code = t[form.len()..].split(['?', '#']).next()?.trim_end_matches('/');
    unb64url(code).filter(|i| i.len() == 32)
}

/// The card under an invitation, opened; SPENT for a tombstone; None when no door gave it (iOS expand, four tries).
fn expand(inv: &[u8]) -> Option<Vec<u8>> {
    for attempt in 0..4 {
        if let Some(sealed) = doors::get_blob(&rdv_bid(inv)) {
            if let Some(p) = crypto::open(&rdv_key(inv), &sealed) {
                if p == SPENT_MAGIC || (1184..=1184 + 64).contains(&p.len()) {
                    return Some(p);
                }
            }
        }
        if attempt < 3 {
            std::thread::sleep(std::time::Duration::from_millis(900));
        }
    }
    None
}

/// Was the link a permanent one? The mark beside the card says so (iOS isPermanent).
fn is_permanent(inv: &[u8]) -> bool {
    doors::get_blob(&perm_bid(inv)).and_then(|s| crypto::open(&rdv_key(inv), &s)).is_some_and(|p| p == b"mt-rdv-perm")
}

/// Meets what the link hands over; the network is asked, so this runs off the window's thread.
pub fn meet(v: &Vault, book: &Book, link: &str) -> Outcome {
    let Some(inv) = invite(link) else { return Outcome::Foreign };
    let inv64 = b64url(&inv);
    // a repeated scan of the same code returns into the conversation it opened (iOS meet_rescan)
    if let Some(known) = jmap(v, SCANS).get(&inv64).and_then(|r| r.as_str()).filter(|r| book.holds(r)) {
        return Outcome::Opened(known.to_string());
    }
    // the card's own invitations are not a meeting with oneself
    if (Card { vault: v }).outstanding_invites().contains(&inv) {
        return Outcome::Own;
    }
    let Some(payload) = expand(&inv) else { return Outcome::Refused };
    if payload == SPENT_MAGIC {
        return Outcome::Spent;
    }
    let root = payload[..1184].to_vec();
    let name = String::from_utf8_lossy(&payload[1184..]).trim().to_string();
    let root64 = b64url(&root);
    // ONE ROOT, ONE TUNNEL (iOS cardBook): whatever door the same card came by, the same chat
    let _g = GATE.lock().unwrap();
    if let Some(known) = jmap(v, CARDS).get(&root64).and_then(|r| r.as_str()).filter(|r| book.holds(r)).map(str::to_string) {
        let mut s = jmap(v, SCANS);
        s.insert(inv64, Value::String(known.clone()));
        jsave(v, SCANS, s);
        return Outcome::Opened(known);
    }
    // one encapsulation, and the correspondence exists; the ciphertext waits for the first letter (iOS MontanaCard.meet)
    let Some((ct, ss)) = crypto::mlkem_encaps(&root) else { return Outcome::Refused };
    let Some(secret) = crypto::first_secret(&ss, &root, &ct) else { return Outcome::Refused };
    let key = PipeKey::new(&secret);
    let Some(r) = book.establish(&key) else { return Outcome::Refused };
    remember_first(v, &r, &key, &ct, &inv);
    let mut s = jmap(v, SCANS);
    s.insert(inv64, Value::String(r.clone()));
    jsave(v, SCANS, s);
    let mut c = jmap(v, CARDS);
    c.insert(root64, Value::String(r.clone()));
    jsave(v, CARDS, c);
    drop(_g);
    book.open(&r, &strip_crown(&name), is_permanent(&inv));
    Outcome::Opened(r)
}

/// The ciphertext and the invitation the next letter to `r` must carry, while the introduction is unanswered.
pub fn first(v: &Vault, r: &str) -> Option<(Vec<u8>, Vec<u8>)> {
    let o = jmap(v, FIRSTS).get(r)?.clone();
    Some((unb64(o["ct"].as_str()?)?, unb64(o["inv"].as_str()?)?))
}

/// The other side wrote under the pipe: the introduction is over, the ciphertext is spent (iOS firstContactDone); the
/// confirmation kept with it goes too.
pub fn first_done(v: &Vault, r: &str) -> bool {
    let _g = FIRSTS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut f = jmap(v, FIRSTS);
    let ended = f.remove(r).is_some();
    if ended {
        jsave(v, FIRSTS, f);
    }
    ended
}

/// The introduction of a pipe just born: its ciphertext, its invitation and -- under keep_birth_confirm() -- the confirmation the
/// secret gives now, bound to the pipe's own name.
pub fn remember_first(v: &Vault, r: &str, key: &PipeKey, ct: &[u8], inv: &[u8]) {
    let _g = FIRSTS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut rec = json!({ "ct": b64(ct), "inv": b64(inv) });
    let cf = key.first_confirm(ct);
    if keep_birth_confirm() && confirm_formula_holds() && !cf.is_empty() && key.reference().as_deref() == Some(r) {
        rec["cf"] = json!(cf);
        rec["cr"] = json!(r);
    }
    let mut f = jmap(v, FIRSTS);
    f.insert(r.to_string(), rec);
    jsave(v, FIRSTS, f);
}

/// THE CONFIRMATION A FIRST LETTER CARRIES (D1): the secret's own while the pipe holds it -- and the record learns it, should
/// it lack it -- else the one kept from the birth of this pipe for this ciphertext; nothing when neither stands (iOS 2311).
pub fn confirm(v: &Vault, r: &str, key: &PipeKey, ct: &[u8]) -> String {
    let now = key.first_confirm(ct);
    if !keep_birth_confirm() || !confirm_formula_holds() || key.reference().as_deref() != Some(r) {
        return now;
    }
    let _g = FIRSTS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut f = jmap(v, FIRSTS);
    let Some(rec) = f.get_mut(r).filter(|o| o["ct"].as_str().and_then(unb64).as_deref() == Some(ct)) else { return now };
    let bound = rec["cr"].as_str() == Some(r);
    let kept = if bound { rec["cf"].as_str().unwrap_or("").to_string() } else { String::new() };
    if now.is_empty() {
        return kept;
    }
    if kept != now {
        if !kept.is_empty() {
            crate::trace_line(&format!("first_conf mended conv={}", &r[..r.len().min(10)]));
        }
        rec["cf"] = json!(now);
        rec["cr"] = json!(r);
        jsave(v, FIRSTS, f);
    }
    now
}

/// The confirmations of every introduction still waiting, kept while the pipes still hold their secrets (called by the book
/// before it floors them): a record from an earlier build, or one whose birth stopped half-way, learns it here.
pub fn keep_confirms(v: &Vault, keys: &std::collections::BTreeMap<String, PipeKey>) {
    if !keep_birth_confirm() || !confirm_formula_holds() {
        return;
    }
    let _g = FIRSTS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut f = jmap(v, FIRSTS);
    let mut learnt = 0;
    for (r, rec) in f.iter_mut() {
        if rec["cf"].as_str().is_some_and(|c| !c.is_empty()) && rec["cr"].as_str() == Some(r.as_str()) {
            continue;
        }
        let (Some(key), Some(ct)) = (keys.get(r), rec["ct"].as_str().and_then(unb64)) else { continue };
        let cf = key.first_confirm(&ct);
        if !cf.is_empty() && key.reference().as_deref() == Some(r.as_str()) {
            rec["cf"] = json!(cf);
            rec["cr"] = json!(r);
            learnt += 1;
        }
    }
    if learnt > 0 {
        jsave(v, FIRSTS, f);
        crate::trace_line(&format!("first_conf kept n={learnt}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_invitation_is_read_from_every_form_a_link_takes() {
        let inv = vec![9u8; 32];
        let code = b64url(&inv);
        for link in [
            format!("https://montana.quest/perp/{code}"),
            format!("HTTPS://montana.quest/temp/{code}."),
            format!("https://montana.xxx/r/{code}?x=1"),
            format!("https://www.montana.xxx/r/{code}/"),
            format!("montana://r/{code}"),
        ] {
            assert_eq!(invite(&link), Some(inv.clone()), "{link}");
        }
        assert_eq!(invite("https://example.com/perp/AAAA"), None);
        assert_eq!(invite(&format!("https://montana.quest/perp/{}", b64url(&[1u8; 31]))), None);
    }

    use crate::pipekey::ERASE_DAY;

    fn on_day(v: &Vault, d: i64) -> Book {
        Book::new_on(v.clone(), std::sync::Arc::new(move || d))
    }
    fn dir(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("montana-meeting-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    /// THE CONFIRMATION OF A FIRST LETTER, BOTH WAYS (D1, #36, decision 188): the record of a pipe born never holds the
    /// secret; while the pipe holds it the confirmation is its own -- [D] 13Y1tBiP47pjtmWHdae8xg for the secret 00..1f and
    /// ct = 07 x 1088, and the formula apart from the key; after a restart past the erase day the floored pipe confirms nothing
    /// (iOS 2311, the switch off) or -- MONTANA_D1_KEEP_CONFIRM -- with the confirmation kept from its birth, bound to its own
    /// name and ciphertext, gone with the answer.
    #[test]
    fn the_first_letters_confirmation_after_the_floor_is_empty_as_ios_or_kept_from_the_birth() {
        for keep in [false, true] {
            let rules = crate::rules::Rules { d1_keep_confirm: keep, ..crate::rules::Rules::IOS };
            crate::rules::with(rules, || {
                let d = dir(&format!("d1-{keep}"));
                let v = Vault::at(&d).unwrap();
                let secret: Vec<u8> = (0..32).collect();
                let (ct, inv) = (vec![7u8; 1088], vec![1u8; 32]);
                let key = PipeKey::new(&secret);
                let want = "13Y1tBiP47pjtmWHdae8xg";
                let after_floor = if keep { want } else { "" };
                assert_eq!(b64url(&crate::wire::domained("mt-first-conf", &[&secret, &ct])[..16]), want, "the formula, apart from the key");
                // the birth, on 02.11
                let book = on_day(&v, ERASE_DAY - 1);
                let r = book.establish(&key).unwrap();
                remember_first(&v, &r, &key, &ct, &inv);
                let rec = jmap(&v, FIRSTS)[&r].clone();
                assert_eq!(rec["cf"].as_str(), keep.then_some(want));
                assert_eq!(rec["cr"].as_str(), keep.then_some(r.as_str()), "bound to its own pipe's name");
                assert!(confirm_formula_holds());
                assert!(!jmap(&v, FIRSTS).values().any(|o| o.to_string().contains(&b64(&secret))), "the record never holds the secret");
                assert_eq!(confirm(&v, &r, &book.key(&r).unwrap(), &ct), want, "while the secret is held: its own");
                // a restart on 03.11: the pipe is the floor, its own confirmation is empty
                let after = on_day(&v, ERASE_DAY);
                let floored = after.key(&r).unwrap();
                assert!(!floored.holds_origin() && floored.first_confirm(&ct).is_empty());
                assert_eq!(confirm(&v, &r, &floored, &ct), after_floor, "empty (iOS) or the birth's, byte for byte");
                // never another pipe's, never another ciphertext's
                let other = PipeKey::new(&[9u8; 32]).floored(ERASE_DAY);
                let ro = other.reference().unwrap();
                assert_eq!(confirm(&v, &ro, &other, &ct), "", "a pipe without a record confirms nothing after the floor");
                assert_eq!(confirm(&v, &r, &other, &ct), "", "a record is bound to its own pipe's name");
                assert_eq!(confirm(&v, &r, &floored, &[8u8; 1088]), "", "and to its own ciphertext");
                let mut moved = jmap(&v, FIRSTS);
                moved[&r]["cr"] = json!(ro);
                jsave(&v, FIRSTS, moved);
                assert_eq!(confirm(&v, &r, &floored, &ct), "", "a record bound to another name gives nothing");
                let mut back = jmap(&v, FIRSTS);
                back[&r]["cr"] = json!(r);
                jsave(&v, FIRSTS, back);
                assert_eq!(confirm(&v, &r, &floored, &ct), after_floor);
                // the other side answered: the record and its confirmation are gone
                assert!(first_done(&v, &r));
                assert_eq!(confirm(&v, &r, &floored, &ct), "");
                let _ = std::fs::remove_dir_all(&d);
            });
        }
    }

    /// A RECORD FROM AN EARLIER BUILD, OR A BIRTH STOPPED HALF-WAY (no confirmation in it), with MONTANA_D1_KEEP_CONFIRM: the book's
    /// first read keeps it while the disk still holds the secret -- even on the first start on 03.11 -- and a wrong one is mended
    /// while the secret is held; the switch off (iOS), nothing is kept and the floored pipe confirms nothing.
    #[test]
    fn a_record_without_its_confirmation_learns_it_before_the_floor() {
        for keep in [false, true] {
            let rules = crate::rules::Rules { d1_keep_confirm: keep, ..crate::rules::Rules::IOS };
            crate::rules::with(rules, || {
                let d = dir(&format!("d1-crash-{keep}"));
                let v = Vault::at(&d).unwrap();
                let secret: Vec<u8> = (0..32).collect();
                let ct = vec![7u8; 1088];
                let key = PipeKey::new(&secret);
                let r = on_day(&v, ERASE_DAY - 1).establish(&key).unwrap();
                let mut f = Map::new();
                f.insert(r.clone(), json!({ "ct": b64(&ct), "inv": b64(&[1u8; 32]) }));
                jsave(&v, FIRSTS, f);
                // the first start of this build is already on 03.11: the record learns the confirmation before the floor is written
                let book = on_day(&v, ERASE_DAY);
                let floored = book.key(&r).unwrap();
                assert!(!floored.holds_origin());
                assert_eq!(confirm(&v, &r, &floored, &ct), if keep { "13Y1tBiP47pjtmWHdae8xg" } else { "" });
                assert_eq!(jmap(&v, FIRSTS)[&r]["cf"].as_str().is_some(), keep);
                if keep {
                    // a wrong confirmation in a record, while the secret is held: the secret's own is sent and the record mended
                    let v2 = Vault::at(&d.join("b")).unwrap();
                    let r2 = on_day(&v2, ERASE_DAY - 1).establish(&key).unwrap();
                    let mut f = Map::new();
                    f.insert(r2.clone(), json!({ "ct": b64(&ct), "inv": b64(&[1u8; 32]), "cf": "AAAAAAAAAAAAAAAAAAAAAA" }));
                    jsave(&v2, FIRSTS, f);
                    assert_eq!(confirm(&v2, &r2, &key, &ct), "13Y1tBiP47pjtmWHdae8xg");
                    assert_eq!(jmap(&v2, FIRSTS)[&r2]["cf"].as_str(), Some("13Y1tBiP47pjtmWHdae8xg"));
                    // an introduction begun before D1 (22:04 UTC, point 7): while the pipe holds its origin, the first letter it
                    // resends keeps the confirmation of that letter's ciphertext, once
                    let v3 = Vault::at(&d.join("c")).unwrap();
                    let r3 = on_day(&v3, ERASE_DAY - 1).establish(&key).unwrap();
                    let mut f = Map::new();
                    f.insert(r3.clone(), json!({ "ct": b64(&ct), "inv": b64(&[1u8; 32]) }));
                    jsave(&v3, FIRSTS, f);
                    assert_eq!(confirm(&v3, &r3, &key, &ct), "13Y1tBiP47pjtmWHdae8xg", "the resend carries its own");
                    let kept = jmap(&v3, FIRSTS)[&r3].clone();
                    assert_eq!((kept["cf"].as_str(), kept["cr"].as_str()), (Some("13Y1tBiP47pjtmWHdae8xg"), Some(r3.as_str())), "kept");
                    assert_eq!(confirm(&v3, &r3, &key, &ct), "13Y1tBiP47pjtmWHdae8xg");
                    assert_eq!(jmap(&v3, FIRSTS)[&r3], kept, "once");
                    assert_eq!(confirm(&v3, &r3, &key.floored(ERASE_DAY), &ct), "13Y1tBiP47pjtmWHdae8xg", "after the floor: the kept one");
                }
                let _ = std::fs::remove_dir_all(&d);
            });
        }
    }
}
