//! THE ONE OWNER OF A PIPE'S SECRET (iOS 2311 MTPipeKey, byte for byte): the bytes a correspondence stands on live in one field
//! private to this file. Every other file holds a pipe by its name or by this value and asks it for what the pipe derives -- the
//! key of a minute, the label of a day, the tag of a window, the blind and the author field of an item, the record kept for good,
//! the proof of a first letter. No function here hands the bytes out but the forms a store of this machine seals (`form`,
//! `stored`, `sealed_b64`, `archive_head`). The compositions themselves are the wire's (`domained`, `seal_key`, `conv_tag`,
//! `pipe_tag`, `author_tag`, `fingerprint_material`).
//!
//! THE PASSAGE BY THE UTC DAY (iOS 2311, decision 126): up to WALK_ANCHOR_DAY (28.10.2026) every key is the one the earlier
//! builds derive; from the day after it the walk steps once a day and an item (a letter's author field, a group word's tag, a
//! coin SEND's blind) stands on the conversation's root; from TAG_WALK_DAY (01.11) the tag of a window stands on the walked
//! value, the earlier tag still accepted on that day and the next; from ERASE_DAY (03.11) the pipe holds the floor -- the walk's
//! value on F = D - 2 and the quantities kept for good -- and the established secret is erased.

use crate::pipeheal::{Heal, Healing, Walk, ANSWER_SIZE, OFFER_SIZE};
use crate::wire::{author_tag as wire_author_tag, b64, b64url, conv_tag, domained, hex, hmac, pipe_tag, seal_key, sha, unb64, unhex, with_author_tag};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

/// The anchor of the walk (iOS walkAnchorDay): 28.10.2026 UTC.
pub const WALK_ANCHOR_DAY: i64 = 20754;
pub const MAX_WALK: i64 = 36_600;
pub const MINUTES_PER_DAY: i64 = 1440;
/// From this day the tag of a window stands on the walk (iOS tagWalkDay): 01.11.2026 UTC.
pub const TAG_WALK_DAY: i64 = WALK_ANCHOR_DAY + 4;
/// From this day the pipe holds the floor and the established secret is erased (iOS eraseDay): 03.11.2026 UTC.
pub const ERASE_DAY: i64 = TAG_WALK_DAY + 2;
/// F = D - 2: the lower bound of the days a box read listens at (iOS floorLag, MTNodeWire.boxDays).
pub const FLOOR_LAG: i64 = 2;
/// 0x01 ‖ F (8, little-endian) ‖ the walk's value on F ‖ root ‖ rank ‖ fingerprint material ‖ face name ‖ face key ‖ pair tag
/// (32 each) ‖ reference (16): 249 bytes (iOS floorForm, floorFormSize).
pub const FLOOR_FORM: u8 = 0x01;
pub const FLOOR_FORM_SIZE: usize = 1 + 8 + 32 * 7 + 16;

/// What a pipe holds once the walk erased the established secret (iOS MTPipeKey.Floor).
#[derive(Clone, PartialEq, Eq)]
struct Floor {
    day: i64,
    value: [u8; 32],
    root: [u8; 32],
    rank: [u8; 32],
    fingerprint: [u8; 32],
    face_bid: [u8; 32],
    face_key: [u8; 32],
    pair: [u8; 32],
    reference: [u8; 16],
}

/// A pipe as its one owner holds it: the established secret, or the floor the walk left of it.
#[derive(Clone, PartialEq, Eq)]
pub struct PipeKey {
    /// the secret a first letter established, or the key a store of this machine derives for a pipe of its own (a slot it keeps,
    /// the pipe of the seed's devices, an invitation's letter key, a room's lane); empty once the walk erased it
    secret: Vec<u8>,
    floor: Option<Box<Floor>>,
    healing: Option<Healing>,
}

impl Drop for PipeKey {
    fn drop(&mut self) {
        self.secret.fill(0);
    }
}

/// A pipe is shown by its name and its floor's day, never by its bytes.
impl std::fmt::Debug for PipeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.reference().unwrap_or_else(|| "-".into());
        match &self.floor {
            Some(fl) => write!(f, "PipeKey({r}, floor {})", fl.day),
            None => write!(f, "PipeKey({r}, held)"),
        }
    }
}

