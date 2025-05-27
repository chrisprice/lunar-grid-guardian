use crate::ConstOne;
use crate::system::system::SystemState;
use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::{Energy, Power, Ratio};

#[derive(Debug, Default)]
pub struct Reactor {
    pub core_thermal_energy: Energy,
    pub coolant_energy_absorption_capacity: Energy,
    pub generator_state: SystemState,
    pub power_output: Power,
    pub target_power_output: Power,
}

impl Reactor {
    pub fn set_target_power_output(
        &mut self,
        target_power: Power,
        game_vars: &crate::game_variables::GameVariables,
    ) {
        let nominal_power = game_vars.reactor_nominal_output;
        self.target_power_output = target_power.min(nominal_power).max(Power::ZERO);
    }

    pub fn tick(&mut self, context: &TickContext) -> Power {
        self.generator_state.tick(context);

        let ramp_amount = context.game_vars.reactor_power_ramp_rate * context.tick_delta;
        self.power_output = match self.generator_state {
            SystemState::Online { damage } => {
                let mut power_output = self.power_output;

                if power_output < self.target_power_output {
                    power_output = (power_output + ramp_amount).min(self.target_power_output);
                } else if power_output > self.target_power_output {
                    power_output = (power_output - ramp_amount).max(self.target_power_output);
                }

                let waste_heat_energy = power_output
                    * (Ratio::ONE - context.game_vars.reactor_thermal_efficiency_factor);
                self.core_thermal_energy += waste_heat_energy * context.tick_delta;

                let coolant_used: Energy = damage
                    .apply(self.coolant_energy_absorption_capacity)
                    .min(self.core_thermal_energy);

                self.core_thermal_energy =
                    (self.core_thermal_energy - coolant_used).max(Energy::ZERO);
                self.coolant_energy_absorption_capacity =
                    (self.coolant_energy_absorption_capacity - coolant_used).max(Energy::ZERO);

                if self.core_thermal_energy >= context.game_vars.reactor_critical_thermal_energy {
                    self.generator_state = SystemState::Offline;
                    return Power::ZERO;
                }

                power_output
            }
            SystemState::Offline | SystemState::Repairing { .. } => {
                (self.power_output - ramp_amount).max(Power::ZERO)
            }
        };

        let coolant_energy_absorption_capacity = self.coolant_energy_absorption_capacity
            + context.game_vars.reactor_coolant_recharge_rate * context.tick_delta;
        self.coolant_energy_absorption_capacity = coolant_energy_absorption_capacity
            .min(context.game_vars.reactor_max_coolant_energy_capacity)
            .max(Energy::ZERO);

        self.power_output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::Damage;
    use crate::game_variables::GameVariables;
    use crate::tick_context::TickContext;
    use uom::si::f32::{Power, PowerRate, Ratio, ThermodynamicTemperature, Time};
    use uom::si::power::kilowatt;
    use uom::si::power_rate::kilowatt_per_second;
    use uom::si::ratio::percent;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::time::second;

    const EPSILON: f32 = 1e-1;

    fn assert_power_eq(actual_power: Power, expected_power_kw: f32, message: &str) {
        let expected = Power::new::<kilowatt>(expected_power_kw);
        assert!(
            (actual_power.get::<kilowatt>() - expected.get::<kilowatt>()).abs() < EPSILON,
            "{}: Expected {} kW, got {} kW. Diff: {} kW",
            message,
            expected_power_kw,
            actual_power.get::<kilowatt>(),
            (actual_power - expected).abs().get::<kilowatt>()
        );
    }

    #[allow(dead_code)]
    fn assert_temperature_eq(
        actual_temp: ThermodynamicTemperature,
        expected_temp_c: f32,
        message: &str,
    ) {
        let expected = ThermodynamicTemperature::new::<degree_celsius>(expected_temp_c);
        assert!(
            (actual_temp.get::<degree_celsius>() - expected.get::<degree_celsius>()).abs()
                < EPSILON,
            "{}: Expected {} °C, got {} °C",
            message,
            expected_temp_c,
            actual_temp.get::<degree_celsius>()
        );
    }

    #[allow(dead_code)]
    fn assert_ratio_eq(actual_ratio: Ratio, expected_ratio_percent: f32, message: &str) {
        let expected = Ratio::new::<percent>(expected_ratio_percent);
        assert!(
            (actual_ratio.get::<percent>() - expected.get::<percent>()).abs() < EPSILON,
            "{}: Expected {} %, got {} %",
            message,
            expected_ratio_percent,
            actual_ratio.get::<percent>()
        );
    }

    fn setup_test_environment(
        tick_delta_s: f32,
        reactor_nominal_power_kw: f32,
        reactor_power_ramp_rate_kws: f32,
    ) -> TickContext<'static> {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(reactor_nominal_power_kw),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(
                reactor_power_ramp_rate_kws,
            ),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let game_vars_leak = Box::leak(Box::new(game_vars));

