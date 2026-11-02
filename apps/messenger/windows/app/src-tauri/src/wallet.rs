//! THE COINS ON THE NODES (iOS MTCoinTx's roads, MTCoinSplit; Audit/COIN-TX-V3.md): the book's coins stand on the person's own
//! chain -- its first v3 record a GENESIS (of 0 since the v2 cut), what the book holds above the chain laid as a MINT within the time
//! the key lived, once a window of τ1. Coins to a person are a SEND on my chain committed to their key under the pair's blind; the
//! receiver's RECV takes it on theirs, and a SEND nobody took comes back by a REFUND after a day. Every record is laid only on the
//! head this machine read and verified as its own; the node's word decides, never the phone's clock.

use crate::coins;
use crate::cointx::{self, Hands, Kind, ZERO};
use crate::doors;
use crate::pipekey::PipeKey;
use crate::vault::Vault;
use serde_json::{json, Value};
use std::sync::Mutex;

const STATE: &str = "coin3";

/// A key's chain as its node folds it (POST /tx-head), and the node's own second.
#[derive(Clone, Debug, Default)]
pub struct Chain3 {
    pub head: String,
    pub height: i64,
    pub birth: i64,
    pub mint_end: i64,
    pub proven: i64,
    pub declared: i64,
    pub v3: bool,
    pub now: i64,
    pub minted: Option<i64>,
}

/// My coin key's tag, derived once a run (iOS cachedKeyTag): the about word carries it to every correspondent.
pub static MY_FROM: Mutex<Option<String>> = Mutex::new(None);

/// The seed's coin key of the person seated.
pub fn hands(words: &str) -> Option<Hands> {
    let h = Hands::from_master(&crate::seed::master_of(words)?)?;
    *MY_FROM.lock().unwrap() = Some(h.from());
    *HELD.lock().unwrap() = Some(std::sync::Arc::new(h.clone()));
    Some(h)
}
/// The coin key this process derived for its person (one person a process): the roads that pay away from the walk ask it.
static HELD: Mutex<Option<std::sync::Arc<Hands>>> = Mutex::new(None);
pub fn held_hands() -> Option<std::sync::Arc<Hands>> {
    HELD.lock().unwrap().clone()
}

/// AN AUTO-REACTION'S COINS ARE A SEND OF THE CHAIN (iOS 2304 MTCoinSend.autoGive, MTCoinTx.once): the letter's landing only names
/// the pay -- a SEND goes to the network, which a landing never waits on -- and the coin walk pays it; once a letter: the name
/// «auto:» and the letter's wire name is taken before the SEND goes (two days are kept, a letter is paid within one), and a SEND
/// that never went gives the name back.
static AUTO: Mutex<Vec<(String, String, i64)>> = Mutex::new(Vec::new());
pub fn auto_queue(vault: &Vault, r: &str, mid: &str, coins_: i64) -> bool {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let name = format!("auto:mid:{mid}");
    let now = crate::wire::now_ms() / 1000;
    let mut s = state(vault);
    let mut once: serde_json::Map<String, Value> = s["once"].as_object().cloned().unwrap_or_default();
    once.retain(|_, at| now - at.as_i64().unwrap_or(0) < 2 * 86_400);
    if once.contains_key(&name) || coins::holds(vault, "spend", &name) {
        return false;
    }
    once.insert(name, json!(now));
    s["once"] = Value::Object(once);
    save(vault, &s);
    AUTO.lock().unwrap().push((r.to_string(), mid.to_string(), coins_));
    true
}
pub fn auto_take() -> Vec<(String, String, i64)> {
    std::mem::take(&mut *AUTO.lock().unwrap())
}
pub fn auto_release(vault: &Vault, mid: &str) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = state(vault);
    if s["once"].as_object_mut().and_then(|m| m.remove(&format!("auto:mid:{mid}"))).is_some() {
        save(vault, &s);
    }
}
/// Whether this correspondent was told my coin key (the about word carries it until then).
pub fn told_key(vault: &Vault, conv: &str, k: &str) -> bool {
    state(vault)["told"][conv].as_str() == Some(k)
}
pub fn note_told_key(vault: &Vault, conv: &str, k: &str) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = state(vault);
    s["told"][conv] = json!(k);
    save(vault, &s);
}
/// Their send found no key of mine (iOS forgetToldKey): the mark goes, and the next about word carries the key again.
pub fn forget_told_key(vault: &Vault, conv: &str) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = state(vault);
    if s["told"].as_object_mut().and_then(|m| m.remove(conv)).is_some() {
        save(vault, &s);
    }
}

type Split = (i64, i64, i64, i64, f64, i64);
/// The chain's last word this run (iOS MTCoinSplit): proven, declared, height, the node's second it was told at, this computer's
/// moment then (iOS 2305 Split.second, at) and the end of the key's last MINT (iOS 2314 Split.mintEnd).
static SPLIT: Mutex<Option<Split>> = Mutex::new(None);
/// THE NODE SECOND THE PERSON CAME (iOS 2314 MTCoinTx.hereFrom, the author's words 26.10.2026 09:2x UTC, links 8615 and 8617: «show how
/// the coins are minted every second ... so that it works right over the network too»): the first second of this stay the chain's
/// node told, none while the person is away. A MINT counts from it and never from before it -- minting happens while the person is
/// here (fold-7) -- and the wallet shows the same seconds rising while they are on their way (minting).
static HERE_FROM: Mutex<Option<i64>> = Mutex::new(None);
/// THE STAY (iOS MTCoinTx.beHere's presence): the walk's tick it began at, none while the person is away.
static STAY: Mutex<Option<u64>> = Mutex::new(None);
/// Seconds of presence not confirmed by the core yet (iOS 2325 MTPresenceWait): kept in the person's folder, never spendable.
const PRESENCE_WAIT: &str = "presenceWait";
static PRESENCE_FROM: Mutex<Option<i64>> = Mutex::new(None);
fn local_s() -> f64 {
    crate::wire::now_ms() as f64 / 1000.0
}
static MINTED_AT: Mutex<i64> = Mutex::new(0);

/// A chain's node asked through any door of it (iOS chainDoor: «through the api door when the phone knows it, else through any
/// door of that node the phone holds»): a door out of reach gives the ask to the next door of the same machine -- the same node,
/// whose word alone a record is believed by.
pub fn ask(door: &str, path: &str, body: &Value) -> (i32, Option<Value>) {
    let mut last = (-1, None);
    for d in doors::of_machine(door) {
        last = doors::post_at(&d, path, body, 15_000);
        if last.0 != -1 {
            break;
        }
    }
    last
}

fn num(o: &Value, k: &str) -> i64 {
    o[k].as_str().and_then(|s| s.parse().ok()).or_else(|| o[k].as_i64()).unwrap_or(0)
}

pub fn chain3(from: &str, door: &str) -> Option<Chain3> {
    let (code, o) = ask(door, "/tx-head", &json!({ "from": from }));
    let o = o.filter(|_| code == 200)?;
    let head = o["head"].as_str().filter(|h| h.len() == 64)?.to_string();
    let now = num(&o, "now");
    Some(Chain3 {
        head, height: num(&o, "height"), birth: num(&o, "birth"), mint_end: num(&o, "mint_end"), proven: num(&o, "proven"),
        declared: num(&o, "declared"), v3: num(&o, "v3") == 1, now: if now == 0 { crate::wire::now_ms() / 1000 } else { now },
        minted: (!o["minted"].is_null()).then(|| num(&o, "minted")),
    })
}