/// What a correspondence derives from its secret once and for good (iOS MTPipeKept). None walks with the day.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Kept {
    /// SHA-256("mt-conv-root" ‖ 0 ‖ secret)
    pub root: [u8; 32],
    /// the safety number's material and a testimony's addressee (fingerprint_material)
    pub fingerprint: [u8; 32],
    /// SHA-256("mt-pipe-f" ‖ 0 ‖ secret), hex: the name of the pipe face's blob
    pub face_bid: String,
    /// SHA-256("mt-pipe-fk" ‖ 0 ‖ secret): the key of the pipe face
    pub face_key: [u8; 32],
    /// HMAC-SHA-256(secret, "mt-coin-pair-v1"): the coin pair's tag
    pub pair: [u8; 32],
    pub reference: Option<String>,
}

type Walks = HashMap<Vec<u8>, (i64, Vec<u8>)>;
/// the base value -> (the steps walked, the value there), emptied at 256 pipes (iOS walkCache)
static WALK: Mutex<Option<Walks>> = Mutex::new(None);

/// The lowest day a floor has replaced; a walk below it is never cached (an outstanding clone may still compute it).
static WALK_MIN: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

/// The identity left: the walk cache goes with it (its keys are derived from a person's secret).
pub fn forget_walks() {
    if let Ok(mut g) = WALK.lock() {
        if let Some(cache) = g.as_mut() {
            cache.clear();
        }
    }
    WALK_MIN.store(0, std::sync::atomic::Ordering::SeqCst);
}

fn steps(day: i64) -> i64 {
    (if day > WALK_ANCHOR_DAY { day - WALK_ANCHOR_DAY } else { 0 }).min(MAX_WALK)
}

fn arr32(b: &[u8]) -> [u8; 32] {
    let mut a = [0u8; 32];
    a.copy_from_slice(&b[..32]);
    a
}

fn hex16(d: &[u8]) -> String {
    hex(&d[..16])
}

/// Constant time over equal lengths.
fn same_bytes(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0u8, |d, (x, y)| d | (x ^ y)) == 0
}

impl PipeKey {
    pub fn new(secret: &[u8]) -> PipeKey {
        PipeKey { secret: secret.to_vec(), floor: None, healing: None }
    }

    /// A pipe: thirty-two bytes, the size every pipe is born with, or the floor the walk left of them; a record of another size
    /// is no pipe.
    pub fn is_whole(&self) -> bool {
        self.floor.is_some() || self.secret.len() == 32
    }
    /// Whether this machine still holds the established secret: up to ERASE_DAY, and never again once the walk erased it.
    pub fn holds_origin(&self) -> bool {
        self.floor.is_none() && self.secret.len() == 32
    }
    /// The day of the floor held, if the walk erased the secret.
    pub fn floor_day(&self) -> Option<i64> {
        self.floor.as_ref().map(|f| f.day)
    }

    // ── the walk by the client's day ──

    /// The value of the walk on a day: from the established secret, or from the floor forward. A day before the floor gets the
    /// floor's own value -- the walk never goes back, and that value opens nothing laid before it.
    fn walked(&self, day: i64) -> Vec<u8> {
        let (base_steps, base): (i64, &[u8]) = match &self.floor {
            Some(f) => (steps(f.day), &f.value),
            None => (0, &self.secret),
        };
        let target = steps(day);
        if target <= base_steps {
            return base.to_vec();
        }
        let mut g = WALK.lock().unwrap_or_else(|e| e.into_inner());
        let cache = g.get_or_insert_with(HashMap::new);
        let (mut at, mut value) = (base_steps, base.to_vec());
        // below the newest floor the value is computed but never cached: a clone from before the floor must not
        // leave a retired value behind. The check shares the cache guard, so a floor cannot slip between it and the insert.
        if base_steps < WALK_MIN.load(std::sync::atomic::Ordering::SeqCst) {
            while at < target {
                value = domained("mt-pipe-ratchet", &[&value]).to_vec();
                at += 1;
            }
            return value;
        }
        // the cache is keyed by a digest, never by the secret or a floor value itself
        let key = crate::wire::sha(&[base]).to_vec();
        if let Some((s, v)) = cache.get(&key) {
            if *s <= target && *s >= at {
                at = *s;
                value = v.clone();
            }
        }
        while at < target {
            value = domained("mt-pipe-ratchet", &[&value]).to_vec();
            at += 1;
        }
        if cache.len() >= 256 {
            cache.clear();
        }
        cache.insert(key, (target, value.clone()));
        value
    }

