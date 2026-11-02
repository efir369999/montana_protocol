//! THE RULES BEYOND iOS 2311, EACH BEHIND ITS OWN SWITCH, all three decided: decision 188 of the author of Montana
//! (25.10.2026 18:45 UTC), option 2 for every platform, with the chairman's rulings on GitHub #36 (D1), #37 (D3) and #38 (the
//! clock guard) -- they are ON by default. «1» in the environment turns a switch on, «0» turns it off; OFF is iOS 2311
//! exactly.
//! - MONTANA_D1_KEEP_CONFIRM (ON, #36): a first letter's confirmation kept beside its ciphertext from the pipe's birth and
//!   carried after the floor with that ciphertext alone (OFF: empty once the pipe is floored, MTPipeKey.swift:166);
//! - MONTANA_D3_SAME_PIPE (ON, #37): a pipe met again in another form is the same pipe when both, walked to the later floor day,
//!   give the same whole floor; the forward form is kept (OFF: the form held must equal the one met, the floor included,
//!   MTPipe.swift:273);
//! - MONTANA_CLOCK_GUARD (ON, #38): the first floor waits while the device's day stands more than a day past the day a node's
//!   Date confirmed, for at most four days by either count (nodeclock); and, case 13 (25.10.2026 19:23 UTC), the book walks to
//!   the day on its first read of each new day, is written at once and only then handed out, every form written following the
//!   walked book (OFF: the device's day alone and the book as read at the start, MTNodeWire.dayWindow, MTPipe.swift:221-263).
//!
//! None of them changes a byte of the wire but the walk of the book at 03.11 00:00 (the window of 02.11 23:59 then hears only
//! the walked tag).

/// The switches, as one value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rules {
    pub d1_keep_confirm: bool,
    pub d3_same_pipe: bool,
    pub clock_guard: bool,
}

impl Rules {
    /// iOS 2311 exactly: every switch off.
    #[cfg(test)]
    pub const IOS: Rules = Rules { d1_keep_confirm: false, d3_same_pipe: false, clock_guard: false };
    /// The switches as decision 188 sets them: D1, D3 and the clock guard on.
    pub const DECIDED: Rules = Rules { d1_keep_confirm: true, d3_same_pipe: true, clock_guard: true };
    /// Every switch on.
    #[cfg(test)]
    pub const ALL: Rules = Rules { d1_keep_confirm: true, d3_same_pipe: true, clock_guard: true };
}

impl Default for Rules {
    fn default() -> Rules {
        Rules::DECIDED
    }
}

/// A switch as the environment sets it: «1» on, «0» off, anything else its default.
fn flag(value: Option<String>, default: bool) -> bool {
    match value.as_deref().map(str::trim) {
        Some("1") => true,
        Some("0") => false,
        _ => default,
    }
}

/// The switches of this run, read once from the environment.
pub fn get() -> Rules {
    #[cfg(test)]
    if let Some(r) = HERE.with(|h| h.get()) {
        return r;
    }
    static R: std::sync::OnceLock<Rules> = std::sync::OnceLock::new();
    *R.get_or_init(|| {
        let d = Rules::DECIDED;
        let v = |name: &str| std::env::var(name).ok();
        let r = Rules {
            d1_keep_confirm: flag(v("MONTANA_D1_KEEP_CONFIRM"), d.d1_keep_confirm),
            d3_same_pipe: flag(v("MONTANA_D3_SAME_PIPE"), d.d3_same_pipe),
            clock_guard: flag(v("MONTANA_CLOCK_GUARD"), d.clock_guard),
        };
        if r != Rules::DECIDED {
            crate::trace_line(&format!("rules set apart from decision 188: {r:?}"));
        }
        r
    })
}

#[cfg(test)]
thread_local! {
    static HERE: std::cell::Cell<Option<Rules>> = const { std::cell::Cell::new(None) };
}

/// A test under the switches given, on its own thread (the code it drives runs there).
#[cfg(test)]
pub fn with<T>(r: Rules, f: impl FnOnce() -> T) -> T {
    struct Back(Option<Rules>);
    impl Drop for Back {
        fn drop(&mut self) {
            HERE.with(|h| h.set(self.0));
        }
    }
    let _back = Back(HERE.with(|h| h.replace(Some(r))));
    f()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_are_decision_188_and_each_switch_turns_off_or_on() {
        assert_eq!(Rules::default(), Rules::DECIDED);
        let decided = Rules { d1_keep_confirm: true, d3_same_pipe: true, clock_guard: true };
        assert_eq!(Rules::DECIDED, decided, "D1, D3 and the guard on");
        assert!(!flag(Some("0".into()), true) && flag(Some(" 1 ".into()), false));
        assert!(flag(None, true) && !flag(None, false) && flag(Some("yes".into()), true), "anything else keeps the default");
        with(Rules::ALL, || assert!(get().d1_keep_confirm && get().clock_guard));
        with(Rules::IOS, || assert_eq!(get(), Rules::IOS));
    }
}
