use crate::tick_context::TickContext;
use uom::si::f32::Time;
use uom::si::time::second;

const EVENT_DURATION_SECONDS: f32 = 3.0;

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
                    event_end: *event_start + Time::new::<second>(EVENT_DURATION_SECONDS),
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
}