/// The head is believed only as my own signed record (iOS headIsMine): its bytes are its name, the core verifies it, it is my key's.
fn head_is_mine(head: &str, me: &str, door: &str) -> bool {
    if head == ZERO {
        return true;
    }
    let (code, o) = ask(door, "/tx-get", &json!({ "id": head }));
    let Some(b) = o.filter(|_| code == 200).and_then(|o| o["rec"].as_str().and_then(crate::wire::unb64)) else { return false };
    cointx::read(&b).is_some_and(|r| r.id == head && r.from == me && (r.version == 2 || r.version == 3))
}

/// One record of my key laid on my chain's head, the head read back and verified as mine first; a head that moved is asked again
/// once (iOS layOnHead): the node's code, its word, the record laid.
pub fn lay_on_head(h: &Hands, make: &dyn Fn(&Chain3) -> Option<Vec<u8>>) -> (i32, String, Option<Vec<u8>>) {
    let me = h.from();
    let Some(door) = cointx::chain_door(&me) else { return (-1, "no door".into(), None) };
    for _ in 0..2 {
        let Some(st) = chain3(&me, &door) else { return (-1, "no head".into(), None) };
        if !head_is_mine(&st.head, &me, &door) {
            return (-1, "head unverified".into(), None);
        }
        let Some(rec) = make(&st) else { return (-1, "no record".into(), None) };
        let (code, o) = ask(&door, "/tx-put", &json!({ "rec": crate::wire::b64(&rec) }));
        let word = o.as_ref().and_then(|o| o["error"].as_str()).unwrap_or("").to_string();
        if code == 200 {
            return (200, String::new(), Some(rec));
        }
        if code == 409 && word == "not on the head" {
            continue;
        }
        return (code, word, Some(rec));
    }
    (409, "not on the head".into(), None)
}

fn nonce() -> Vec<u8> {
    crate::crypto::random(16).unwrap_or_else(|| vec![7; 16])
}
fn birth_ms() -> u64 {
    crate::wire::now_ms().max(0) as u64
}

/// THE CHAIN HOLDS WHAT THE BOOK HOLDS (iOS reconcile, sections 3-4): the key's first v3 record its GENESIS (of 0 since the v2 cut);
/// then what the book holds above the chain laid as a MINT of the time since the last one, at most what the key's time allows --
/// once a window of τ1, or at once when a send needs it. The interval is the node's clock.
pub fn reconcile(vault: &Vault, h: &Hands, force: bool) -> Option<Chain3> {
    let me = h.from();
    let door = cointx::chain_door(&me)?;
    let mut st = chain3(&me, &door)?;
    let book = coins::balance(vault);
    if !st.v3 {
        let amount = book.max(0).min(cointx::genesis_room(st.height, st.birth, st.now));
        let (code, word, _) = lay_on_head(h, &|s| cointx::build3(h, Kind::Genesis, amount as u64, birth_ms(), &s.head, &nonce(), &[0; 32], &[0; 32], ZERO, 0, 0));
        mark(&format!("genesis code={code} word={word} amount={amount}"));
        if code != 200 {
            return None;
        }
        st = chain3(&me, &door)?;
    }
    // ONLY THE CHAIN MINTS (iOS reconcile, section 4, fold-6 and fold-9): from the node's TX_TIME_CUT the chain credits the key's
    // seconds itself, so the phone lays no MINT and shows the fold /tx-head answers; from TX_MINT_STOP it lays no MINT at all
    if cointx::TIME_CUT <= st.now {
        // MINTING IS ITS OWN ROAD (fold-7): the node counts of a MINT only what its interval holds of the last MINT_FRESH seconds
        // before it takes it, so while the person is here the computer lays the seconds since the last MINT, never older than the
        // freshness less the road's margin; the node's minute between two MINTs is the pace
        let stays = here() && staying();   // asked before the coming is held: the stay's beat holds them in the other order
        let from = {
            let mut came = HERE_FROM.lock().unwrap();
            if stays && came.is_none() && cointx::ONLINE_CUT <= st.now {
                *came = Some(st.now);
            }
            came.unwrap_or(st.now)
        };
        if !(cointx::ONLINE_CUT <= st.now && st.now < cointx::MINT_STOP && here() && st.mint_end + 60 <= st.now) {
            told(&st);
            return Some(st);
        }
        let (code, word, _) = lay_on_head(h, &|s| {
            if s.height <= 0 || s.now < cointx::ONLINE_CUT || cointx::MINT_STOP <= s.now {
                return None;
            }
            let t1 = s.now;
            let t0 = mint_from(s.mint_end, t1, from);
            // the present seconds alone weigh it (fold-8): the life the node credited up to the online cut is not asked again
            let amount = cointx::present(t0, t1, t1 + PRESENT_MARGIN);
            if amount <= 0 || t1 < t0 + 60 {
                return None;
            }
            cointx::build3(h, Kind::Mint, amount as u64, birth_ms(), &s.head, &nonce(), &[0; 32], &[0; 32], ZERO, t0 as u64, t1 as u64)
        });
        mark(&format!("present mint code={code} word={word}"));
        let after = if code == 200 { chain3(&me, &door).unwrap_or(st) } else { st };
        told(&after);
        return Some(after);
    }
    let wall = crate::wire::now_ms() / 1000;
    {
        let mut at = MINTED_AT.lock().unwrap();
        if !force && wall - *at < cointx::WINDOW {
            told(&st);
            return Some(st);
        }
        *at = wall;
    }
    let gap = book - st.proven - st.declared;
    if gap <= 0 || st.now - 120 < st.mint_end + 60 {
        told(&st);
        return Some(st);
    }
    let (code, word, _) = lay_on_head(h, &|s| {
        let (t0, t1) = (s.mint_end, s.now - 120);
        let room = cointx::allow_time(t1 - s.birth, s.now) - s.minted.unwrap_or_else(|| cointx::allow_time(t0 - s.birth, s.now));
        let amount = (book - s.proven - s.declared).min(room);
        if amount <= 0 || t1 < t0 + 60 {
            return None;
        }
        cointx::build3(h, Kind::Mint, amount as u64, birth_ms(), &s.head, &nonce(), &[0; 32], &[0; 32], ZERO, t0 as u64, t1 as u64)
    });
    mark(&format!("mint code={code} word={word} gap={gap}"));
    let after = if code == 200 { chain3(&me, &door).unwrap_or(st) } else { st };
    told(&after);
    Some(after)
}

fn told(st: &Chain3) {
    if st.v3 {
        *SPLIT.lock().unwrap() = Some((st.proven, st.declared, st.height, st.now, local_s(), st.mint_end));
        // a SEND laid on a head below this height stands in this state, or never will
        FLYING.lock().unwrap().retain(|(_, h, _)| st.height <= *h);
    }
}

