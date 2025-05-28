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
    use crate::test::assert_quantities_eq;
    use uom::si::power::watt;

    #[test]
    fn test_comms_initial_state_online_power_demand() {
        let mut comms = Communications::default();
        let context = TickContext::new_static(
            &GameVariables {
                comms_power_demand: Power::new::<watt>(50.0),
                ..Default::default()
            },
            0.0,
            1.0,
        );

        let power_demand = comms.tick(&context);

        assert_quantities_eq(power_demand, 50.0);
        assert!(matches!(comms.system, System::Online { .. }));
    }

        #[test]
        fn test_comms_offline_power_demand() {
            let mut comms = Communications {
                system: System::Offline,
            };
            let context = TickContext::new_static(
                &GameVariables {
                    comms_power_demand: Power::new::<watt>(50.0),
                    ..Default::default()
                },
                0.0,
                1.0,
            );

            let power_demand = comms.tick(&context);

            assert_quantities_eq(power_demand, 0.0);
            assert!(matches!(comms.system, System::Offline));
        }
}
