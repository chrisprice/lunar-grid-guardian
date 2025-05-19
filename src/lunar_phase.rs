pub const LUNAR_DAY_SECONDS: f32 = 29.5 * 24.0 * 60.0 * 60.0;

/// Represents the current lunar phase and time in cycle.
pub enum LunarPhase {
    Day { elapsed: f32 },
    Night { elapsed: f32 },
}

impl LunarPhase {
    /// Returns the time in seconds until the next phase change (sunset or sunrise).
    pub fn remaining(&self) -> f32 {
        match self {
            LunarPhase::Day { elapsed: time_in_cycle } => {
                (LUNAR_DAY_SECONDS / 2.0) - time_in_cycle
            }
            LunarPhase::Night { elapsed: time_in_cycle } => {
                LUNAR_DAY_SECONDS - time_in_cycle
            }
        }
    }
}
