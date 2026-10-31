// Fullbright, a native mod: toggle full brightness in-game with a key the
// player can rebind in Options > Mods. This file is both the mod's Cargo
// library (its tests) and the module the bundle adds to the game.
use std::cell::RefCell;

/// The mod's id (its settings folder and its keys' lines), name and version.
pub const ID: &str = "fullbright";
pub const NAME: &str = "Fullbright";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "See in the dark: toggle full brightness with a key you can rebind. \
Never touches your brightness slider, so turning it off gives your own brightness back.";

/// The toggle key's name, and its default.
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.b";

/// The gamma strengths the settings page cycles through, in percent of the
/// video slider's 100% "Bright": 1500% is the classic gamma-15 fullbright.
pub const GAMMA_LEVELS: [u32; 7] = [100, 150, 200, 300, 500, 1000, 1500];
/// The gamma strength kept for a stored value outside the levels.
const DEFAULT_GAMMA_PERCENT: u32 = 1500;

/// How the lightmap is brightened while the mod is on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Every lightmap entry fully lit, as Horizon Extras' full light does.
    Full,
    /// The video slider's gamma past its 100% limit, at `gamma_percent`.
    Gamma,
}

impl Mode {
    /// From the stored value; full light for the ones missing or unknown.
    pub fn read(value: &str) -> Self {
        match value.trim() {
            "gamma" => Self::Gamma,
            _ => Self::Full,
        }
    }

    /// The value to store, as `read` takes it.
    pub fn stored(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Gamma => "gamma",
        }
    }

    /// The settings page's label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Full => "Full light",
            Self::Gamma => "Gamma",
        }
    }
}

/// The mod's state: the toggle, the mode and the gamma strength. One file per
/// key under `horizon-mods/.horizon-mod-data/fullbright/`, so a player keeps
/// them between sessions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State {
    pub on: bool,
    pub mode: Mode,
    /// One of `GAMMA_LEVELS`, percent of the 100% "Bright" slider.
    pub gamma_percent: u32,
}

impl State {
    /// From the stored values (`get` reads one key's file); off, full light
    /// and the classic gamma-15 for the ones missing, and the nearest level
    /// for a gamma strength outside them.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let on = get("on").is_some_and(|v| v.trim() == "true");
        let mode = get("mode").map_or(Mode::Full, |v| Mode::read(&v));
        let gamma_percent = get("gamma")
            .and_then(|v| v.trim().parse::<u32>().ok())
            .map_or(DEFAULT_GAMMA_PERCENT, |v| {
                GAMMA_LEVELS
                    .iter()
                    .copied()
                    .min_by_key(|&level| level.abs_diff(v))
                    .unwrap_or(DEFAULT_GAMMA_PERCENT)
            });
        Self {
            on,
            mode,
            gamma_percent,
        }
    }

    /// The gamma to give the renderer in gamma mode: 100% is the slider's own
    /// "Bright", 1500% the classic gamma-15 fullbright.
    pub fn gamma(self) -> f32 {
        self.gamma_percent as f32 / 100.0
    }

    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        let on = if self.on { "ON" } else { "OFF" };
        vec![
            format!("Fullbright: {on}"),
            format!("Mode: {}", self.mode.label()),
            format!("Gamma strength: {}%", self.gamma_percent),
        ]
    }

    /// A click on button `i` of `labels`.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.on = !self.on,
            1 => {
                self.mode = match self.mode {
                    Mode::Full => Mode::Gamma,
                    Mode::Gamma => Mode::Full,
                }
            }
            2 => {
                // An unknown level counts as the first.
                let at = GAMMA_LEVELS
                    .iter()
                    .position(|&level| level == self.gamma_percent)
                    .unwrap_or(0);
                self.gamma_percent = GAMMA_LEVELS[(at + 1) % GAMMA_LEVELS.len()];
            }
            _ => {}
        }
    }

    /// What to store, key by key.
    pub fn stored(&self) -> [(&'static str, String); 3] {
        let on = if self.on { "true" } else { "false" };
        [
            ("on", on.to_owned()),
            ("mode", self.mode.stored().to_owned()),
            ("gamma", self.gamma_percent.to_string()),
        ]
    }
}

thread_local! {
    /// The state, read once from the files, then kept.
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// The state, read once through `get` (one file per key).
pub fn state(get: impl Fn(&str) -> Option<String>) -> State {
    STATE.with_borrow_mut(|s| *s.get_or_insert_with(|| State::read(get)))
}

/// Replaces the state (the key toggled it or the settings page changed it).
pub fn set_state(state: State) {
    STATE.with_borrow_mut(|s| *s = Some(state));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn defaults_to_off_full_light_and_the_classic_gamma() {
        let s = State::read(stored(&[]));
        assert_eq!(
            s,
            State {
                on: false,
                mode: Mode::Full,
                gamma_percent: 1500,
            }
        );
        assert_eq!(s.gamma(), 15.0);
        assert_eq!(
            s.labels(),
            [
                "Fullbright: OFF",
                "Mode: Full light",
                "Gamma strength: 1500%"
            ]
        );
    }

    #[test]
    fn reads_the_stored_state() {
        let s = State::read(stored(&[
            ("on", "true\n"),
            ("mode", "gamma"),
            ("gamma", "300"),
        ]));
        assert_eq!(
            s,
            State {
                on: true,
                mode: Mode::Gamma,
                gamma_percent: 300,
            }
        );
        assert_eq!(s.gamma(), 3.0);
    }

    #[test]
    fn an_unknown_gamma_snaps_to_the_nearest_level() {
        assert_eq!(State::read(stored(&[("gamma", "250")])).gamma_percent, 200);
        assert_eq!(
            State::read(stored(&[("gamma", "99999")])).gamma_percent,
            1500
        );
        assert_eq!(State::read(stored(&[("gamma", "abc")])).gamma_percent, 1500);
    }

    #[test]
    fn clicks_cycle_the_buttons() {
        let mut s = State::read(stored(&[]));
        s.click(0);
        assert!(s.on);
        s.click(1);
        assert_eq!(s.mode, Mode::Gamma);
        s.click(2);
        assert_eq!(s.gamma_percent, 100);
        s.click(2);
        assert_eq!(s.gamma_percent, 150);
        s.click(3);
        assert_eq!(s.mode, Mode::Gamma);
        assert_eq!(
            s.labels(),
            ["Fullbright: ON", "Mode: Gamma", "Gamma strength: 150%"]
        );
        assert_eq!(
            s.stored(),
            [
                ("on", "true".into()),
                ("mode", "gamma".into()),
                ("gamma", "150".into()),
            ]
        );
    }

    #[test]
    fn modes_round_trip_through_their_stored_names() {
        assert_eq!(Mode::read("gamma"), Mode::Gamma);
        assert_eq!(Mode::read("full\n"), Mode::Full);
        assert_eq!(Mode::read("nonsense"), Mode::Full);
        assert_eq!(Mode::Full.stored(), "full");
        assert_eq!(Mode::Gamma.stored(), "gamma");
    }
}
