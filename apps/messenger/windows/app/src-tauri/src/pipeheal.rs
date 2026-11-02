//! The receive side of iOS 2323–2325's pipe heal: one ML-KEM-768 offer or answer in a letter's ninth field,
//! the healed walk beside the old one until a letter of the other side opens under the healed one.

use crate::pipekey::PipeKey;
use crate::wire::{b64, unb64};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const OFFER_SIZE: usize = 1184;
pub const ANSWER_SIZE: usize = 1088;
pub const RHO_SIZE: usize = 32;

mod bytes {
    use super::*;

    pub fn serialize<S: Serializer>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&b64(value))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let value = String::deserialize(deserializer)?;
        unb64(&value).ok_or_else(|| serde::de::Error::custom("invalid base64"))
    }
}

mod optional_bytes {
    use super::*;

    pub fn serialize<S: Serializer>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.serialize_str(&b64(value)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error> {
        let value = Option::<String>::deserialize(deserializer)?;
        value.map(|value| unb64(&value).ok_or_else(|| serde::de::Error::custom("invalid base64"))).transpose()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Walk {
    Healed,
    Old,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Offering,
    Answering,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heal {
    pub day: i64,
    #[serde(with = "bytes")]
    pub first: Vec<u8>,
}

impl Heal {
    pub fn walked(&self, on: i64) -> Option<Vec<u8>> {
        if on <= self.day {
            return None;
        }
        let mut value = self.first.clone();
        let mut at = self.day + 1;
        while at < on && at - self.day <= crate::pipekey::MAX_WALK {
            value = crate::wire::domained("mt-pipe-ratchet", &[&value]).to_vec();
            at += 1;
        }
        Some(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Healing {
    pub side: Role,
    pub heal: Heal,
    #[serde(rename = "holdsOld", default = "held")]
    pub holds_old: bool,
}

fn held() -> bool {
    true
}

impl Healing {
    pub fn new(side: Role, heal: Heal) -> Healing {
        Healing { side, heal, holds_old: true }
    }

    pub fn sends_healed(&self, day: i64) -> bool {
        day > self.heal.day && (self.side == Role::Offering || !self.holds_old)
    }

    pub fn opens(&self, day: i64) -> Vec<Walk> {
        let mut out = Vec::with_capacity(2);
        if day > self.heal.day {
            out.push(Walk::Healed);
        }
        if self.holds_old {
            out.push(Walk::Old);
        }
        out
    }

    pub fn opened(&mut self, walk: Walk) {
        if walk == Walk::Healed {
            self.holds_old = false;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Part {
    Offer(Vec<u8>),
    Answer { ct: Vec<u8>, rho: Vec<u8> },
}

#[allow(dead_code)]
impl Part {
    pub fn field(&self) -> Vec<u8> {
        match self {
            Part::Offer(pk) => [&[1u8][..], pk].concat(),
            Part::Answer { ct, rho } => [&[2u8][..], ct, rho].concat(),
        }
    }

    pub fn parse(field: &[u8]) -> Option<Part> {
        match field.split_first() {
            Some((1, rest)) if rest.len() == OFFER_SIZE => Some(Part::Offer(rest.to_vec())),
            Some((2, rest)) if rest.len() == ANSWER_SIZE + RHO_SIZE => Some(Part::Answer {
                ct: rest[..ANSWER_SIZE].to_vec(),
                rho: rest[ANSWER_SIZE..].to_vec(),
            }),
            _ => None,
        }
    }

    pub fn rho(pk: &[u8]) -> &[u8] {
        &pk[pk.len() - RHO_SIZE..]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offer {
    #[serde(with = "bytes")]
    pub pk: Vec<u8>,
    #[serde(with = "bytes")]
    pub sk: Vec<u8>,
    pub day: i64,
    pub rode: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owed {
    #[serde(with = "bytes")]
    pub pk: Vec<u8>,
    #[serde(with = "bytes")]
    pub ct: Vec<u8>,
    #[serde(with = "bytes")]
    pub ss: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Side {
    pub offer: Option<Offer>,
    pub owed: Option<Owed>,
    pub healing: Option<Healing>,
    #[serde(with = "optional_bytes")]
    pub answered: Option<Vec<u8>>,
    pub rebased: bool,
}

#[allow(dead_code)]
impl Side {
    pub fn holds_one_walk(&self) -> bool {
        !self.healing.as_ref().is_some_and(|h| h.holds_old)
    }

    pub fn carry(&mut self, day: i64) -> Option<Part> {
        if let Some(owed) = &self.owed {
            return Some(Part::Answer { ct: owed.ct.clone(), rho: Part::rho(&owed.pk).to_vec() });
        }
        if !self.holds_one_walk() {
            return None;
        }
        if self.offer.as_ref().is_none_or(|offer| offer.day < day) {
            let (pk, sk) = crate::crypto::mlkem_keypair(&crate::crypto::random(crate::abi::MT_MLKEM_SEED_LEN)?)?;
            self.offer = Some(Offer { pk, sk, day, rode: false });
        }
        let offer = self.offer.as_ref().filter(|offer| offer.day == day && !offer.rode)?;
        Some(Part::Offer(offer.pk.clone()))
    }

    pub fn sent(&mut self, part: &Part, day: i64, pipe: &PipeKey) {
        match part {
            Part::Offer(pk) => {
                if self.offer.as_ref().is_some_and(|offer| &offer.pk == pk) {
                    self.offer.as_mut().unwrap().rode = true;
                }
            }
            Part::Answer { ct, .. } => {
                let Some(owed) = self.owed.clone().filter(|owed| &owed.ct == ct) else { return };
                self.healing = pipe.heal(day, &owed.ss, &owed.pk, &owed.ct).map(|heal| Healing::new(Role::Answering, heal));
                self.owed = None;
            }
        }
    }

    pub fn received(&mut self, part: &Part, day: i64, pipe: &PipeKey) {
        match part {
            Part::Offer(pk) => {
                if self.answered.as_deref() == Some(pk.as_slice()) || self.owed.as_ref().is_some_and(|owed| &owed.pk == pk) {
                    return;
                }
                if let Some(healing) = &self.healing {
                    if healing.holds_old {
                        if healing.side != Role::Answering {
                            return;
                        }
                        self.healing = None;
                    }
                }
                if self.offer.as_ref().is_some_and(|offer| offer.rode && PipeKey::offer_stands(&offer.pk, pk)) {
                    return;
                }
                self.offer = None;
                if let Some((ct, ss)) = crate::crypto::mlkem_encaps(pk) {
                    self.owed = Some(Owed { pk: pk.clone(), ct, ss });
                    self.answered = Some(pk.clone());
                }
            }
            Part::Answer { ct, rho } => {
                let Some(offer) = self.offer.clone().filter(|offer| offer.rode && Part::rho(&offer.pk) == rho) else { return };
                let Some(ss) = crate::crypto::mlkem_decaps(&offer.sk, ct) else { return };
                let Some(heal) = pipe.heal(day, &ss, &offer.pk, ct) else { return };
                self.healing = Some(Healing::new(Role::Offering, heal));
                self.offer = None;
            }
        }
    }
}

pub fn agrees() -> bool {
    use crate::wire::{hex, sha};
    let test_secret = sha(&[b"mt-vector-pipe-secret"]);
    let old = crate::wire::domained("mt-pipe-ratchet", &[&test_secret]);
    let ss = vec![0x22; 32];
    let pk = vec![0x23; OFFER_SIZE];
    let ct = vec![0x24; ANSWER_SIZE];
    let low = [&[1u8][..], &vec![0xff; OFFER_SIZE - 1]].concat();
    let high = [&[2u8][..], &vec![0u8; OFFER_SIZE - 1]].concat();
    let Some(heal) = PipeKey::healed(&old, &ss, &pk, &ct) else { return false };
    hex(&test_secret) == "a225e0b87df8a5568331badccfa4b217aaee3c7077b6021a52fab89d8856ef56"
        && hex(&old) == "92aa039f4d82a59e12a6e65eacaa998fa06455b63e4aef6405556f9e99e4c666"
        && hex(&heal) == "112f58fbae5637bb976ac9c6a85ed9bf5d26669a511de8310387a62419d81adc"
        && hex(&crate::wire::domained("mt-pipe-ratchet", &[&heal])) == "43ee0cf3597a6d9450483d33820847866be77ad8cf53191102c0ee8dd401f283"
        && PipeKey::healed(&crate::wire::domained("mt-pipe-ratchet", &[&old]), &ss, &pk, &ct).map(|v| hex(&v))
            == Some("85169530874db497c0d4f527900ac25eff7fc12ac2d132e7c9984b42836924b5".into())
        && PipeKey::healed(&old, &ss, &pk[..OFFER_SIZE - 1], &ct).is_none()
        && PipeKey::healed(&old, &ss, &pk, &[ct, vec![0]].concat()).is_none()
        && PipeKey::offer_stands(&pk, &vec![0x24; OFFER_SIZE])
        && !PipeKey::offer_stands(&vec![0x24; OFFER_SIZE], &pk)
        && !PipeKey::offer_stands(&pk, &pk)
        && PipeKey::offer_stands(&low, &high)
        && !PipeKey::offer_stands(&high, &low)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipekey::{WALK_ANCHOR_DAY, MINUTES_PER_DAY};
    use crate::wire::{letter_heal, with_author_tag, with_heal};

    #[test]
    fn the_letters_of_a_heal_agree_with_ios() {
        let pipe = PipeKey::new(&[0x5au8; 32]);
        let day = WALK_ANCHOR_DAY + 1;
        let head = b"m1\0hi\0Ann\0A\0";
        let mut offering = Side::default();
        let mut answering = Side::default();
        let Some(Part::Offer(pk)) = offering.carry(day) else { panic!("offer") };
        let field = Part::Offer(pk.clone()).field();
        let letter = with_heal(&with_author_tag(head, "QUFBQUFBQUFBQUFBQUFBQQ=="), &field);
        assert!(letter.len() <= crate::wire::ENVELOPE);
        assert_eq!(Part::parse(&letter_heal(&letter).unwrap()), Some(Part::Offer(pk.clone())));
        assert_eq!(crate::wire::fields(&letter, 0, 8), ["m1", "hi", "Ann", "A", "", "", "", "QUFBQUFBQUFBQUFBQUFBQQ=="]);
        offering.sent(&Part::Offer(pk.clone()), day, &pipe);
        answering.received(&Part::Offer(pk.clone()), day, &pipe);
        let answer = answering.carry(day).unwrap();
        let Part::Answer { ref ct, ref rho } = answer else { panic!("answer") };
        assert_eq!(rho, &pk[pk.len() - RHO_SIZE..]);
        answering.sent(&answer, day, &pipe);
        offering.received(&answer, day, &pipe);
        assert_eq!(offering.healing.as_ref().unwrap().heal, answering.healing.as_ref().unwrap().heal);
        assert_eq!(offering.healing.as_ref().unwrap().side, Role::Offering);
        assert_eq!(answering.healing.as_ref().unwrap().side, Role::Answering);

        let window = (day + 1) * MINUTES_PER_DAY + 7;
        let offering_pipe = pipe.with_healing(offering.healing.clone());
        let answering_pipe = pipe.with_healing(answering.healing.clone());
        assert_ne!(offering_pipe.body_key(window), pipe.body_key(window));
        assert_eq!(answering_pipe.body_keys(window)[0], (Walk::Healed, offering_pipe.body_key(window)));
        assert_eq!(answering_pipe.body_key(window), pipe.body_key(window));
        assert!(offering_pipe.body_keys(window).contains(&(Walk::Old, answering_pipe.body_key(window))));
        answering.healing.as_mut().unwrap().opened(Walk::Healed);
        let rebased = pipe.with_healing(answering.healing.clone()).rebased().unwrap();
        assert_eq!(rebased.body_key(window), offering_pipe.body_key(window));
        assert!(!rebased.body_keys(window).contains(&(Walk::Old, pipe.body_key(window))));
        assert_eq!(rebased.kept(), pipe.kept());

        let encoded = serde_json::to_vec(&offering).unwrap();
        assert_eq!(serde_json::from_slice::<Side>(&encoded).unwrap(), offering);
        let long = [b"m2\0".as_slice(), &vec![b'x'; 600], b"\0Ann\0A\0"].concat();
        assert_eq!(with_heal(&long, &field), long);
        assert!(Part::parse(&field[..field.len() - 1]).is_none());
        assert!(Part::parse(&[&[9u8][..], &pk].concat()).is_none());
        let _ = ct;
    }
}