    /// The walk's value for the checks of the Canon's walk (wire::canon_walk) -- never in the program.
    #[cfg(test)]
    pub(crate) fn walked_for_check(&self, day: i64) -> Vec<u8> {
        self.walked(day)
    }

    /// The pipe as this machine may hold it on a day: up to ERASE_DAY as it is; from it the floor at F = D - 2, walked forward
    /// from the floor already held, never back. The established secret leaves the value here and every store sealed from it.
    pub fn floored(&self, day: i64) -> PipeKey {
        if day < ERASE_DAY || !self.is_whole() {
            return self.clone();
        }
        let f = day - FLOOR_LAG;
        if self.floor.as_ref().is_some_and(|fl| fl.day >= f) {
            return self.clone();
        }
        self.floor_on(f)
            .map(|fl| PipeKey { secret: vec![], floor: Some(Box::new(fl)), healing: self.healing.clone() })
            .unwrap_or_else(|| self.clone())
    }
    /// The whole floor this pipe gives on the floor day f: the walk's value on f and every quantity kept for good; None for no
    /// pipe, or for a floor already further down than f.
    fn floor_on(&self, f: i64) -> Option<Floor> {
        if !self.is_whole() || self.floor.as_ref().is_some_and(|fl| fl.day > f) {
            return None;
        }
        let k = self.kept();
        let (reference, bid) = (self.ref_bytes()?, unhex(&k.face_bid).filter(|b| b.len() == 32)?);
        let value = arr32(&self.walked(f));
        // the floor replaces the secret: under the cache guard, nothing below it may stay or be cached again
        if let Ok(mut g) = WALK.lock() {
            WALK_MIN.fetch_max(steps(f), std::sync::atomic::Ordering::SeqCst);
            if let Some(cache) = g.as_mut() {
                cache.clear();
            }
        }
        Some(Floor { day: f, value, root: self.root_key(), rank: self.same_rank(), fingerprint: k.fingerprint,
                     face_bid: arr32(&bid), face_key: k.face_key, pair: k.pair, reference })
    }

    fn send_walk(&self, day: i64) -> Vec<u8> {
        if let Some(healing) = &self.healing {
            if healing.sends_healed(day) {
                if let Some(value) = healing.heal.walked(day) {
                    return value;
                }
            }
        }
        self.walked(day)
    }

    fn open_walks(&self, day: i64) -> Vec<(Walk, Vec<u8>)> {
        let Some(healing) = &self.healing else { return vec![(Walk::Old, self.walked(day))] };
        healing.opens(day).into_iter().filter_map(|walk| match walk {
            Walk::Healed => healing.heal.walked(day).map(|value| (walk, value)),
            Walk::Old => Some((walk, self.walked(day))),
        }).collect()
    }

