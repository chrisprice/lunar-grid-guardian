use uom::ConstZero;
use uom::si::f32::Power;

use crate::system::system::System;
use crate::tick_context::TickContext;

/// Represents the state of the Communications system.
#[derive(Debug, Default, Clone, Copy)]
pub struct Communications {
    pub system: System,
}

impl Communications {
    /// Processes a time step for the Comms system.
    ///
    /// Returns the calculated power demand for the current tick.
    pub fn tick(&mut self, context: &TickContext) -> Power {
        self.system.tick(context);

        if let System::Online { .. } = self.system {
            context.game_vars.comms_power_demand
        } else {
            Power::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_variables::GameVariables;
    use uom::si::power::watt;

    fn create_tick_context<'a>(
        game_vars: &'a GameVariables,
        mission_time_seconds: f32,
        tick_delta_seconds: f32,
    ) -> TickContext<'a> {
        TickContext {
            game_vars,
            mission_time: uom::si::f32::Time::new::<uom::si::time::second>(mission_time_seconds),
            tick_delta: uom::si::f32::Time::new::<uom::si::time::second>(tick_delta_seconds),
        }
    }

    #[test]
    fn test_comms_initial_state_online_power_demand() {
        let mut comms = Communications::default();
        let mut game_vars = GameVariables::default();
        game_vars.comms_power_demand = Power::new::<watt>(50.0);
        let context = create_tick_context(&game_vars, 0.0, 1.0);

        let power_demand = comms.tick(&context);
        assert_eq!(power_demand.get::<watt>(), 50.0);
        assert!(matches!(comms.system, System::Online { .. }));
    }

    #[test]
    fn test_comms_offline_power_demand() {
        let mut comms = Communications {
            system: System::Offline,
        };
        let mut game_vars = GameVariables::default();
        game_vars.comms_power_demand = Power::new::<watt>(50.0);
        let context = create_tick_context(&game_vars, 0.0, 1.0);

        let power_demand = comms.tick(&context);
        assert_eq!(power_demand.get::<watt>(), 0.0);
    }
}
