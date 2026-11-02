//! THE CLOCK GUARD BEFORE THE FIRST FLOOR (decision 188, the chairman's ruling on #38; MONTANA_CLOCK_GUARD, on by default, «0»
//! turns it off -- off is iOS 2311, the device's day alone). The rule as the chairman confirmed it for every port (#38, 25.10.2026
//! 18:11 UTC):
//! - d: the device's day; M: the highest day a node's Date header named, raised only by a node's Date, never lowered (any one
//!   door counts, the highest wins); c: the confirmed day, max(M, the day of a Date received now) -- none without either;
//!   H: the device's day at the first moment the first floor waited, Hc: c at that moment; N: the box's days, 4.
//! - d before the erase day: nothing floors, H and Hc stay. No c, or d <= c + 1: the floor applies on d, H and Hc cleared.
//!   Otherwise H = d and Hc = c if unset; d >= H + N: the floor on d (by the device); c >= Hc + N: the floor on d (by the
//!   confirmed day); else the floor waits, left = min(H + N - d, Hc + N - c). H and Hc are let go whenever the floor applies.
//! - Case 13 (the ruling of 25.10.2026 19:23 UTC): on the first read of a new day the book walks to that day -- the guard asked
//!   first, so the floor may still wait -- is written at once and only then handed out, in the running process (book.rs).
//! - Once the first floor applied the guard is not asked again; the guard keeps that in its record with M, H and Hc, sealed
//!   beside the pipe book (book.rs) and never in the archive. M is raised by any node answer carrying a Date, whatever its status
//!   -- not by a cancelled request or a failed transport. With no confirmed source a line says c=none. A record that does not
//!   read is never written over and the guard does not withhold; a restore asks the guard (the ruling of 25.10.2026 22:04 UTC). A Date is read here for this and for nothing else; no byte of the wire changes, and letters, labels
//!   and tags follow the device's day as always.
//!   A clock that stays ahead while no door answers after H keeps the floor waiting with the same `left` -- named and accepted.

use crate::pipekey::ERASE_DAY;
use std::sync::atomic::{AtomicI64, Ordering};

/// N: the days a box read listens at (iOS MTNodeWire.boxDays, D-2 .. D+1).
pub const BOX_DAYS: i64 = 4;

/// the highest day a door's Date named in this run
static HEARD: AtomicI64 = AtomicI64::new(i64::MIN);

/// A door's Date header heard (http.rs): its day raises the highest one of this run.
pub fn heard(node_s: i64) {
    if node_s > 0 {
        HEARD.fetch_max(node_s.div_euclid(86_400), Ordering::SeqCst);
    }
}
/// The second an HTTP Date header names (IMF-fixdate, «Sun, 26 Oct 2026 01:02:03 GMT»); None for another form.
pub fn seconds_of_http_date(v: &str) -> Option<i64> {
    let p: Vec<&str> = v.split_whitespace().collect();
    let [_, day, mon, year, time, "GMT"] = p.as_slice() else { return None };
    let m = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"].iter().position(|x| x == mon)? as i64 + 1;
    let t: Vec<i64> = time.split(':').map(|x| x.parse().ok()).collect::<Option<_>>()?;
    let [hh, mm, ss] = t.as_slice() else { return None };
    Some(crate::http::unix_of(year.parse().ok()?, m, day.parse().ok()?, *hh, *mm, *ss))
}
/// The highest day a door's Date named in this run; None before any answered.
pub fn day_heard() -> Option<i64> {
    let d = HEARD.load(Ordering::SeqCst);
    (d != i64::MIN).then_some(d)
}
/// The highest of the days a set of Date seconds names (any one door counts, the highest wins), as heard folds them.
#[cfg(test)]
pub fn highest(node_s: &[i64]) -> Option<i64> {
    node_s.iter().filter(|s| **s > 0).map(|s| s.div_euclid(86_400)).max()
}

/// What the guard says of the first floor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Guard {
    /// before the erase day: nothing floors
    Before,
    /// the floor applies on d; `cleared` -- a held H was let go
    Free { cleared: bool },
    /// the floor applies on d after the wait: by the device's count or by the confirmed day's
    Ceiling { since: i64, by_device: bool },
    /// the floor waits
    Held { since: i64, left: i64 },
}

impl Guard {
    /// Whether the floor applies on d.
    pub fn floors(&self) -> bool {
        matches!(self, Guard::Free { .. } | Guard::Ceiling { .. })
    }
    /// The trace line of the decision, if it is one the rule names.
    pub fn line(&self, d: i64, c: Option<i64>) -> Option<String> {
        let c = c.map_or("none".to_string(), |c| c.to_string()); // no confirmed source: «none», one form on every port
        match self {
            Guard::Before | Guard::Free { cleared: false } => None,
            Guard::Free { cleared: true } => Some(format!("floor_free d={d} c={c}")),
            Guard::Ceiling { since, by_device } => {
                Some(format!("floor_ceiling d={d} c={c} since={since} by={}", if *by_device { "device" } else { "confirmed" }))
            }
            Guard::Held { since, left } => Some(format!("floor_held d={d} c={c} since={since} left={left}")),
        }
    }
}