    /// The letter body key of a minute: the seal key under the walk this side sends under on that minute's day.
    pub fn body_key(&self, window: i64) -> [u8; 32] {
        seal_key(&self.send_walk(window.div_euclid(MINUTES_PER_DAY)), window)
    }
    /// Every key a letter of this minute may open under, the healed walk first.
    pub fn body_keys(&self, window: i64) -> Vec<(Walk, [u8; 32])> {
        self.open_walks(window.div_euclid(MINUTES_PER_DAY)).into_iter().map(|(walk, value)| (walk, seal_key(&value, window))).collect()
    }
    /// The conversation's daily label at the node: both sides derive it from the walk this side sends under.
    pub fn conv_w(&self, day: i64) -> String {
        conv_tag(&self.send_walk(day), day)
    }
    /// The labels of a day this side reads a box under, the healed walk first.
    pub fn conv_ws(&self, day: i64) -> Vec<String> {
        self.open_walks(day).into_iter().map(|(_, value)| conv_tag(&value, day)).collect()
    }
    fn own_tag(&self, window: i64) -> Vec<u8> {
        let day = window.div_euclid(MINUTES_PER_DAY);
        if day >= TAG_WALK_DAY || !self.holds_origin() {
            pipe_tag(&self.walked(day), window)
        } else {
            pipe_tag(&self.secret, window)
        }
    }
    /// The tag of a pipe in a window: over the healed walk once this side sends under it, else the pipe's own walk.
    pub fn tag(&self, window: i64) -> Vec<u8> {
        let day = window.div_euclid(MINUTES_PER_DAY);
        if let Some(healing) = &self.healing {
            if healing.sends_healed(day) {
                if let Some(value) = healing.heal.walked(day) {
                    return pipe_tag(&value, window);
                }
            }
        }
        self.own_tag(window)
    }
    /// The tags a receiver accepts in a window: the healed walk first, then the old one while it is held.
    pub fn tags(&self, window: i64) -> Vec<Vec<u8>> {
        let day = window.div_euclid(MINUTES_PER_DAY);
        let walks = self.open_walks(day);
        let mut out: Vec<Vec<u8>> = walks.iter().filter(|(walk, _)| *walk == Walk::Healed)
            .map(|(_, value)| pipe_tag(value, window)).collect();
        if !walks.iter().any(|(walk, _)| *walk == Walk::Old) {
            return out;
        }
        out.push(self.own_tag(window));
        if self.holds_origin() && (TAG_WALK_DAY..ERASE_DAY).contains(&day) {
            out.push(pipe_tag(&self.secret, window));
        }
        out
    }

    // ── the root of a conversation ──

    fn root_of(secret: &[u8]) -> [u8; 32] {
        domained("mt-conv-root", &[secret])
    }
    fn root_key(&self) -> [u8; 32] {
        match &self.floor {
            Some(f) => f.root,
            None => Self::root_of(&self.secret),
        }
    }
    /// The key an item laid on a day stands on: the root past the anchor, the established secret up to it.
    fn item_key(&self, day: i64) -> Vec<u8> {
        if day > WALK_ANCHOR_DAY || !self.holds_origin() { self.root_key().to_vec() } else { self.secret.clone() }
    }
    /// The keys a received item may stand on, the root first: the receiver tries both while it holds the established secret.
    fn item_keys(&self) -> Vec<Vec<u8>> {
        if self.holds_origin() { vec![self.root_key().to_vec(), self.secret.clone()] } else { vec![self.root_key().to_vec()] }
    }

    /// The blind of a coin SEND laid on a day, by the coin's own rule (cointx::blind): applied to the item key here, only the
    /// blind comes out.
    pub fn blind(&self, nonce: &[u8], day: i64, rule: impl Fn(&[u8], &[u8]) -> [u8; 32]) -> [u8; 32] {
        rule(&self.item_key(day), nonce)
    }
    /// The blinds a received SEND may stand on, the root's first.
    pub fn blinds(&self, nonce: &[u8], rule: impl Fn(&[u8], &[u8]) -> [u8; 32]) -> Vec<[u8; 32]> {
        self.item_keys().iter().map(|k| rule(k, nonce)).collect()
    }

    // ── the letter's author is the person ──

    /// The body with the author's field, signed by the person's key over the item key of the day.
    pub fn with_author(&self, body: &[u8], person: Option<&[u8]>, mid: &str, text: &str, day: i64) -> Vec<u8> {
        let Some(key) = person.filter(|k| k.len() == 32) else { return body.to_vec() };
        with_author_tag(body, &wire_author_tag(key, &self.item_key(day), mid, text))
    }
    /// Whether a letter's author field is this person's: the tag made again under this person's key over each item key,
    /// compared in constant time.
    pub fn author_holds(&self, tag: &str, person: Option<&[u8]>, mid: &str, text: &str) -> bool {
        let Some(key) = person.filter(|k| k.len() == 32) else { return false };
        if tag.is_empty() {
            return false;
        }
        let Some(got) = unb64(tag).filter(|g| g.len() == 16) else { return false };
        self.item_keys().iter().any(|k| unb64(&wire_author_tag(key, k, mid, text)).is_some_and(|want| same_bytes(&got, &want)))
    }
    /// The person's tag on a group word carried on this pipe (iOS MontanaGroup.ownTag): over the item key of the day.
    pub fn author_tag(&self, person: &[u8], mid: &str, text: &str, day: i64) -> String {
        wire_author_tag(person, &self.item_key(day), mid, text)
    }