/// The person's own chain as this computer's copy of the chains folded it (iOS MTCoinSplit.heard fromCopy): heard before the node's
/// word, never over a higher state; the same state keeps the moment it was first told.
pub fn heard_copy(st: &cointx::St, second: i64) {
    if !st.v3 {
        return;
    }
    let mut split = SPLIT.lock().unwrap();
    if split.is_some_and(|(_, _, h, ..)| st.height < h) {
        return;
    }
    if split.map(|(p, d, h, .., e)| (p, d, h, e)) != Some((st.proven, st.declared, st.height, st.mint_end)) {
        *split = Some((st.proven, st.declared, st.height, second, local_s(), st.mint_end));
    }
    drop(split);
    FLYING.lock().unwrap().retain(|(_, h, _)| st.height <= *h);
    mark(&format!("split proven={} declared={} copy=1", st.proven, st.declared));
}

/// THE PERSON HERE MINTS (iOS MTCoinTx.here, fold-7): online -- the window in front of the person (Windows holds no tunnel of its own).
pub fn here() -> bool {
    crate::person::front()
}
/// The seconds the road to the node may take: a MINT reaching it later than this counts less than it names (iOS presentMargin).
const PRESENT_MARGIN: i64 = 20;
/// Under the node's minute plus one beat, a MINT's interval never passes the freshness less the margin (iOS presentBeat).
pub const PRESENT_BEAT: u64 = 40;

/// THE PERSON STAYS (iOS MTCoinTx.beHere): the person come, the chain is asked at once, then every beat while they stay; the stay ends
/// at the first beat that finds them away, and the second they came goes with it. The answer: whether the chain is asked at this tick.
pub fn stay_due(vault: &Vault, tick: u64) -> bool {
    let now = crate::wire::now_ms() / 1000;
    let mut stay = STAY.lock().unwrap();
    let (next, ask, fresh) = stay_step(*stay, tick, here());
    if ask && here() && cointx::MINT_STOP <= now {
        let mut from = PRESENCE_FROM.lock().unwrap();
        if from.is_none() {
            *from = Some(now);
        } else {
            presence_fold_locked(vault, &mut from, now);
        }
    }
    if fresh {
        *HERE_FROM.lock().unwrap() = None;
        if !here() {
            presence_close(vault);
        }
    }
    *stay = next;
    ask
}
/// Whether a stay goes on (iOS presence != nil).
pub fn staying() -> bool {
    STAY.lock().unwrap().is_some()
}
/// One tick of the stay: (the stay after it, whether the chain is asked now, whether the second the person came is forgotten).
fn stay_step(stay: Option<u64>, tick: u64, here: bool) -> (Option<u64>, bool, bool) {
    match stay {
        None if here => (Some(tick), true, true),
        None => (None, false, false),
        Some(began) if (tick - began).is_multiple_of(PRESENT_BEAT) =>if here { (stay, true, false) } else { (None, false, true) },
        Some(_) => (stay, false, false),
    }
}
fn presence_fold_locked(vault: &Vault, from: &mut Option<i64>, now: i64) {
    let Some(start) = *from else { return };
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let elapsed = (now - start).max(0);
    if elapsed > 0 {
        let mut s = state(vault);
        s[PRESENCE_WAIT] = json!(s[PRESENCE_WAIT].as_i64().unwrap_or(0) + elapsed);
        save(vault, &s);
    }
    *from = Some(now);
}
#[cfg(test)]
fn presence_fold_at(vault: &Vault, now: i64) {
    let mut from = PRESENCE_FROM.lock().unwrap();
    presence_fold_locked(vault, &mut from, now);
}
/// The app leaving the screen folds the open stay at once (iOS 2325 MTPresenceWait.close).
pub fn presence_close(vault: &Vault) {
    let mut from = PRESENCE_FROM.lock().unwrap();
    presence_fold_locked(vault, &mut from, crate::wire::now_ms() / 1000);
    *from = None;
}
fn presence_seconds_at(vault: &Vault, now: i64) -> i64 {
    if MY_FROM.lock().unwrap().is_none() {
        return 0;
    }
    let kept = state(vault)[PRESENCE_WAIT].as_i64().unwrap_or(0);
    let open = PRESENCE_FROM.lock().unwrap().map_or(0, |from| (now - from).max(0));
    kept + open
}
fn presence_seconds(vault: &Vault) -> i64 {
    presence_seconds_at(vault, crate::wire::now_ms() / 1000)
}
/// A present MINT's first second (iOS reconcile's t0, 2314): the later of the last MINT's end, the node's freshness less the road's
/// margin, and the person's coming.
fn mint_from(mint_end: i64, t1: i64, from: i64) -> i64 {
    mint_end.max(t1 - cointx::MINT_FRESH + PRESENT_MARGIN).max(from)
}
/// THE SECONDS ON THEIR WAY TO THE CHAIN (iOS 2314 MTCoinSplit.minting, the author's words 26.10.2026 09:2x UTC, links 8615 and 8617:
/// «I do not see how the coins are minted every second on the Time Coins page ... even if not all are available or finalized»):
/// from the online cut the chain credits a person's seconds only by a MINT laid while the person is here, once in a minute or so, so
/// the chain's number stood and leapt. These are the very seconds the next MINT will name, counted at the node's second; when it
/// lands the chain holds them and they leave here, so the sum moves on without a leap. Shown and never spent.
pub fn minting() -> i64 {
    let Some((.., second, at, mint_end)) = *SPLIT.lock().unwrap() else { return 0 };
    let Some(from) = *HERE_FROM.lock().unwrap() else { return 0 };
    if !here() {
        return 0;
    }
    on_way(second + (local_s() - at).max(0.0) as i64, mint_end, from)
}
fn on_way(now: i64, mint_end: i64, from: i64) -> i64 {
    if !(cointx::ONLINE_CUT..cointx::MINT_STOP).contains(&now) {
        return 0;
    }
    (now - mint_from(mint_end, now, from)).max(0)
}
/// The Time Balance (iOS 2325 MTCoinSplit.growing): the confirmed available and seconds of presence awaiting the core, never spent.
pub fn growing(vault: &Vault) -> i64 {
    available(vault) + if SPLIT.lock().unwrap().is_none() { 0 } else { minting() } + presence_seconds(vault)
}

/// THE SENDS ON THEIR WAY TO THE CHAIN (iOS MTCoinSplit.flying, fold-7): a SEND laid on the head of height h stands in every state of a
/// height above h, or never will; until the chain tells a state above h its coins are on their way and the available leaves them out.
/// Held in memory: a new run reads the chain after the send's road has ended.
/// (the SEND's name, the height it was laid on, its coins)
static FLYING: Mutex<Vec<(String, i64, i64)>> = Mutex::new(Vec::new());
fn laid(id: &str, coins_: i64, height: i64) {
    let mut f = FLYING.lock().unwrap();
    f.retain(|(k, ..)| k != id);
    f.push((id.to_string(), height, coins_));
}