/// THE RULE (pure): the decision on the device's day d with the confirmed day c and the hold (H, Hc) kept, and the hold kept after.
pub fn decide(d: i64, c: Option<i64>, hold: Option<(i64, i64)>) -> (Guard, Option<(i64, i64)>) {
    if d < ERASE_DAY {
        return (Guard::Before, hold);
    }
    let Some(c) = c.filter(|c| d > c + 1) else { return (Guard::Free { cleared: hold.is_some() }, None) };
    let (h, hc) = hold.unwrap_or((d, c));
    // H and Hc are let go whenever the floor applies (rule 2 or rule 3)
    if d >= h + BOX_DAYS {
        (Guard::Ceiling { since: h, by_device: true }, None)
    } else if c >= hc + BOX_DAYS {
        (Guard::Ceiling { since: h, by_device: false }, None)
    } else {
        (Guard::Held { since: h, left: (h + BOX_DAYS - d).min(hc + BOX_DAYS - c) }, Some((h, hc)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The twelve checks of the rule (#38, eraseDay 20760), on the rule itself; book.rs holds them on the book with what it keeps.
    #[test]
    fn the_rule_holds_its_twelve_checks() {
        let line = |d: i64, c: Option<i64>, hold: Option<(i64, i64)>| {
            let (g, next) = decide(d, c, hold);
            (g.floors(), g.line(d, c), next)
        };
        // 1: d 20760, c 20760 -- the floor on 20760
        assert_eq!(line(20760, Some(20760), None), (true, None, None));
        // 2: d 20800, c 20760, nothing held -- withheld, H 20800, Hc 20760
        assert_eq!(line(20800, Some(20760), None),
                   (false, Some("floor_held d=20800 c=20760 since=20800 left=4".into()), Some((20800, 20760))));
        // 3 and 12: d 20801, H 20800 kept, no answer, M 20760 -- withheld, left 3, the same on every load
        for _ in 0..3 {
            assert_eq!(line(20801, Some(20760), Some((20800, 20760))),
                       (false, Some("floor_held d=20801 c=20760 since=20800 left=3".into()), Some((20800, 20760))));
        }
        // 4: d 20804 -- the floor by the device's count
        assert_eq!(line(20804, Some(20760), Some((20800, 20760))),
                   (true, Some("floor_ceiling d=20804 c=20760 since=20800 by=device".into()), None));
        // 5: the clock set right, d 20761, c 20761 -- the floor on 20761, H let go
        assert_eq!(line(20761, Some(20761), Some((20800, 20760))), (true, Some("floor_free d=20761 c=20761".into()), None));
        // 6: the clock set to 20750 -- nothing floors, H kept; back to 20802 with c 20760 -- withheld, left 2
        assert_eq!(line(20750, Some(20760), Some((20800, 20760))), (false, None, Some((20800, 20760))));
        assert_eq!(line(20802, Some(20760), Some((20800, 20760))).1.as_deref(), Some("floor_held d=20802 c=20760 since=20800 left=2"));
        // 7: doors that disagree (20760 and 20790), d 20790 -- c 20790, the floor on 20790
        let c7 = highest(&[20760 * 86_400 + 5, 20790 * 86_400 + 5]);
        assert_eq!(c7, Some(20790));
        assert_eq!(line(20790, c7, None), (true, None, None));
        // 8: no M ever, no answer, d 20800 -- the floor on 20800
        assert_eq!(line(20800, None, None), (true, None, None));
        // 10: the clock set back to 20790 while waiting, the nodes confirm 20764 -- c >= Hc + 4: the floor by the confirmed day
        assert_eq!(line(20790, Some(20764), Some((20800, 20760))),
                   (true, Some("floor_ceiling d=20790 c=20764 since=20800 by=confirmed".into()), None));
        // 11: d 20798 after H 20800, c 20762 -- withheld, left min(6, 2)
        assert_eq!(line(20798, Some(20762), Some((20800, 20760))).1.as_deref(), Some("floor_held d=20798 c=20762 since=20800 left=2"));
        // (9: a book already holding its floor does not ask the guard -- book.rs)
        // no c and a hold let go: «c» stands empty
        assert_eq!(Guard::Free { cleared: true }.line(20800, None).as_deref(), Some("floor_free d=20800 c=none"));
    }

    #[test]
    fn an_http_date_reads_as_its_second() {
        assert_eq!(seconds_of_http_date("Tue, 03 Nov 2026 00:00:05 GMT").map(|s| s.div_euclid(86_400)), Some(20760));
        assert_eq!(seconds_of_http_date("Tue, 03 Nov 2026 00:00:05 GMT").map(|s| s.rem_euclid(86_400)), Some(5));
        assert_eq!(seconds_of_http_date("Tuesday, 03-Nov-26 00:00:05 GMT"), None);
        assert_eq!(seconds_of_http_date(""), None);
    }

    #[test]
    fn a_door_heard_raises_the_highest_day_and_nothing_else() {
        heard(20790 * 86_400 + 7);
        heard(20760 * 86_400 + 7);
        heard(0);
        assert!(day_heard().is_some_and(|d| d >= 20790));
        assert_eq!(highest(&[]), None);
    }
}
