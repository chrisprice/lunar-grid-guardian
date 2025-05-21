use std::marker::PhantomData;
use uom::si::f32::{Ratio, Time};

use crate::tick_context::TickContext;

const LUNAR_DAY_DURATION: Time = Time {
    dimension: PhantomData,
    units: PhantomData,
    value: 29.5 * 24.0 * 60.0 * 60.0,
};

const LUNAR_PHASE_DURATION: Time = Time {
    dimension: PhantomData,
    units: PhantomData,
    value: LUNAR_DAY_DURATION.value / 2.0,
};

/// Represents the current lunar phase and time in cycle.
/// N.B. The elapsed time is scaled.
pub enum LunarPhase {
    Day { elapsed: Time },
    Night { elapsed: Time },
}

impl LunarPhase {
    pub fn from_tick_context(tick_context: &TickContext) -> Self {
        let lunar_time =
            tick_context.mission_time / tick_context.game_vars.mission_time_per_lunar_time;
        let elapsed = lunar_time % LUNAR_DAY_DURATION;
        if lunar_time % LUNAR_DAY_DURATION < LUNAR_PHASE_DURATION {
            LunarPhase::Day { elapsed }
        } else {
            LunarPhase::Night { elapsed }
        }
    }

    /// Returns the time until the next phase change (sunset or sunrise).
    pub fn remaining_time(&self) -> Time {
        match self {
            LunarPhase::Day { elapsed } | LunarPhase::Night { elapsed } => {
                LUNAR_PHASE_DURATION - *elapsed
            }
        }
    }

    /// Returns the ratio through the current phase.
    pub fn elapsed_ratio(&self) -> Ratio {
        match self {
            LunarPhase::Day { elapsed } | LunarPhase::Night { elapsed } => {
                *elapsed / LUNAR_PHASE_DURATION
            }
        }
    }
}