/// THE AVAILABLE, ONE NUMBER EVERYWHERE (iOS MTCoinSplit.available): what the person's chain holds for its owner, proven and declared,
/// less the sends on their way to it; before the chain spoke this run, the book. The book no longer caps it (fold-7): it mints nothing
/// after the time cut, so as a ceiling it would freeze the number while the chain mints.
pub fn available(vault: &Vault) -> i64 {
    let Some((p, d, height, second, at, _)) = *SPLIT.lock().unwrap() else { return coins::balance(vault) };
    let away: i64 = FLYING.lock().unwrap().iter().filter(|(_, h, _)| height <= *h).map(|(.., c)| *c).sum();
    // the seconds the chain has credited since it spoke, by its own step (iOS 2305 available's «lived»)
    let lived = cointx::since(second, second + (local_s() - at).max(0.0) as i64);
    (p + d + lived - away).max(0)
}

/// THE BEAT OF THE CHAIN'S MINTING (iOS 2305 MTCoinSplit.beat, the author's words 23.10.2026 23:2x MSK: «in the client the minting
/// shows just as before, only it all happens at the core's level»): every second the available is read again, and what the chain's
/// own step raised it by rises on the screen as a coin -- never a coin the chain did not fold. The seconds on their way rise by the
/// same beat (iOS 2314: growing), and when they leave -- the person away, or the MINT landed -- the figure is told without a rise.
static SHOWN: Mutex<Option<i64>> = Mutex::new(None);
pub fn beat(vault: &Vault) {
    if SPLIT.lock().unwrap().is_none() {
        return;
    }
    let now = growing(vault);
    let before = SHOWN.lock().unwrap().replace(now);
    if let Some(b) = before.filter(|b| *b < now) {
        coins::flash_chain(vault, now - b);
    }
}
pub fn split() -> Option<(i64, i64, i64)> {
    SPLIT.lock().unwrap().map(|(p, d, h, ..)| (p, d, h))
}

/// AN UNRECORDED CALL BURNS ON THE CHAIN (iOS MTCoinTx.owe, fold-7): from the online cut, the first second the node takes a BURN, the
/// seconds a call burns are owed in the person's folder and laid as one BURN on my chain's head (burn_owed).
pub fn owe_burn(vault: &Vault, coins_: i64) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if coins_ <= 0 || crate::wire::now_ms() / 1000 < cointx::ONLINE_CUT {
        return;
    }
    let mut s = state(vault);
    s["burn"]["owed"] = json!(s["burn"]["owed"].as_i64().unwrap_or(0).saturating_add(coins_));
    save(vault, &s);
    BURN_DUE.lock().unwrap().get_or_insert(crate::wire::now_ms() + cointx::WINDOW * 1000);
}
/// When the owed goes (iOS owe → burnDue, burnNow): a window after the first second owed while a call talks, at once at its end.
static BURN_DUE: Mutex<Option<i64>> = Mutex::new(None);
/// The call's last seconds go to the chain at its end, not a window later (iOS MTCoinTx.burnNow).
pub fn burn_now(vault: &Vault) {
    let s = state(vault);
    if s["burn"]["owed"].as_i64().unwrap_or(0) > 0 || s["burn"]["rec"].is_string() {
        *BURN_DUE.lock().unwrap() = Some(0);
    }
}
/// Whether the owed is due now; asked, it is taken.
pub fn burn_due() -> bool {
    let mut due = BURN_DUE.lock().unwrap();
    if due.is_some_and(|t| t <= crate::wire::now_ms()) {
        *due = None;
        return true;
    }
    false
}

/// THE OWED LAID AS ONE BURN (iOS burnOwed): a BURN an ended run or a lost answer left goes first to its node's word -- the owed it
/// covered falls only if the node took it, so no second burns twice; then what is owed, as far as the chain holds it, as the book's
/// burn stops at its balance. The record is kept before it goes and waits for its node's word when the answer is lost.
pub fn burn_owed(vault: &Vault, h: &Hands) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let me = h.from();
    let Some(door) = cointx::chain_door(&me) else { return };
    let mut s = state(vault);
    if let Some(rec) = s["burn"]["rec"].as_str().and_then(crate::wire::unb64) {
        let mut left = ask(&door, "/tx-get", &json!({ "id": cointx::name(&rec) })).0 == 200;
        if !left {
            let (code, _) = ask(&door, "/tx-put", &json!({ "rec": crate::wire::b64(&rec) }));
            if code == -1 {
                return; // no word yet: the record waits
            }
            left = code == 200;
        }
        let (owed, covers) = (s["burn"]["owed"].as_i64().unwrap_or(0), s["burn"]["covers"].as_i64().unwrap_or(0));
        s["burn"] = json!({ "owed": if left { (owed - covers).max(0) } else { owed } });
        save(vault, &s);
        mark(&format!("burn settled left={left} covers={covers}"));
    }
    let owing = s["burn"]["owed"].as_i64().unwrap_or(0);
    if owing <= 0 || crate::wire::now_ms() / 1000 < cointx::ONLINE_CUT {
        return;
    }
    let (code, word, rec) = lay_on_head(h, &|c| {
        if !c.v3 || c.now < cointx::ONLINE_CUT {
            return None;
        }
        let amount = owing.min(c.proven + c.declared);
        if amount <= 0 {
            return None;
        }
        let rec = cointx::build3(h, Kind::Burn, amount as u64, birth_ms(), &c.head, &nonce(), &[0; 32], &[0; 32], ZERO, (amount - c.proven).max(0) as u64, 0)?;
        let mut s = state(vault);
        s["burn"] = json!({ "owed": s["burn"]["owed"].as_i64().unwrap_or(owing), "covers": owing, "rec": crate::wire::b64(&rec) });
        save(vault, &s);
        Some(rec)
    });
    mark(&format!("burn code={code} word={word} owed={owing}"));
    if rec.is_none() || code == -1 {
        return; // nothing laid, or the answer was lost: the record waits for its node's word
    }
    let mut s = state(vault);
    let owed = s["burn"]["owed"].as_i64().unwrap_or(0);
    s["burn"] = json!({ "owed": if code == 200 { (owed - owing).max(0) } else { owed } });
    save(vault, &s);
}

fn state(vault: &Vault) -> Value {
    vault.get(STATE).and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_else(|| json!({ "k": {}, "told": {}, "open": [] }))
}
fn save(vault: &Vault, v: &Value) {
    vault.set(STATE, v.to_string().as_bytes());
}
/// Every writer of the coin JSON takes this guard across its read-modify-write, so a fold and a peer mark cannot
/// overwrite each other's seconds.
static STATE_LOCK: Mutex<()> = Mutex::new(());

/// A correspondent's coin key, told in their about word («k»): the key a SEND of version 3 commits to.
pub fn peer_from(vault: &Vault, conv: &str) -> Option<String> {
    state(vault)["k"][conv].as_str().map(str::to_string)
}
pub fn note_peer_from(vault: &Vault, conv: &str, from: &str) {
    if from.len() != 64 || !from.chars().all(|c| c.is_ascii_hexdigit()) {
        return;
    }
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = state(vault);
    if s["k"][conv].as_str() == Some(from) {
        return;
    }
    s["k"][conv] = json!(from);
    save(vault, &s);
}

pub enum Sent {
    Taken(String),
    Pending,
    NotApplicable(String),
    Failed,
}