    // ── the proofs that stand on the secret itself ──

    /// The key confirmation a first letter carries: SHA-256("mt-first-conf" ‖ 0 ‖ secret ‖ ct)[0..16] in base64url without
    /// padding; a pipe whose secret the walk erased confirms nothing here (the confirmation kept at the pipe's birth is the
    /// meeting's, meeting::confirm).
    pub fn first_confirm(&self, ct: &[u8]) -> String {
        if self.holds_origin() { b64url(&domained("mt-first-conf", &[&self.secret, ct])[..16]) } else { String::new() }
    }
    /// ONE PERSON, ONE CONVERSATION (iOS MTSamePair): the tag of this pipe (the older) beside a pipe just born (the newer), over
    /// the two roots from TAG_WALK_DAY, over the two established secrets before it. This port does not fold two pipes yet.
    #[allow(dead_code)]
    pub fn same_tag(&self, domain: &str, newer: &PipeKey, day: i64) -> String {
        if day < TAG_WALK_DAY && self.holds_origin() && newer.holds_origin() {
            return hex16(&domained(domain, &[&self.secret, &newer.secret]));
        }
        hex16(&domained(domain, &[&self.root_key(), &newer.root_key()]))
    }
    /// The proofs a receiver accepts: the one of the day and, on TAG_WALK_DAY and the day after, the one over the secrets.
    #[allow(dead_code)]
    pub fn same_tags(&self, domain: &str, newer: &PipeKey, day: i64) -> Vec<String> {
        let now = self.same_tag(domain, newer, day);
        if !(TAG_WALK_DAY..ERASE_DAY).contains(&day) || !self.holds_origin() || !newer.holds_origin() {
            return vec![now];
        }
        vec![now, hex16(&domained(domain, &[&self.secret, &newer.secret]))]
    }
    /// This pipe's place in the one order both ends compute alike: kept for good, the same bytes before the floor and after.
    pub fn same_rank(&self) -> [u8; 32] {
        match &self.floor {
            Some(f) => f.rank,
            None => domained("mt-same-rank", &[&self.secret]),
        }
    }

    /// The local name of a correspondence: SHA-256 of the secret, sixteen bytes, kept in the floor.
    fn ref_bytes(&self) -> Option<[u8; 16]> {
        if let Some(f) = &self.floor {
            return Some(f.reference);
        }
        (self.secret.len() == 32).then(|| {
            let mut r = [0u8; 16];
            r.copy_from_slice(&sha(&[&self.secret])[..16]);
            r
        })
    }
    pub fn reference(&self) -> Option<String> {
        if !self.is_whole() {
            return None;
        }
        self.ref_bytes().map(|r| hex(&r))
    }

    /// The record of the permanent derivatives, made at the pipe's birth and carried by the floor after it.
    pub fn kept(&self) -> Kept {
        if let Some(f) = &self.floor {
            return Kept { root: f.root, fingerprint: f.fingerprint, face_bid: hex(&f.face_bid), face_key: f.face_key, pair: f.pair,
                          reference: self.reference() };
        }
        Kept {
            root: Self::root_of(&self.secret),
            fingerprint: crate::fabric::fingerprint_material(&self.secret),
            face_bid: hex(&domained("mt-pipe-f", &[&self.secret])),
            face_key: domained("mt-pipe-fk", &[&self.secret]),
            pair: hmac(&self.secret, b"mt-coin-pair-v1"),
            reference: self.reference(),
        }
    }

    /// THE SAME CORRESPONDENCE, PROVEN BY WHAT IT DERIVES (D3, #37, decision 188; Book::establish under MONTANA_D3_SAME_PIPE):
    /// two forms of one pipe -- the secret and a floor of it, or two floors -- each walked to the later floor day give the same
    /// whole floor, byte for byte (the walk's value, root, rank, fingerprint material, face name, face key, pair tag and name);
    /// two whole secrets are one pipe exactly when they are equal. A name alone, or any one quantity, never makes two pipes one.
    pub fn same_pipe(&self, other: &PipeKey) -> bool {
        if !self.is_whole() || !other.is_whole() {
            return false;
        }
        if self.holds_origin() && other.holds_origin() {
            return same_bytes(&self.secret, &other.secret);
        }
        let f = self.floor_day().unwrap_or(i64::MIN).max(other.floor_day().unwrap_or(i64::MIN));
        let form = |k: &PipeKey| k.floor_on(f).map(|fl| PipeKey { secret: vec![], floor: Some(Box::new(fl)), healing: None }.sealed_form());
        match (form(self), form(other)) {
            (Some(a), Some(b)) => same_bytes(&a, &b),
            _ => false,
        }
    }
    /// Of two forms of one pipe, the one standing further down the walk: a floor over the secret, the later floor over the
    /// earlier. A form never goes back.
    pub fn forward(have: &PipeKey, other: &PipeKey) -> PipeKey {
        match (have.floor_day(), other.floor_day()) {
            (None, Some(_)) => other.clone(),
            (Some(a), Some(b)) if b > a => other.clone(),
            _ => have.clone(),
        }
    }

