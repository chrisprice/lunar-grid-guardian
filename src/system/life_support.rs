use uom::ConstZero;
use uom::si::f32::{Power, Time};
use uom::si::time::{day, second};

use crate::game_variables::GameVariables;
use crate::system::state::State;
use crate::tick_context::TickContext;

/// Represents the state of the life support system.
/// Manages colony damage and calculates power consumption.
#[derive(Debug, Default)]
pub struct LifeSupport {
    /// Current system state (online, offline, repairing).
    pub system: State,
    /// Indicates if life support is operating in emergency restrictions mode.
    pub emergency_restrictions_active: bool,
}

impl LifeSupport {
    /// Activates or deactivates the emergency restrictions mode for life support.
    pub fn set_emergency_restrictions(&mut self, active: bool) {
        self.emergency_restrictions_active = active;
    }

    /// Processes a time step for the life support system.
    ///
    /// Returns the calculated power demand for the current tick.
    pub fn tick(&mut self, context: &TickContext) -> Power {
        self.system.tick(context);

        if self.emergency_restrictions_active {
            self.system.damage(
                context.game_vars.colony_damage_rate_emergency / Time::new::<second>(1.0)
                    * context.tick_delta,
            );
            Power::ZERO
        } else {
            if let State::Online { .. } = &self.system {
                self.system.repair(context.mission_time, context.game_vars);
            }

            context.game_vars.life_support_base_power_demand
                + context.game_vars.life_support_power_demand_increase
                    * context.mission_time.floor::<day>()
        }
    }

    pub fn boost(&mut self, game_vars: &GameVariables) {
        if let State::Online { damage } = &mut self.system {
            damage.repair(game_vars.boost_life_support_amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::Damage;
    use crate::game_variables::GameVariables;
    use crate::test::assert_quantities_eq;
    use uom::si::f32::{Power, Ratio, Time};
    use uom::si::power::watt;
    use uom::si::ratio::percent;
    use uom::si::time::day;

    #[test]
    fn test_life_support_initial_state() {
        let life_support = LifeSupport::default();
        let State::Online { damage } = life_support.system else {
            panic!("Expected System::Online");
        };
        assert_quantities_eq(damage.inner(), 0.0);
    }

    #[test]
    fn test_life_support_power_demand_initial() {
        let game_vars = GameVariables {
            life_support_base_power_demand: Power::new::<watt>(100.0),
            life_support_power_demand_increase: Power::new::<watt>(10.0) / Time::new::<day>(1.0),
            ..Default::default()
        };

        let mut life_support = LifeSupport::default();
        let context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let power_demand = life_support.tick(&context);

        assert_quantities_eq(power_demand, 100.0);
    }

    #[test]
    fn test_life_support_power_demand_increases_over_time() {
        let game_vars = GameVariables {
            life_support_base_power_demand: Power::new::<watt>(100.0),
            life_support_power_demand_increase: Power::new::<watt>(10.0) / Time::new::<day>(1.0),
            ..Default::default()
        };

        let mut life_support = LifeSupport::default();

        let context_almost_one_day = TickContext::new_static(&game_vars, 86400.0 - 1.0, 1.0);
        let power_demand_almost_one_day = life_support.tick(&context_almost_one_day);
        assert_quantities_eq(power_demand_almost_one_day, 100.0);

        let context_one_day = TickContext::new_static(&game_vars, 86400.0, 1.0);
        let power_demand_one_day = life_support.tick(&context_one_day);
        assert_quantities_eq(power_demand_one_day, 110.0);
    }

    #[test]
    fn test_life_support_emergency_restrictions_mode_power_and_damage() {
        let game_vars = GameVariables {
            life_support_base_power_demand: Power::new::<watt>(100.0),
            colony_damage_rate_emergency: Ratio::new::<percent>(0.1),
            ..Default::default()
        };
        let mut life_support = LifeSupport::default();
        life_support.set_emergency_restrictions(true);

        let context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let power_demand = life_support.tick(&context);

        assert_quantities_eq(power_demand, 0.0);
        if let State::Online { damage } = life_support.system {
            assert_quantities_eq(damage.inner(), 0.001);
        } else {
            panic!("Expected System::Online");
        }
    }

    #[test]
    fn test_life_support_emergency_restrictions_mode_stops_repair() {
        let game_vars = GameVariables {
            colony_damage_rate_emergency: Ratio::new::<percent>(0.1),
            repair_time: Time::new::<day>(1.0),
            ..Default::default()
        };
        let mut life_support = LifeSupport {
            system: State::Online {
                damage: Damage::new(Ratio::new::<percent>(10.0)),
            },
            emergency_restrictions_active: false,
        };

        life_support.set_emergency_restrictions(true);

        let context = TickContext::new_static(&game_vars, 0.0, 1.0);
        life_support.tick(&context);

        if let State::Online { damage, .. } = life_support.system {
            assert_quantities_eq(damage.inner(), 0.101);
        } else {
            panic!("Expected System::Online");
        }
    }
}