/// SENDS (iOS send3): the receiver's key known, my chain announced and holding the sum; the record names the declared part (proven
/// is spent first), commits to the receiver's key under the pair's blind and is laid on my chain's head. The book's debit stands
/// under the record's name; a record no node took gives it back.
/// `on` -- the letter a paid reaction's SEND stands under (iOS send3 on target): its coins stand under that letter.
pub fn send3(vault: &Vault, h: &Hands, conv: &str, key: &PipeKey, coins_: i64, on: Option<&str>) -> Sent {
    let Some(peer) = peer_from(vault, conv).and_then(|p| cointx::unhex32(&p)) else { return Sent::NotApplicable("no receiver key".into()) };
    let me = h.from();
    let Some(door) = cointx::chain_door(&me) else { return Sent::Failed };
    let Some(first) = reconcile(vault, h, true) else { return Sent::NotApplicable("no chain".into()) };
    if !first.v3 || first.proven + first.declared < coins_ {
        return Sent::NotApplicable("chain short".into());
    }
    let nonce = nonce();
    // the pair's tag is kept for good (iOS MTPipeKept.pair); the blind stands on the key an item laid today stands on (iOS
    // MTPipeKey.blind): the conversation's root past the anchor
    let pair = key.kept().pair;
    let commit = cointx::commitment(&peer, &key.blind(&nonce, crate::wire::day(), cointx::blind));
    for attempt in 1..=2 {
        let Some(st) = chain3(&me, &door) else { return Sent::Failed };
        if !st.v3 || st.proven + st.declared < coins_ {
            return Sent::NotApplicable("chain short".into());
        }
        if !head_is_mine(&st.head, &me, &door) {
            return Sent::Failed;
        }
        let dec = (coins_ - st.proven).max(0);
        let Some(rec) = cointx::build3(h, Kind::Send, coins_ as u64, birth_ms(), &st.head, &nonce, &pair, &commit, ZERO, dec as u64, 0) else { return Sent::Failed };
        let id = cointx::name(&rec);
        // A SEND THE CHAIN HOLDS STANDS IN THE BOOK WHATEVER ITS BALANCE (iOS sendHeld, fold-7): the chain weighed the sum a moment
        // before, and the book, which mints nothing after the time cut, holds less than the chain -- its refusal would keep coins the
        // chain lets go
        if !coins::moved(vault, "retake", coins_, &format!("{}{id}", coins::TX), Some(conv), on) {
            return Sent::Failed;
        }
        let (code, o) = ask(&door, "/tx-put", &json!({ "rec": crate::wire::b64(&rec) }));
        mark(&format!("send code={code} coins={coins_} dec={dec} id={} attempt={attempt}", &id[..12]));
        if code == 200 && o.as_ref().and_then(|o| o["id"].as_str()) == Some(id.as_str()) {
            let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let mut s = state(vault);
            if let Some(a) = s["open"].as_array_mut() { a.push(json!({ "id": id, "conv": conv, "coins": coins_, "at": st.now })); }
            save(vault, &s);
            laid(&id, coins_, st.height);
            return Sent::Taken(id);
        }
        if code == -1 && ask(&door, "/tx-get", &json!({ "id": id })).0 == 200 {
            laid(&id, coins_, st.height);
            return Sent::Taken(id);
        }
        // not taken: the coins come back under the record's name, and a head that moved is tried once more
        coins::moved(vault, "receive", coins_, &format!("{}{id}", coins::TX_BACK), Some(conv), on);
        if code != 409 {
            return if code == -1 { Sent::Pending } else { Sent::Failed };
        }
    }
    Sent::Failed
}

/// RECEIVES (iOS receive3): a SEND of my pair that commits to my key is taken by a RECV on my chain; the book credits it only when
/// my chain holds it (or the SEND's node names me as the one who took it).
pub fn receive3(vault: &Vault, h: &Hands, s: &cointx::Rec, conv: &str, key: &PipeKey) -> bool {
    if s.kind != Kind::Send as u8 {
        return false;
    }
    let me = h.from();
    let Some(mine) = cointx::unhex32(&me) else { return false };
    // the sender's day is its own clock: the root first, then the established secret while it is held (iOS MTPipeKey.blinds)
    let Some(blind) = key.blinds(&s.nonce, cointx::blind).into_iter()
        .find(|b| cointx::commitment(&mine, b).as_slice() == s.to.as_slice()) else { return false };
    let name = format!("{}{}", coins::TX, s.id);
    if coins::holds(vault, "receive", &name) {
        return false;
    }
    let Some(st) = reconcile(vault, h, false) else { return false };
    if !st.v3 {
        return false;
    }
    let (code, word, _) = lay_on_head(h, &|c| cointx::build3(h, Kind::Recv, s.amount, birth_ms(), &c.head, &nonce(), &[0; 32], &blind, &s.id, s.a, 0));
    let mut taken = code == 200;
    if !taken && (word == "settled" || word == "not yours") {
        if let Some(door) = cointx::chain_door(&s.from) {
            let (_, o) = ask(&door, "/tx-settle", &json!({ "send": s.id }));
            taken = o.is_some_and(|o| o["state"].as_str() == Some("recv") && o["by"].as_str() == Some(me.as_str()));
        }
    }
    mark(&format!("recv code={code} word={word} taken={taken}"));
    taken && coins::moved(vault, "receive", s.amount as i64, &name, Some(conv), heard_on(vault, &s.id).as_deref())
}