    /// The first value of a healed walk (iOS 2323–2325): old on the answering letter's day, then ss, pk and ct.
    pub fn healed(old: &[u8], ss: &[u8], pk: &[u8], ct: &[u8]) -> Option<Vec<u8>> {
        if old.len() != 32 || ss.len() != 32 || pk.len() != OFFER_SIZE || ct.len() != ANSWER_SIZE {
            return None;
        }
        Some(domained("mt-pipe-heal", &[old, ss, pk, ct]).to_vec())
    }

    /// Of two crossing offers the lower encapsulation key stands; one key met twice is no crossing.
    pub fn offer_stands(mine: &[u8], theirs: &[u8]) -> bool {
        mine != theirs && mine < theirs
    }

    /// This pipe's heal answered on a UTC day, never over the next day's old value.
    pub fn heal(&self, answered_on: i64, ss: &[u8], pk: &[u8], ct: &[u8]) -> Option<Heal> {
        if answered_on < WALK_ANCHOR_DAY || self.floor.as_ref().is_some_and(|floor| steps(answered_on) < steps(floor.day)) {
            return None;
        }
        Self::healed(&self.walked(answered_on), ss, pk, ct).map(|first| Heal { day: answered_on, first })
    }

    pub fn with_healing(&self, healing: Option<Healing>) -> PipeKey {
        let mut key = self.clone();
        key.healing = healing;
        key
    }

    /// The pipe standing on the healed walk once the old one was erased by a healed letter of the other side.
    pub fn rebased(&self) -> Option<PipeKey> {
        let healing = self.healing.as_ref().filter(|healing| !healing.holds_old)?;
        if !self.is_whole() {
            return None;
        }
        let kept = self.kept();
        let (reference, bid) = (self.ref_bytes()?, unhex(&kept.face_bid).filter(|b| b.len() == 32)?);
        Some(PipeKey {
            secret: vec![],
            floor: Some(Box::new(Floor {
                day: healing.heal.day + 1,
                value: arr32(&healing.heal.first),
                root: self.root_key(),
                rank: self.same_rank(),
                fingerprint: kept.fingerprint,
                face_bid: arr32(&bid),
                face_key: kept.face_key,
                pair: kept.pair,
                reference,
            })),
            healing: None,
        })
    }

    // ── the forms a store of this machine seals: the one road the bytes take out of this type ──