        TickContext {
            game_vars: game_vars_leak,
            tick_delta: Time::new::<second>(tick_delta_s),
            mission_time: Time::new::<second>(0.0),
        }
    }

    #[test]
    fn test_set_target_power_output_online_generator() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Online { damage: Damage::default() },
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_power_eq(
            reactor.target_power_output,
            75.0,
            "Target power set within limits",
        );

        reactor.set_target_power_output(Power::new::<kilowatt>(120.0), tick_context.game_vars);
        assert_power_eq(
            reactor.target_power_output,
            100.0,
            "Target power capped by nominal_output",
        );

        reactor.set_target_power_output(Power::new::<kilowatt>(-10.0), tick_context.game_vars);
        assert_power_eq(
            reactor.target_power_output,
            0.0,
            "Target power floored at zero",
        );
    }

    #[test]
    fn test_set_target_power_output_offline_generator() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Offline,
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_power_eq(
            reactor.target_power_output,
            75.0,
            "Target power can be set even if generator offline",
        );
    }

    #[test]
    fn test_set_target_power_output_repairing_generator() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Repairing {
                event_end: Time::new::<second>(10.0),
            },
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_power_eq(
            reactor.target_power_output,
            75.0,
            "Target power should be set even if generator repairing",
        );
    }

    #[test]
    fn test_tick_generator_offline_state_ramps_down_power() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Offline,
            power_output: Power::new::<kilowatt>(50.0),
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };

        let power_supplied_tick1 = reactor.tick(&tick_context);
        assert_power_eq(
            power_supplied_tick1,
            40.0,
            "Power supplied tick 1 (ramping down due to Offline state)",
        );
        assert_power_eq(reactor.power_output, 40.0, "Reactor power output tick 1");

        for _ in 0..4 {
            reactor.tick(&tick_context);
        }
        let power_supplied_tick5 = reactor.tick(&tick_context);
        assert_power_eq(
            power_supplied_tick5,
            0.0,
            "Power supplied tick 5 (should be zero)",
        );
    }

    #[test]
    fn test_tick_generator_repairing_state_ramps_down_power() {
        let mut tick_context = setup_test_environment(1.0, 100.0, 10.0);
        tick_context.mission_time = Time::new::<second>(0.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Repairing {
                event_end: Time::new::<second>(100.0),
            },
            power_output: Power::new::<kilowatt>(30.0),
            ..Default::default()
        };

        tick_context.mission_time = Time::new::<second>(1.0);
        let _ = reactor.tick(&tick_context);
        assert_power_eq(
            reactor.power_output,
            20.0,
            "Repairing state ramp down tick 1",
        );

        tick_context.mission_time = Time::new::<second>(2.0);
        let _ = reactor.tick(&tick_context);
        assert_power_eq(
            reactor.power_output,
            10.0,
            "Repairing state ramp down tick 2",
        );

        tick_context.mission_time = Time::new::<second>(3.0);
        let _ = reactor.tick(&tick_context);
        assert_power_eq(
            reactor.power_output,
            0.0,
            "Repairing state ramp down tick 3",
        );
    }

    #[test]
    fn test_tick_online_mode_ramps_up_to_target() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(30.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(70.0), tick_context.game_vars);

        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 40.0, "Ramp up tick 1");
        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 50.0, "Ramp up tick 2");
        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 60.0, "Ramp up tick 3");
        reactor.tick(&tick_context);
        assert_power_eq(
            reactor.power_output,
            70.0,
            "Ramp up tick 4 (reached target)",
        );
        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 70.0, "Ramp up tick 5 (at target)");
    }

    #[test]
    fn test_tick_online_mode_ramps_down_to_target() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(80.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(50.0), tick_context.game_vars);

        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 70.0, "Ramp down tick 1");
        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 60.0, "Ramp down tick 2");
        reactor.tick(&tick_context);
        assert_power_eq(
            reactor.power_output,
            50.0,
            "Ramp down tick 3 (reached target)",
        );
        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 50.0, "Ramp down tick 4 (at target)");
    }

    #[test]
    fn test_tick_online_mode_target_zero_ramps_down() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator_state: SystemState::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(25.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(0.0), tick_context.game_vars);

        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 15.0, "Ramp to zero tick 1");
        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 5.0, "Ramp to zero tick 2");
        reactor.tick(&tick_context);
        assert_power_eq(
            reactor.power_output,
            0.0,
            "Ramp to zero tick 3 (reached zero)",
        );
        reactor.tick(&tick_context);
        assert_power_eq(reactor.power_output, 0.0, "Ramp to zero tick 4 (at zero)");
    }
}