/// THE LETTER A PAID REACTION'S SEND STANDS UNDER, AS ITS WORD NAMED IT (iOS MTCoinTx.heard): the word moves no coins; it names the
/// SEND, and the scan asked at once credits that SEND under the letter. A word later than the scan leaves the coins right and under
/// no letter. The newest 256 names are kept.
pub fn heard(vault: &Vault, tx: &str, sid: &str) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if tx.len() != 64 || !tx.chars().all(|c| c.is_ascii_hexdigit()) || sid.is_empty() {
        return;
    }
    let mut s = state(vault);
    let mut rows: Vec<Value> = s["heard"].as_array().cloned().unwrap_or_default().into_iter().filter(|r| r[0].as_str() != Some(tx)).collect();
    rows.push(json!([tx, sid]));
    let from = rows.len().saturating_sub(256);
    s["heard"] = Value::Array(rows.split_off(from));
    save(vault, &s);
    if !coins::holds(vault, "receive", &format!("{}{tx}", coins::TX)) {
        SCAN_SOON.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
fn heard_on(vault: &Vault, tx: &str) -> Option<String> {
    state(vault)["heard"].as_array()?.iter().rev().find(|r| r[0].as_str() == Some(tx)).and_then(|r| r[1].as_str().map(str::to_string))
}
/// A paid reaction's word came: the pairs' scan is asked now, not at its next round (iOS scanFresh «reaction»).
static SCAN_SOON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn scan_due() -> bool {
    SCAN_SOON.swap(false, std::sync::atomic::Ordering::SeqCst)
}

/// A TRANSFER TO ONE'S OWN OTHER ACCOUNT IS ONE PAIR, SENT AND RECEIVED IN THE SAME STEP (iOS MTCoinTx.landOwn; the author's words
/// 12.10.2026 13:2x MSK «either the coins left or not -- there is no third»; 15:3x, No.66 «one pair of records, sent and received»):
/// the computer holds both seeds, so the RECV of my SEND is laid at once on the other key's own chain with that key -- the chain
/// opened first by its GENESIS of the book its folder holds, as its own device would open it. A SEND whose RECV did not stand stays
/// the receiver's to take when seated, or comes back by the REFUND after a day -- never both. Whether it was taken, and the word.
pub fn land_own(their: &Hands, their_book: i64, send: &str, key: &PipeKey) -> (bool, String) {
    let me = match MY_FROM.lock().unwrap().clone() { Some(m) => m, None => return (false, "no key of mine".into()) };
    let Some(door) = cointx::chain_door(&me) else { return (false, "no door".into()) };
    let (code, o) = ask(&door, "/tx-get", &json!({ "id": send }));
    let Some(s) = o.filter(|_| code == 200).and_then(|o| o["rec"].as_str().and_then(crate::wire::unb64)).and_then(|b| cointx::read(&b)) else {
        return (false, format!("no record code={code}"));
    };
    if s.id != send || s.version != 3 || s.kind != Kind::Send as u8 {
        return (false, "not a send of v3".into());
    }
    let them = their.from();
    let (Some(theirs), Some(tdoor)) = (cointx::unhex32(&them), cointx::chain_door(&them)) else { return (false, "no door of theirs".into()) };
    let Some(blind) = key.blinds(&s.nonce, cointx::blind).into_iter()
        .find(|b| cointx::commitment(&theirs, b).as_slice() == s.to.as_slice()) else { return (false, "not their key".into()) };
    if let Some(st) = chain3(&them, &tdoor).filter(|st| !st.v3) {
        let amount = their_book.max(0).min(cointx::genesis_room(st.height, st.birth, st.now));
        let (code, word, _) = lay_on_head(their, &|c| cointx::build3(their, Kind::Genesis, amount as u64, birth_ms(), &c.head, &nonce(), &[0; 32], &[0; 32], ZERO, 0, 0));
        mark(&format!("own genesis code={code} word={word} amount={amount}"));
    }
    let (code, word, _) = lay_on_head(their, &|c| cointx::build3(their, Kind::Recv, s.amount, birth_ms(), &c.head, &nonce(), &[0; 32], &blind, &s.id, s.a, 0));
    (code == 200, format!("recv code={code} word={word}"))
}

/// THE PAIRS' RECORDS (iOS scan): every pipe's tag asked on both chain nodes; a verified record of another key is a credit (a v3 SEND
/// taken by my RECV), a record of my own key my send from another device of the words. A record is believed only from its key's
/// chain node. The pipes: (conversation, its pipe).
pub fn scan(vault: &Vault, h: &Hands, pipes: &[(String, PipeKey)]) -> usize {
    let me = h.from();
    let by_tag: std::collections::HashMap<String, (String, PipeKey)> =
        pipes.iter().map(|(c, k)| (crate::wire::hex(&k.kept().pair), (c.clone(), k.clone()))).collect();
    if by_tag.is_empty() {
        return 0;
    }
    let tags: Vec<String> = by_tag.keys().cloned().collect();
    let mut credited = 0;
    let mut seen = std::collections::HashSet::new();
    for (node, host) in cointx::CHAIN_NODES {
        let door = format!("https://{host}/pushwake");
        for part in tags.chunks(256) {
            let mut since = 0i64;
            loop {
                let (code, o) = ask(&door, "/tx-pair", &json!({ "tags": part, "since": since }));
                let Some(o) = o.filter(|_| code == 200) else { break };
                let txs = o["txs"].as_array().cloned().unwrap_or_default();
                for x in &txs {
                    let (Some(id), Some(b)) = (x["id"].as_str(), x["rec"].as_str().and_then(crate::wire::unb64)) else { continue };
                    if !seen.insert(id.to_string()) {
                        continue;
                    }
                    let Some(r) = cointx::read(&b).filter(|r| r.id == id) else { continue };
                    let Some((conv, key)) = by_tag.get(&crate::wire::hex(&r.pair)) else { continue };
                    if r.version != 1 && cointx::chain_node(&r.from) != Some(node) {
                        continue; // a record laid on another node than its key's moves nothing (a v1 record stands on no chain)
                    }
                    let name = format!("{}{}", coins::TX, r.id);
                    if r.from == me {
                        if !coins::holds(vault, "send", &name) {
                            coins::moved(vault, "retake", r.amount as i64, &name, Some(conv), None);
                        }
                    } else if r.version == 3 {
                        if receive3(vault, h, &r, conv, key) {
                            credited += 1;
                        }
                    } else {
                        // A v1 OR v2 RECORD OF ANOTHER KEY IS CREDITED AS IT CAME (iOS scan): the book takes it under its name; a v2 one is
                        // noted with its sender, so the REFUND that takes it back at the sender's node takes it out of this book too
                        if coins::moved(vault, "receive", r.amount as i64, &name, Some(conv), None) {
                            credited += 1;
                        }
                        if r.version == 2 && coins::holds(vault, "receive", &name) {
                            note_credit2(vault, &r.id, &r.from, r.amount as i64, conv);
                        }
                    }
                }
                let last = o["last"].as_i64().unwrap_or(since);
                if txs.len() < 512 || last <= since {
                    break;
                }
                since = last;
            }
        }
    }
    mark(&format!("scan pipes={} credited={credited}", by_tag.len()));
    credited
}

/// The v2 records this book credited, each with its sender's key, its coins and its pair (iOS noteCredit2): a REFUND by the sender
/// takes them back (credits_gone).
fn note_credit2(vault: &Vault, id: &str, from: &str, coins: i64, conv: &str) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = state(vault);
    if !s["credits2"][id].is_null() {
        return;
    }
    if !s["credits2"].is_object() {
        s["credits2"] = json!({});
    }
    s["credits2"][id] = json!({ "f": from, "n": coins, "p": conv });
    save(vault, &s);
}

/// THE COINS ARE EITHER GONE OR NOT (iOS creditsGone; constitution, article 8): a v2 record credited here whose sender's REFUND took
/// it back at its chain node leaves this book by one move under its name, whatever the balance (retake: the truth of a debt). Asked
/// once an hour. Nothing is laid on any chain: the book follows what the chain already says.
pub fn credits_gone(vault: &Vault) {
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let s = state(vault);
    let notes = s["credits2"].as_object().cloned().unwrap_or_default();
    let now = crate::wire::now_ms() / 1000;
    if notes.is_empty() || now - s["credits2_at"].as_i64().unwrap_or(0) < 3600 {
        return;
    }
    let mut left = serde_json::Map::new();
    for (id, c) in notes {
        let (Some(from), Some(coins), Some(conv)) = (c["f"].as_str(), c["n"].as_i64(), c["p"].as_str()) else { continue };
        let Some(door) = cointx::chain_door(from) else { continue };
        let (code, f) = ask(&door, "/tx-settle", &json!({ "send": id }));
        if code != 200 || f.as_ref().and_then(|f| f["state"].as_str()) != Some("refund") {
            left.insert(id, c);
            continue;
        }
        let gone = format!("{}{id}", coins::TX_GONE);
        if !(coins::holds(vault, "send", &gone) || coins::moved(vault, "retake", coins, &gone, Some(conv), None)) {
            left.insert(id.clone(), c);
        }
        mark(&format!("credit gone coins={coins} id={}", id.get(..12).unwrap_or("")));
    }
    let mut s = state(vault);
    s["credits2"] = Value::Object(left);
    s["credits2_at"] = json!(now);
    save(vault, &s);
}

