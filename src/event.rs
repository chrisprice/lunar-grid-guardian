use crate::display::demand_management::EventAlertStatus;
use crate::rng;
use crate::tick_context::TickContext;
use rand::Rng;
use uom::ConstZero;
use uom::si::f32::{Ratio, Time};

#[derive(Debug)]
pub enum EventState {
    Dormant,
    Scheduled { event_start: Time },
    Acknowledged { event_start: Time },
    Impacting { event_end: Time },
}

pub struct Event {
    state: EventState,
    probability: Ratio,
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

    /// Returns true if the event is currently impacting.
    pub fn is_impacting(&self) -> bool {
        matches!(self.state, EventState::Impacting { .. })
    }

    /// Returns the time remaining until the scheduled start or end of the event.
    pub fn countdown(&self, current_time: Time) -> Option<Time> {
        match self.state {
            EventState::Scheduled {
                event_start: event_time,
            }
            | EventState::Impacting {
                event_end: event_time,
            } => {
                let remaining = event_time - current_time;
                if remaining > Time::ZERO {
                    Some(remaining)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl From<&Event> for EventAlertStatus {
    fn from(event: &Event) -> Self {
        match event.state {
            EventState::Scheduled { .. } => EventAlertStatus::UnacknowledgedAlert,
            EventState::Acknowledged { .. } | EventState::Impacting { .. } => {
                EventAlertStatus::AcknowledgedAlert
            }
            EventState::Dormant => EventAlertStatus::NoAlert,
        }
    }
}
