const EVENT_DURATION: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventState {
    Dormant,
    Scheduled { event_start: u32 },
    Acknowledged { event_start: u32 },
    Impacting { event_end: u32 },
}

impl EventState {
    pub fn tick(&mut self, current_tick: u32) {
        *self = match self {
            EventState::Scheduled { event_start } | EventState::Acknowledged { event_start }
                if current_tick >= *event_start =>
            {
                EventState::Impacting {
                    event_end: *event_start + EVENT_DURATION,
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