/// An untaken SEND comes back after a day by a REFUND on my chain (iOS refundDue): its node's clock decides, one fate a SEND.
pub fn refund_due(vault: &Vault, h: &Hands) {
    let me = h.from();
    let Some(door) = cointx::chain_door(&me) else { return };
    let s = state(vault);
    let open = s["open"].as_array().cloned().unwrap_or_default();
    if open.is_empty() {
        return;
    }
    let mut resolved: Vec<String> = Vec::new();
    for o in open {
        let (Some(id), Some(conv), Some(c)) = (o["id"].as_str(), o["conv"].as_str(), o["coins"].as_i64()) else { continue };
        let at = o["at"].as_i64().unwrap_or(0);
        let (_, f) = ask(&door, "/tx-settle", &json!({ "send": id }));
        let fate = f.as_ref().and_then(|f| f["state"].as_str()).unwrap_or("").to_string();
        if fate == "recv" {
            resolved.push(id.to_string());
            continue;
        }
        let back = format!("{}{id}", coins::TX_BACK);
        if fate == "refund" {
            coins::moved(vault, "receive", c, &back, Some(conv), None);
            resolved.push(id.to_string());
            continue;
        }
        let due = fate == "open" && chain3(&me, &door).is_some_and(|st| at + 86_460 <= st.now);
        if !due {
            continue;
        }
        let (code, word, _) = lay_on_head(h, &|c3| cointx::build3(h, Kind::Refund, c as u64, birth_ms(), &c3.head, &nonce(), &[0; 32], &[0; 32], id, 0, 0));
        mark(&format!("refund code={code} word={word}"));
        if code == 200 {
            coins::moved(vault, "receive", c, &back, Some(conv), None);
            resolved.push(id.to_string());
        }
    }
    let _w = STATE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut latest = state(vault);
    let kept: Vec<Value> = latest["open"].as_array().cloned().unwrap_or_default()
        .into_iter()
        .filter(|o| o["id"].as_str().is_none_or(|id| !resolved.iter().any(|r| r == id)))
        .collect();
    latest["open"] = Value::Array(kept);
    save(vault, &latest);
}

