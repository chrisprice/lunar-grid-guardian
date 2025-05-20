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
    pub fn tick(&mut self, context: &TickContext) {
        *self = match self {
            EventState::Scheduled { event_start } | EventState::Acknowledged { event_start }
                if context.mission_time >= *event_start =>
            {
                EventState::Impacting {
                    event_end: *event_start + Time::new::<second>(EVENT_DURATION_SECONDS),
                }
            }
            EventState::Impacting { event_end } if context.mission_time >= *event_end => {
                EventState::Dormant
            }
            EventState::Scheduled { .. }
            | EventState::Acknowledged { .. }
            | EventState::Impacting { .. }
            | EventState::Dormant => *self,
        }
    }
}
