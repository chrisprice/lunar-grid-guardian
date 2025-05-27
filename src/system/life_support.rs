use uom::ConstZero;
use uom::si::f32::{Power, Time};
use uom::si::time::{day, second};

use crate::system::system::System;
use crate::tick_context::TickContext;

/// Represents the state of the life support system.
/// Manages colony damage and calculates power consumption.
#[derive(Debug, Default)]
pub struct LifeSupport {
    /// Current system state (online, offline, repairing).
    pub system: System,
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
            if let System::Online { .. } = &mut self.system {
                self.system
                    .repair(context.mission_time, context.game_vars);
            }

            context.game_vars.life_support_base_power_demand
                + context.game_vars.life_support_power_demand_increase
                    * context.mission_time.floor::<day>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::Damage;
    use crate::game_variables::GameVariables;
    use uom::si::f32::{Ratio, Time};
    use uom::si::power::watt;
    use uom::si::ratio::percent;
    use uom::si::time::second;

    const EPSILON: f32 = 1e-6;

    fn assert_ratio_approx_eq(actual: Ratio, expected_percent: f32, message: &str) {
        let actual_val = actual.get::<percent>();
        assert!(
            (actual_val - expected_percent).abs() < EPSILON,
            "{}: Expected ratio ~{:.6}%, but got {:.6}%",
            message,
            expected_percent,
            actual_val
        );
    }

    fn assert_power_approx_eq(actual: Power, expected_watts: f32, message: &str) {
        let actual_val = actual.get::<watt>();
        assert!(
            (actual_val - expected_watts).abs() < EPSILON,
            "{}: Expected power ~{:.6} W, but got {:.6} W",
            message,
            expected_watts,
            actual_val
        );
    }

    fn create_tick_context<'a>(
        game_vars: &'a GameVariables,
        mission_time_seconds: f32,
        tick_delta_seconds: f32,
    ) -> TickContext<'a> {
        TickContext {
            game_vars,
            mission_time: Time::new::<second>(mission_time_seconds),
            tick_delta: Time::new::<second>(tick_delta_seconds),
        }
    }

    #[test]
    fn test_life_support_initial_state() {
        let life_support = LifeSupport::default();
        let System::Online { damage } = life_support.system else {
            panic!(
                "Expected System::Online, got {:?}",
                life_support.system
            );
        };
        assert_ratio_approx_eq(damage.inner(), 0.0, "Initial colony damage");
    }

    #[test]
    fn test_life_support_power_demand_initial() {
        let mut game_vars = GameVariables::default();
        game_vars.life_support_base_power_demand = Power::new::<watt>(100.0);
        game_vars.life_support_power_demand_increase =
            Power::new::<watt>(10.0) / Time::new::<day>(1.0);

        let mut life_support = LifeSupport::default();
        let context = create_tick_context(&game_vars, 0.0, 1.0);
        let power_demand = life_support.tick(&context);

        assert_power_approx_eq(power_demand, 100.0, "Initial power demand");
    }

    #[test]
    fn test_life_support_power_demand_increases_over_time() {
        let mut game_vars = GameVariables::default();
        game_vars.life_support_base_power_demand = Power::new::<watt>(100.0);
        game_vars.life_support_power_demand_increase =
            Power::new::<watt>(10.0) / Time::new::<day>(1.0);

        let mut life_support = LifeSupport::default();

        let context_almost_one_day = create_tick_context(&game_vars, 86400.0 - 1.0, 1.0);
        let power_demand_almost_one_day = life_support.tick(&context_almost_one_day);
        assert_power_approx_eq(
            power_demand_almost_one_day,
            100.0,
            "Power demand just before 1 day",
        );

        let context_one_day = create_tick_context(&game_vars, 86400.0, 1.0);
        let power_demand_one_day = life_support.tick(&context_one_day);
        assert_power_approx_eq(power_demand_one_day, 110.0, "Power demand at 1 day");

        let context_two_days = create_tick_context(&game_vars, 2.0 * 86400.0, 1.0);
        let power_demand_two_days = life_support.tick(&context_two_days);
        assert_power_approx_eq(power_demand_two_days, 120.0, "Power demand at 2 days");
    }

    #[test]
    fn test_life_support_emergency_restrictions_mode_power_and_damage() {
        let mut game_vars = GameVariables::default();
        game_vars.life_support_base_power_demand = Power::new::<watt>(100.0);
        game_vars.colony_damage_rate_emergency = Ratio::new::<percent>(5.0);

        let mut life_support = LifeSupport::default();
        life_support.set_emergency_restrictions(true);

        let context_tick1 = create_tick_context(&game_vars, 0.0, 1.0);
        let power_demand_tick1 = life_support.tick(&context_tick1);
        assert_power_approx_eq(
            power_demand_tick1,
            0.0,
            "Power demand in emergency mode (tick 1)",
        );
        let System::Online { damage } = life_support.system else {
            panic!(
                "Expected System::Online, got {:?}",
                life_support.system
            );
        };
        assert_ratio_approx_eq(
            damage.inner(),
            5.0,
            "Colony damage after 1s in emergency mode",
        );

        let context_tick2 = create_tick_context(&game_vars, 1.0, 2.0);
        let power_demand_tick2 = life_support.tick(&context_tick2);
        assert_power_approx_eq(
            power_demand_tick2,
            0.0,
            "Power demand in emergency mode (tick 2)",
        );
        let System::Online { damage } = life_support.system else {
            panic!(
                "Expected System::Online, got {:?}",
                life_support.system
            );
        };
        assert_ratio_approx_eq(
            damage.inner(),
            15.0,
            "Colony damage after 3s total in emergency mode (1s + 2s)",
        );
    }

    #[test]
    fn test_life_support_emergency_restrictions_mode_stops_repair() {
        let mut game_vars = GameVariables::default();
        game_vars.colony_damage_rate_emergency = Ratio::new::<percent>(0.5);

        let mut life_support = LifeSupport::default();
        life_support.system = System::Online {
            damage: Damage::new(Ratio::new::<percent>(10.0)),
        };
        life_support.set_emergency_restrictions(true);

        let context = create_tick_context(&game_vars, 0.0, 1.0);
        let _power_demand = life_support.tick(&context);

        let System::Online { damage } = life_support.system else {
            panic!(
                "Expected System::Online, got {:?}",
                life_support.system
            );
        };
        assert_ratio_approx_eq(
            damage.inner(),
            10.5,
            "Colony damage should increase due to emergency rate, not repair",
        );
    }
}
