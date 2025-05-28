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

pub struct Event {
    pub state: EventState,
    pub probability: Ratio,
}

impl Event {
    pub fn new(probability: Ratio) -> Self {
        Event {
            state: EventState::Dormant,
            probability,
        }
    }

    /// Ticks the event state based on the current mission time.
    /// Returns `true` if the event transitioned to impacting.
    pub fn tick(&mut self, context: &TickContext) -> bool {
        match self.state {
            EventState::Dormant => {
                if rng().random_bool(self.probability.value as f64) {
                    let event_start_time =
                        context.mission_time + context.game_vars.event_schedule_offset;
                    self.state = EventState::Scheduled {
                        event_start: event_start_time,
                    };
                }
                false
            }
            EventState::Scheduled { event_start } | EventState::Acknowledged { event_start }
                if context.mission_time >= event_start =>
            {
                self.state = EventState::Impacting {
                    event_end: event_start + context.game_vars.event_duration,
                };
                true
            }
            EventState::Impacting { event_end } if context.mission_time >= event_end => {
                self.state = EventState::Dormant;
                false
            }
            EventState::Scheduled { .. }
            | EventState::Acknowledged { .. }
            | EventState::Impacting { .. } => false,
        }
    }
}
