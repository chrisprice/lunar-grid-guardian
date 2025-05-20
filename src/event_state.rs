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
    pub fn tick(&mut self, current_tick: Time) {
        *self = match self {
            EventState::Scheduled { event_start } | EventState::Acknowledged { event_start }
                if current_tick >= *event_start =>
            {
                EventState::Impacting {
                    event_end: *event_start + Time::new::<second>(EVENT_DURATION_SECONDS),
                }
            }
            EventState::Impacting { event_end } if current_tick >= *event_end => {
                EventState::Dormant
            }
            EventState::Scheduled { .. }
            | EventState::Acknowledged { .. }
            | EventState::Impacting { .. }
            | EventState::Dormant => *self,
        }
    }
}