    fn sealed_form(&self) -> Vec<u8> {
        let Some(f) = &self.floor else { return self.secret.clone() };
        let mut d = vec![FLOOR_FORM];
        d.extend_from_slice(&(f.day as u64).to_le_bytes());
        for p in [&f.value, &f.root, &f.rank, &f.fingerprint, &f.face_bid, &f.face_key, &f.pair] {
            d.extend_from_slice(p);
        }
        d.extend_from_slice(&f.reference);
        d
    }
    fn from_sealed(d: &[u8]) -> PipeKey {
        if d.len() != FLOOR_FORM_SIZE || d[0] != FLOOR_FORM {
            return PipeKey::new(d);
        }
        let day = u64::from_le_bytes(d[1..9].try_into().unwrap()) as i64;
        let part = |n: usize| arr32(&d[9 + 32 * n..9 + 32 * n + 32]);
        let mut reference = [0u8; 16];
        reference.copy_from_slice(&d[9 + 32 * 7..FLOOR_FORM_SIZE]);
        PipeKey {
            secret: vec![],
            floor: Some(Box::new(Floor { day, value: part(0), root: part(1), rank: part(2), fingerprint: part(3), face_bid: part(4),
                                         face_key: part(5), pair: part(6), reference })),
            healing: None,
        }
    }
    /// One pipe as a store seals it on a day (base64 of its form, floored by the day): a seat's handed pipe.
    pub fn sealed_b64(&self, day: i64) -> String {
        b64(&self.floored(day).sealed_form())
    }
    /// One pipe read back from a store, floored by the day of reading.
    pub fn from_sealed_b64(s: &str, day: i64) -> Option<PipeKey> {
        unb64(s).map(|d| Self::from_sealed(&d).floored(day))
    }
    /// A book as a store keeps it before sealing it: reference -> base64 of the form, every form floored on the day written.
    pub fn stored(keys: &BTreeMap<String, PipeKey>, day: i64) -> Map<String, Value> {
        keys.iter().map(|(r, k)| (r.clone(), Value::String(k.sealed_b64(day)))).collect()
    }
    /// A book read back, every form floored on the day read; a value that is no base64 is no pipe and is passed over.
    pub fn held(stored: &Map<String, Value>, day: i64) -> BTreeMap<String, PipeKey> {
        stored.iter().filter_map(|(r, v)| Some((r.clone(), Self::from_sealed_b64(v.as_str()?, day)?))).collect()
    }
    /// The whole book as JSON, and read back (iOS form, book) -- the book here writes its own map through stored and held, which
    /// keep a value that does not read as it lies (Book::write_pipes).
    #[cfg(test)]
    pub fn form(keys: &BTreeMap<String, PipeKey>, day: i64) -> Vec<u8> {
        Value::Object(Self::stored(keys, day)).to_string().into_bytes()
    }
    #[cfg(test)]
    pub fn book(form: &[u8], day: i64) -> BTreeMap<String, PipeKey> {
        match serde_json::from_slice::<Value>(form) {
            Ok(Value::Object(m)) => Self::held(&m, day),
            _ => BTreeMap::new(),
        }
    }
    /// Whether a form sealed on the disk holds more than the walk allows on the day: the book then writes itself again.
    pub fn lags(stored: &Map<String, Value>, day: i64) -> bool {
        stored.values().filter_map(|v| v.as_str()).any(|s| Self::from_sealed_b64(s, day).is_some_and(|k| k.sealed_b64(day) != s))
    }
    /// The head the archive seals under the seed («s:» ‖ base64 of the form): what a restore by the words answers from.
    pub fn archive_head(&self) -> Vec<u8> {
        [b"s:".as_slice(), b64(&self.sealed_form()).as_bytes()].concat()
    }
    /// A head read back: either form, floored on the day of reading; a record of another size is no pipe.
    pub fn from_archive_head(head: &str, day: i64) -> Option<PipeKey> {
        let k = Self::from_sealed(&unb64(head.strip_prefix("s:")?)?);
        k.is_whole().then(|| k.floored(day))
    }

    // ── the frozen vectors of the walk (iOS MTPipeKey.agreesWithCanon, every row) ──

