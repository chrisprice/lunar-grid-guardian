use crate::rng;
use crate::tick_context::TickContext;
use rand::Rng;
use uom::si::f32::{Ratio, Time};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventState {
    Dormant,
    Scheduled { event_start: Time },
    Acknowledged { event_start: Time },
    Impacting { event_end: Time },
}

impl EventState {
    /// Ticks the event state based on the current mission time.
    /// Returns `true` if the event transitioned to impacting.
    pub fn tick(&mut self, context: &TickContext) -> bool {
        match self {
            EventState::Scheduled { event_start } | EventState::Acknowledged { event_start }
                if context.mission_time >= *event_start =>
            {
                *self = EventState::Impacting {
                    event_end: *event_start + context.game_vars.event_duration,
                };
                true
            }
            EventState::Impacting { event_end } if context.mission_time >= *event_end => {
                *self = EventState::Dormant;
                false
            }
            EventState::Scheduled { .. }
            | EventState::Acknowledged { .. }
            | EventState::Impacting { .. }
            | EventState::Dormant => false,
        }
    }

    /// Attempts to schedule the event if it is currently dormant.
    ///
    /// Returns `true` if the event was successfully scheduled.
    pub fn try_schedule(&mut self, context: &TickContext, probability: Ratio) -> bool {
        if let EventState::Dormant = self {
            if rng().random_bool(probability.value as f64) {
                let event_start_time =
                    context.mission_time + context.game_vars.event_schedule_offset;
                *self = EventState::Scheduled {
                    event_start: event_start_time,
                };
                return true;
            }
        }
        false
    }
}
