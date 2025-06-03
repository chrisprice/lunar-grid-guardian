use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::Power;
use uom::si::f32::Time;

use crate::display::demand_management::OperationsDisplay;

#[derive(Debug, Default)]
pub enum SupplyDrop {
    #[default]
    Idle,
    Scheduled {
        event_start: Time,
        remaining: Time,
    },
    AwaitingAuthorization,
    DockingInProgress {
        event_end: Time,
    },
}

#[derive(Debug, Default)]
pub struct Operations {
    pub offline: bool,
    pub supply_drop: SupplyDrop,
    pub power_level: Power,
}

pub struct TickResult {
    pub power_consumed: Power,
    pub docking_completed: bool,
}

impl Operations {
    /// Ticks the state.
    /// Returns TickResult indicating power consumed and if docking completed.
    pub fn tick(&mut self, context: &TickContext) -> TickResult {
        let mut docking_completed = false;
        let mut current_power_consumption = Power::ZERO;

        if !self.offline {
            current_power_consumption = context.game_vars.operations_base_power_demand;
        }

        match &mut self.supply_drop {
            SupplyDrop::Scheduled { event_start, remaining } => {
                if context.mission_time >= *event_start {
                    self.supply_drop = SupplyDrop::AwaitingAuthorization;
                } else {
                    *remaining = *event_start - context.mission_time;
                }
            }
            SupplyDrop::DockingInProgress { event_end } => {
                if !self.offline {
                    current_power_consumption += context.game_vars.operations_docking_spike_power;
                    if context.mission_time >= *event_end {
                        self.supply_drop = SupplyDrop::Idle;
                        docking_completed = true;
                    }
                } else {
                    self.supply_drop = SupplyDrop::DockingInProgress {
                        event_end: *event_end + context.tick_delta,
                    };
                }
            }
            _ => {}
        }

        self.power_level = current_power_consumption;

        TickResult {
            power_consumed: current_power_consumption,
            docking_completed,
        }
    }

    /// Attempts to authorize docking.
    /// Returns true if authorization was successful and docking started.
    pub fn authorize_docking(&mut self, context: &TickContext) -> bool {
        if !self.offline {
            if let SupplyDrop::AwaitingAuthorization = self.supply_drop {
                self.supply_drop = SupplyDrop::DockingInProgress {
                    event_end: context.mission_time
                        + context.game_vars.supply_drop_docking_duration,
                };
                return true;
            }
        }
        false
    }

    pub fn set_online(&mut self, online: bool) {
        self.offline = !online;
    }

    pub fn display(&self) -> OperationsDisplay {
        let pending_docking_indicator = match self.supply_drop {
            SupplyDrop::AwaitingAuthorization => true,
            _ => false,
        };

        let next_supply_drop_timer = match &self.supply_drop {
            SupplyDrop::Scheduled { remaining, .. } => *remaining,
            _ => Time::ZERO,
        };

        OperationsDisplay {
            power_level: self.power_level,
            online_status: !self.offline,
            pending_docking_indicator,
            next_supply_drop_timer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_variables::GameVariables;
    use crate::test::assert_quantities_eq;
    use crate::tick_context::TickContext;
    use uom::si::{f32::Time, power::watt, time::second};

    #[test]
    fn test_operations_initial_state_idle_power_demand() {
        let mut ops = Operations::default();
        let context = TickContext::new_static(
            &GameVariables {
                operations_base_power_demand: Power::new::<watt>(10.0),
                ..Default::default()
            },
            0.0,
            1.0,
        );

        let result = ops.tick(&context);

        assert_quantities_eq(result.power_consumed, 10.0);
        assert!(!result.docking_completed);
        assert!(!ops.offline);
        assert!(matches!(ops.supply_drop, SupplyDrop::Idle));
    }

    #[test]
    fn test_operations_offline_power_demand() {
        let mut ops = Operations {
            offline: true,
            ..Default::default()
        };
        let context = TickContext::new_static(
            &GameVariables {
                operations_base_power_demand: Power::new::<watt>(10.0),
                ..Default::default()
            },
            0.0,
            1.0,
        );

        let result = ops.tick(&context);

        assert_quantities_eq(result.power_consumed, 0.0);
        assert!(!result.docking_completed);
        assert!(ops.offline);
    }

    #[test]
    fn test_supply_drop_scheduled_to_awaiting_authorization() {
        let mut ops = Operations {
            supply_drop: SupplyDrop::Scheduled {
                event_start: Time::new::<second>(100.0),
                remaining: Time::new::<second>(10.0),
            },
            ..Default::default()
        };
        let context = TickContext::new_static(&GameVariables::default(), 100.0, 1.0);

        ops.tick(&context);

        assert!(matches!(ops.supply_drop, SupplyDrop::AwaitingAuthorization));
    }

    #[test]
    fn test_supply_drop_docking_to_idle_on_completion() {
        let mut ops = Operations {
            supply_drop: SupplyDrop::DockingInProgress {
                event_end: Time::new::<second>(200.0),
            },
            ..Default::default()
        };
        let context = TickContext::new_static(&GameVariables::default(), 200.0, 1.0);

        let result = ops.tick(&context);

        assert!(result.docking_completed);
        assert!(matches!(ops.supply_drop, SupplyDrop::Idle));
    }

    #[test]
    fn test_authorize_docking_success() {
        let mut ops = Operations {
            supply_drop: SupplyDrop::AwaitingAuthorization,
            ..Default::default()
        };
        let context = TickContext::new_static(
            &GameVariables {
                supply_drop_docking_duration: Time::new::<second>(60.0),
                ..Default::default()
            },
            0.0,
            1.0,
        );

        let authorized = ops.authorize_docking(&context);

        assert!(authorized);
        assert!(matches!(
            ops.supply_drop,
            SupplyDrop::DockingInProgress { .. }
        ));
    }

    #[test]
    fn test_authorize_docking_fail_system_offline() {
        let mut ops = Operations {
            offline: true,
            supply_drop: SupplyDrop::AwaitingAuthorization,
            ..Default::default()
        };
        let context = TickContext::new_static(&GameVariables::default(), 0.0, 1.0);

        let authorized = ops.authorize_docking(&context);

        assert!(!authorized);
        assert!(matches!(ops.supply_drop, SupplyDrop::AwaitingAuthorization));
    }

    #[test]
    fn test_authorize_docking_fail_not_awaiting_authorization() {
        let mut ops = Operations {
            supply_drop: SupplyDrop::Idle,
            ..Default::default()
        };
        let context = TickContext::new_static(&GameVariables::default(), 0.0, 1.0);

        let authorized = ops.authorize_docking(&context);

        assert!(!authorized);
        assert!(matches!(ops.supply_drop, SupplyDrop::Idle));
    }

    #[test]
    fn test_docking_power_spike() {
        let mut ops = Operations {
            supply_drop: SupplyDrop::DockingInProgress {
                event_end: Time::new::<second>(10.0),
            },
            ..Default::default()
        };
        let context = TickContext::new_static(
            &GameVariables {
                operations_base_power_demand: Power::new::<watt>(10.0),
                operations_docking_spike_power: Power::new::<watt>(5.0),
                ..Default::default()
            },
            0.0,
            1.0,
        );

        let result = ops.tick(&context);
        assert_quantities_eq(result.power_consumed, 15.0);
    }
}