fn mark(line: &str) {
    crate::trace_line(&format!("coin_v3 {line}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn throwaway() -> (Vault, std::path::PathBuf, Hands) {
        let dir = std::env::temp_dir().join(format!("mt-wallet-{}", crate::board::fresh_id()));
        let h = Hands::from_seed(&crate::crypto::random(32).unwrap()).unwrap();
        (Vault::at(&dir).unwrap(), dir, h)
    }

    /// A throwaway key whose chain node answers now: a key's node is fixed by its first byte (iOS chainNodes), and a node out of
    /// reach this hour is not this test's subject -- the key is drawn again, and the test says which node it stands on.
    fn throwaway_on_a_living_node() -> (Vault, std::path::PathBuf, Hands) {
        let mut silent = vec![];
        for _ in 0..16 {
            let (v, d, h) = throwaway();
            let door = cointx::chain_door(&h.from()).unwrap();
            if silent.contains(&door) {
                continue;
            }
            if ask(&door, "/tx-head", &json!({ "from": h.from() })).0 == 200 {
                eprintln!("a throwaway key on {door}{}", if silent.is_empty() { String::new() } else { format!(" ({} did not answer)", silent.join(", ")) });
                return (v, d, h);
            }
            silent.push(door);
        }
        panic!("no chain node answers: {silent:?}");
    }

    /// THE CALL'S END LAYS ITS BURN AT ONCE (iOS burnNow): nothing owed, nothing is due; owed, the end makes it due now, and once
    #[test]
    fn a_call_end_makes_the_owed_burn_due_at_once() {
        let (v, dir, _) = throwaway();
        burn_now(&v);
        assert!(!burn_due(), "nothing owed, nothing due");
        let mut s = state(&v);
        s["burn"]["owed"] = json!(5);
        save(&v, &s);
        burn_now(&v);
        assert!(burn_due(), "the owed goes at the call's end");
        assert!(!burn_due(), "and only once");
        let _ = std::fs::remove_dir_all(dir);
    }

    /// THE STAY (iOS 2314 beHere): the person come, the chain is asked at once and the coming forgotten for the chain's next word;
    /// then every beat while they stay; the first beat that finds them away ends the stay and forgets the coming again
    #[test]
    fn a_stay_asks_at_once_then_every_beat_and_ends_away() {
        assert_eq!(stay_step(None, 7, false), (None, false, false), "away: nothing asked");
        assert_eq!(stay_step(None, 7, true), (Some(7), true, true), "came: asked at once, the coming anew");
        assert_eq!(stay_step(Some(7), 8, true), (Some(7), false, false));
        assert_eq!(stay_step(Some(7), 7 + PRESENT_BEAT, true), (Some(7), true, false), "a beat later: asked again");
        assert_eq!(stay_step(Some(7), 7 + PRESENT_BEAT + 3, false), (Some(7), false, false), "away between beats: the stay waits");
        assert_eq!(stay_step(Some(7), 7 + 2 * PRESENT_BEAT, false), (None, false, true), "away at the beat: the stay ends");
    }

    /// A PRESENT MINT NEVER COUNTS FROM BEFORE THE PERSON CAME (iOS 2314 reconcile t0): the later of the last MINT's end, the freshness
    /// less the margin, and the coming; the seconds on their way (minting) are the same seconds, nothing before the online cut
    #[test]
    fn a_present_mint_counts_from_the_coming_and_the_page_shows_the_same_seconds() {
        let cut = cointx::ONLINE_CUT;
        let t1 = cut + 10_000;
        // came long ago, the last MINT 81 s back: the MINT names those 81 s, the page shows the same 81 on their way
        assert_eq!(mint_from(t1 - 81, t1, t1 - 5_000), t1 - 81);
        assert_eq!(on_way(t1, t1 - 81, t1 - 5_000), 81);
        // the last MINT long ago: the freshness less the margin bounds it (100 s), as the node counts no more
        assert_eq!(mint_from(t1 - 9_000, t1, t1 - 5_000), t1 - cointx::MINT_FRESH + PRESENT_MARGIN);
        assert_eq!(on_way(t1, t1 - 9_000, t1 - 5_000), cointx::MINT_FRESH - PRESENT_MARGIN);
        // came 30 s ago after an hour away: 30 s, never the seconds before the coming (20:47:11 laid 100 s of an install)
        assert_eq!(mint_from(t1 - 3_600, t1, t1 - 30), t1 - 30);
        assert_eq!(on_way(t1, t1 - 3_600, t1 - 30), 30);
        // before the online cut nothing is on its way
        assert_eq!(on_way(cut - 1, cut - 500, cut - 400), 0);
        // fold-9: no MINT is shown or laid from the stop
        assert_eq!(on_way(cointx::MINT_STOP, cut - 500, cut - 400), 0);
    }

    #[test]
    fn presence_wait_is_kept_but_never_changes_the_available() {
        let (v, dir, h) = throwaway();
        let old_from = MY_FROM.lock().unwrap().clone();
        let old_stay = *PRESENCE_FROM.lock().unwrap();
        let old_split = *SPLIT.lock().unwrap();
        *MY_FROM.lock().unwrap() = Some(h.from());
        *PRESENCE_FROM.lock().unwrap() = Some(cointx::MINT_STOP + 10);
        *SPLIT.lock().unwrap() = None;
        assert_eq!(presence_seconds_at(&v, cointx::MINT_STOP + 15), 5);
        presence_fold_at(&v, cointx::MINT_STOP + 20);
        assert_eq!(presence_seconds_at(&v, cointx::MINT_STOP + 25), 15);
        presence_fold_at(&v, cointx::MINT_STOP + 30);
        assert_eq!(presence_seconds_at(&v, cointx::MINT_STOP + 30), 20);
        let now = crate::wire::now_ms() / 1000;
        *PRESENCE_FROM.lock().unwrap() = Some(now);
        let before = available(&v);
        let total = growing(&v);
        assert!((before + 20..=before + 21).contains(&total), "waiting seconds rise only in the Time Balance: {total}");
        assert_eq!(available(&v), before, "the available alone is spendable");
        *MY_FROM.lock().unwrap() = old_from;
        *PRESENCE_FROM.lock().unwrap() = old_stay;
        *SPLIT.lock().unwrap() = old_split;
        let _ = std::fs::remove_dir_all(dir);
    }

    /// THROUGH THE NODES, TWO THROWAWAY KEYS (test identities only, the owner's word 21.10.2026 19:0x): each chain opens with its
    /// GENESIS of 0; the book mints nothing and the chain holds the seconds A's key lived, never more (fold-6: the node credits them
    /// itself, no MINT is laid); A sends 10 committed to B's key under the pair's blind; B's RECV takes them on B's chain -- both nodes'
    /// folds say so.
    #[test]
    #[ignore]
    fn two_keys_open_their_chains_and_a_send_is_taken_by_its_receiver() {
        if std::env::var("MONTANA_LIVE").is_err() {
            return;
        }
        let (va, da, ha) = throwaway_on_a_living_node();
        let (vb, db, hb) = throwaway_on_a_living_node();
        // ONLY THE CHAIN MINTS (fold-6, from 23.10.2026): the book mints nothing, the node credits the key's life itself
        assert_eq!(coins::in_window(&va, 300, coins::TAP, 300, false), 0, "the book mints nothing after the time cut");
        coins::close_windows(&va, true);
        let a0 = reconcile(&va, &ha, true).expect("A's chain answers");
        // a GENESIS of 0: nothing declared, and nothing proven beyond the seconds the key has lived (the nodes' fold counts them)
        assert!(a0.v3 && a0.declared == 0 && a0.proven <= a0.now - a0.birth + 1, "a new key opens with a GENESIS of 0: {a0:?}");
        let b0 = reconcile(&vb, &hb, true).expect("B's chain answers");
        assert!(b0.v3);
        // fold-9: no MINT goes from this client, and the confirmed state does not move by itself
        let a1 = reconcile(&va, &ha, true).expect("A's chain answers again");
        assert_eq!((a1.height, a1.proven), (a0.height, a0.proven), "no MINT after the stop: {a1:?}");
        let secret = PipeKey::new(&crate::crypto::random(32).unwrap());
        note_peer_from(&va, "B", &hb.from());
        let id = match send3(&va, &ha, "B", &secret, 10, None) {
            Sent::Taken(id) => id,
            _ => panic!("the SEND was not taken"),
        };
        let door = cointx::chain_door(&ha.from()).unwrap();
        let (code, o) = ask(&door, "/tx-get", &json!({ "id": id }));
        assert_eq!(code, 200);
        let rec = cointx::read(&crate::wire::unb64(o.unwrap()["rec"].as_str().unwrap()).unwrap()).unwrap();
        let b_before = chain3(&hb.from(), &cointx::chain_door(&hb.from()).unwrap()).unwrap();
        assert!(receive3(&vb, &hb, &rec, "A", &secret), "B's RECV is taken");
        let b1 = chain3(&hb.from(), &cointx::chain_door(&hb.from()).unwrap()).unwrap();
        let a2 = chain3(&ha.from(), &door).unwrap();
        // the records this machine laid: one SEND on A's chain, one RECV on B's. The nodes' fold of «proven» counts each key's lived
        // seconds by itself since 23.10.2026 (B, whose book never held a coin to mint, reads 192 + 10), so the sums are read as
        // they move: B's by the 10 taken, A's by the 10 sent -- each against the same key's time
        assert_eq!((b1.height, a2.height), (b_before.height + 1, a1.height + 1), "one SEND, one RECV: {a1:?} {a2:?} {b_before:?} {b1:?}");
        assert!(b1.proven >= b_before.proven + 10 && b1.proven <= b_before.proven + 10 + (b1.now - b_before.now) + 2, "B took the 10: {b_before:?} {b1:?}");
        assert!(a2.proven <= a1.proven - 10 + (a2.now - a1.now) + 2, "A's 10 left: {a1:?} {a2:?}");
        assert_eq!(coins::balance(&vb), 10);
        let _ = (std::fs::remove_dir_all(da), std::fs::remove_dir_all(db));
    }

    /// A v2 CREDIT ITS SENDER TOOK BACK LEAVES THE BOOK (iOS creditsGone), read only against the network: a record of the network
    /// whose sender's REFUND stands at its node (1 369 000 coins, refunded 2026-10-20), credited in a throwaway book and noted; the
    /// walk takes it out by one move under «txgone:», the balance back where it was, and a second walk within the hour asks nothing.
    #[test]
    #[ignore]
    fn a_v2_credit_its_sender_took_back_leaves_the_book() {
        if std::env::var("MONTANA_LIVE").is_err() {
            return;
        }
        let id = "dedcaae88f7beb908f3c16705cf9f04d1f6d57620359b77289ece0176a721577";
        let rec = cointx::CHAIN_NODES.iter().find_map(|(_, host)| {
            let (code, o) = crate::doors::post_machine(&format!("https://{host}/pushwake"), "/tx-get", &json!({ "id": id }), 30_000);
            o.filter(|_| code == 200).and_then(|o| o["rec"].as_str().and_then(crate::wire::unb64)).and_then(|b| cointx::read(&b))
        }).expect("a node holds the record");
        assert_eq!((rec.version, rec.kind, rec.amount), (2, Kind::Send as u8, 1_369_000));
        let (v, dir, _) = throwaway();
        let name = format!("{}{id}", coins::TX);
        assert!(coins::moved(&v, "receive", rec.amount as i64, &name, Some("pair"), None));
        note_credit2(&v, id, &rec.from, rec.amount as i64, "pair");
        assert_eq!(coins::balance(&v), 1_369_000);
        credits_gone(&v);
        assert!(coins::holds(&v, "send", &format!("{}{id}", coins::TX_GONE)), "the REFUND took it back");
        assert_eq!(coins::balance(&v), 0);
        assert!(state(&v)["credits2"].as_object().is_some_and(|m| m.is_empty()), "the note is let go");
        credits_gone(&v);
        assert_eq!(coins::balance(&v), 0, "taken out once");
        let _ = std::fs::remove_dir_all(dir);
    }
}
