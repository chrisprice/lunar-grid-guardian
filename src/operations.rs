use crate::game_variables::GameVariables;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationsState {
    // Dormant: waiting for next supply drop to be scheduled.
    Dormant,
    // Supply drop is scheduled to arrive (become AwaitingAuthorization) at event_start.
    Scheduled { event_start: u32 },
    AwaitingAuthorization, // Supply drop has arrived and is ready for player to authorize docking
    // Docking is in progress, will complete at event_end (mission time).
    DockingInProgress { event_end: u32 },
}

impl OperationsState {
    pub fn new() -> Self {
        OperationsState::Dormant
    }

    /// Ticks the state.
    /// Returns true if docking is completed.
    pub fn tick(&mut self, current_mission_time: u32) -> bool{
        let mut docking_completed = false;
        *self = match self {
            OperationsState::Scheduled { event_start } if current_mission_time >= *event_start => {
                OperationsState::AwaitingAuthorization
            }
            OperationsState::DockingInProgress { event_end } if current_mission_time >= *event_end => {
                docking_completed = true;
                OperationsState::Dormant
            }
            OperationsState::Scheduled { .. }
            | OperationsState::AwaitingAuthorization
            | OperationsState::DockingInProgress { .. }
            | OperationsState::Dormant => *self,
        };
        docking_completed
    }

    /// Attempts to authorize docking.
    /// GameState should ensure operations are online before calling this.
    /// Returns true if authorization was successful and docking started.
    pub fn authorize_docking(&mut self, current_mission_time: u32, game_vars: &GameVariables) -> bool {
        if matches!(self, OperationsState::AwaitingAuthorization) {
            *self = OperationsState::DockingInProgress {
                event_end: current_mission_time + game_vars.supply_drop_docking_duration as u32,
            };
            true
        } else {
            false
        }
    }
}