    /// The walk of the pipe secret by the day, the root, the author field and the passage by the day, each row as iOS 2311 holds
    /// it (MTPipeKey.swift:313-379). What it rejects is named there: a walk that never steps, one stepping once, one counting the
    /// day's number, one without the domain; a root without its separator; a signer keeping the secret past the anchor; a tag that
    /// never walks or walks before TAG_WALK_DAY, an old tag accepted past the day after it, a floor a day off, a floor keeping the
    /// secret, a form in another order.
    pub fn agrees_with_canon() -> bool {
        let body_secret: Vec<u8> = (0..32).collect();
        let k = PipeKey::new(&body_secret);
        let a = WALK_ANCHOR_DAY;
        if hex(&k.body_key(1000)) != "e25542e65a84d1a5b5c72a13172ecb62319c18f21a59978a53e7dac2c22a966a"
            || k.walked(a) != body_secret
            || hex(&k.walked(a + 1)) != "5dcfd87692fcd444a4344d756dfa37abe670d2eb252aa9b402646be97cf3c50a"
            || hex(&k.walked(a + 2)) != "0c31775e80ae463de61fecee68bee8bb4ebbcee8e6b2045f8714e93ef18cb1b0"
            || hex(&k.body_key((a + 2) * MINUTES_PER_DAY)) != "925378a4bb83e7137cbb1f8cdaba8b79fc37e30d3159e4158da6e0fee8518bb5"
            || k.conv_w(a) != "d29425fcf807a325b74d8bb017420cfe2250cc47e8bdc6351f6006cd4a6bdc9b"
            || k.conv_w(a + 2) != "c665fdca8978b9c6e54b9a4f16a06ccac657ba7da259df0a8f3e2550d7c2d518"
        {
            return false;
        }
        let root = Self::root_of(&body_secret);
        if hex(&root) != "d9c3e7f533da2d0ed10b6729601c478ab29d1b0cafed60443113ee6a63824e11"
            || k.item_key(a) != body_secret
            || k.item_key(a + 1) != root
            || k.item_keys() != vec![root.to_vec(), body_secret.clone()]
        {
            return false;
        }
        let person = [0x33u8; 32];
        let root_tag = wire_author_tag(&person, &root, "m", "t");
        let anchor_tag = wire_author_tag(&person, &body_secret, "m", "t");
        if root_tag == anchor_tag
            || k.with_author(&[], Some(&person), "m", "t", a + 1) != [vec![0u8; 7], root_tag.as_bytes().to_vec(), vec![0]].concat()
            || k.with_author(&[], Some(&person), "m", "t", a) != [vec![0u8; 7], anchor_tag.as_bytes().to_vec(), vec![0]].concat()
            || !k.author_holds(&root_tag, Some(&person), "m", "t")
            || !k.author_holds(&anchor_tag, Some(&person), "m", "t")
        {
            return false;
        }
        let (y, e, mpd) = (TAG_WALK_DAY, ERASE_DAY, MINUTES_PER_DAY);
        let floored = k.floored(e + 1);
        hex(&k.tag((y - 1) * mpd + 5)) == "ec0c1a23e2450dd465356cd0b8175c4f"
            && hex(&k.tag(y * mpd + 5)) == "96d8937ada4908bcba9fd12c78b9c11c"
            && k.tags(y * mpd + 5).iter().map(|t| hex(t)).collect::<Vec<_>>() == ["96d8937ada4908bcba9fd12c78b9c11c", "cc6997487bfa3ed9c4a7542be4fc2bbb"]
            && k.tags(e * mpd + 5).len() == 1
            && k.floored(e - 1) == k
            && k.holds_origin()
            && !floored.holds_origin()
            && floored.is_whole()
            && floored.walked(20759) == floored.walked(0)
            && hex(&floored.walked(20759)) == "047f100cb7db75364efcefe4b58cf0038b1e1bc441884b06b5674eaf7b857523"
            && floored.sealed_form().len() == FLOOR_FORM_SIZE
            && hex(&sha(&[&floored.sealed_form()])) == "da6e42f21c5935263ba7339fdc38adfd36db060a69d335f97e21f0dfbf1ae119"
            && hex(&floored.tag((e + 1) * mpd + 5)) == "9e0c92b8f91b94880bf15eee1ca672c7"
            && floored.tag((e + 1) * mpd + 5) == k.tag((e + 1) * mpd + 5)
            && floored.body_key((e + 3) * mpd) == k.body_key((e + 3) * mpd)
            && floored.conv_w(e + 1) == k.conv_w(e + 1)
            && floored.kept() == k.kept()
            && floored.reference() == k.reference()
            && floored.same_rank() == k.same_rank()
            && Self::from_sealed(&floored.sealed_form()) == floored
            && floored.floored(e + 1) == floored
            && crate::pipeheal::agrees()
    }
}

/// THE CHECK STANDS BEFORE ANY HISTORY IS WRITTEN (iOS MontanaArchive.deriveKeys: the archive's keys are refused when the frozen
/// vectors do not reproduce): computed once per process; a failure is said in the trace, never silently.
pub fn agrees() -> bool {
    static OK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *OK.get_or_init(|| {
        let ok = PipeKey::agrees_with_canon();
        if !ok {
            crate::trace_line("pipes_canon FAIL -- the frozen vectors of the pipe do not reproduce: no history is written");
        }
        ok
    })
}

#[cfg(test)]
mod days;
#[cfg(test)]
mod tests;
