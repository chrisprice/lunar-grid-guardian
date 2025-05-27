use crate::system::system::System;
use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::Power;
use uom::si::f32::Time;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SupplyDrop {
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

#[derive(Debug, Clone, Copy, Default)]
pub struct OperationsState {
    pub system: System,
    pub supply_drop: SupplyDrop,
}

pub struct TickResult {
    pub power_consumed: Power,
    pub docking_completed: bool,
}

impl OperationsState {
    /// Ticks the state.
    /// Returns TickResult indicating power consumed and if docking completed.
    pub fn tick(&mut self, context: &TickContext) -> TickResult {
        self.system.tick(context);

        let mut docking_completed = false;
        let mut current_power_consumption = Power::ZERO;

        if self.system != System::Offline {
            current_power_consumption = context.game_vars.operations_base_power_demand;
            if let SupplyDrop::DockingInProgress { .. } = self.supply_drop {
                current_power_consumption += context.game_vars.operations_docking_spike_power;
            }
        }

        self.supply_drop = match self.supply_drop {
            SupplyDrop::Scheduled { event_start }
                if context.mission_time >= event_start =>
            {
                SupplyDrop::AwaitingAuthorization
            }
            SupplyDrop::DockingInProgress { event_end, .. }
                if context.mission_time >= event_end =>
            {
                docking_completed = true;
                SupplyDrop::Idle
            }
            s => s,
        };

        TickResult {
            power_consumed: current_power_consumption,
            docking_completed,
        }
    }

    /// Attempts to authorize docking.
    /// Returns true if authorization was successful and docking started.
    pub fn authorize_docking(&mut self, context: &TickContext) -> bool {
        if let System::Online { .. } = self.system {
            if self.supply_drop == SupplyDrop::AwaitingAuthorization {
                self.supply_drop = SupplyDrop::DockingInProgress {
                    event_end: context.mission_time
                        + context.game_vars.supply_drop_docking_duration,
                };
                return true;
            }
        }
        false
    }

    pub fn repair(&mut self, context: &TickContext) {
        self.system = self
            .system
            .repair(context.mission_time, context.game_vars);
    }

    pub fn damage(&mut self, amount: uom::si::f32::Ratio) {
        self.system.damage(amount);
    }
}
