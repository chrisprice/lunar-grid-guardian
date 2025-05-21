use crate::tick_context::TickContext;
use uom::si::f32::Power;
use uom::si::f32::Time;

#[derive(Debug, Clone, Copy, Default)]
pub enum OperationsState {
    #[default]
    Idle,
    Scheduled {
        event_start: Time,
    },
    AwaitingAuthorization,
    DockingInProgress {
        event_end: Time,
    },
}

pub struct TickResult {
    pub power_consumed: Power,
    pub docking_completed: bool,
}

impl OperationsState {
    /// Ticks the state.
    /// Returns TickResult indicating power consumed and if docking completed.
    pub fn tick(&mut self, context: &TickContext) -> TickResult {
        let mut docking_completed = false;

        let mut current_power_consumption = context.game_vars.operations_base_power_demand;

        if matches!(*self, OperationsState::DockingInProgress { .. }) {
            current_power_consumption += context.game_vars.operations_docking_spike_power;
        }

        let new_state = match *self {
            OperationsState::Scheduled { event_start } if context.mission_time >= event_start => {
                OperationsState::AwaitingAuthorization
            }
            OperationsState::DockingInProgress { event_end, .. }
                if context.mission_time >= event_end =>
            {
                docking_completed = true;
                OperationsState::Idle
            }
            s => s,
        };
        *self = new_state;

        TickResult {
            power_consumed: current_power_consumption,
            docking_completed,
        }
    }

    /// Attempts to authorize docking.
    /// GameState should ensure operations are online before calling this.
    /// Returns true if authorization was successful and docking started.
    pub fn authorize_docking(&mut self, context: &TickContext) -> bool {
        if matches!(self, OperationsState::AwaitingAuthorization) {
            *self = OperationsState::DockingInProgress {
                event_end: context.mission_time + context.game_vars.supply_drop_docking_duration,
            };
            true
        } else {
            false
        }
    }
}
