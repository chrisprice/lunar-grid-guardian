use crate::ConstOne;
use crate::game_variables::GameVariables;
use crate::system::state::State;
use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::{Energy, Power, Ratio};

#[derive(Debug, Default)]
pub struct Reactor {
    pub core_thermal_energy: Energy,
    pub coolant_energy_absorption_capacity: Energy,
    pub generator: State,
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
        self.generator.tick(context);

        let ramp_amount = context.game_vars.reactor_power_ramp_rate * context.tick_delta;
        self.power_output = match self.generator {
            State::Online { damage } => {
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
                    self.generator = State::Offline;
                    return Power::ZERO;
                }

                power_output
            }
            State::Offline | State::Repairing { .. } => {
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

    pub fn boost(&mut self, game_vars: &GameVariables) {
        self.coolant_energy_absorption_capacity += game_vars.boost_coolant_energy_capacity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::Damage;
    use crate::game_variables::GameVariables;
    use crate::test::assert_quantities_eq;
    use crate::tick_context::TickContext;
    use uom::si::f32::{Power, PowerRate, Time};
    use uom::si::power::kilowatt;
    use uom::si::power_rate::kilowatt_per_second;
    use uom::si::time::second;

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
            generator: State::Online {
                damage: Damage::default(),
            },
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_quantities_eq(reactor.target_power_output, 75_000.0);

        reactor.set_target_power_output(Power::new::<kilowatt>(120.0), tick_context.game_vars);
        assert_quantities_eq(reactor.target_power_output, 100_000.0);

        reactor.set_target_power_output(Power::new::<kilowatt>(-10.0), tick_context.game_vars);
        assert_quantities_eq(reactor.target_power_output, 0.0);
    }

    #[test]
    fn test_set_target_power_output_offline_generator() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator: State::Offline,
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_quantities_eq(reactor.target_power_output, 75_000.0);
    }

    #[test]
    fn test_set_target_power_output_repairing_generator() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator: State::Repairing {
                event_end: Time::new::<second>(10.0),
            },
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_quantities_eq(reactor.target_power_output, 75_000.0);
    }

    #[test]
    fn test_tick_generator_offline_ramps_down_power() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator: State::Offline,
            power_output: Power::new::<kilowatt>(50.0),
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };

        let power_supplied_tick1 = reactor.tick(&tick_context);
        assert_quantities_eq(power_supplied_tick1, 40_000.0);
        assert_quantities_eq(reactor.power_output, 40_000.0);

        for _ in 0..4 {
            reactor.tick(&tick_context);
        }
        let power_supplied_tick5 = reactor.tick(&tick_context);
        assert_quantities_eq(power_supplied_tick5, 0.0);
    }

    #[test]
    fn test_tick_generator_repairing_ramps_down_power() {
        let mut tick_context = setup_test_environment(1.0, 100.0, 10.0);
        tick_context.mission_time = Time::new::<second>(0.0);
        let mut reactor = Reactor {
            generator: State::Repairing {
                event_end: Time::new::<second>(100.0),
            },
            power_output: Power::new::<kilowatt>(30.0),
            ..Default::default()
        };

        tick_context.mission_time = Time::new::<second>(1.0);
        let _ = reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 20_000.0);

        tick_context.mission_time = Time::new::<second>(2.0);
        let _ = reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 10_000.0);

        tick_context.mission_time = Time::new::<second>(3.0);
        let _ = reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 0.0);
    }

    #[test]
    fn test_tick_online_mode_ramps_up_to_target() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator: State::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(30.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(70.0), tick_context.game_vars);

        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 40_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 50_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 60_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 70_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 70_000.0);
    }

    #[test]
    fn test_tick_online_mode_ramps_down_to_target() {
        let tick_context = setup_test_environment(1.0, 100.0, 10.0);
        let mut reactor = Reactor {
            generator: State::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(80.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(50.0), tick_context.game_vars);

        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 70_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 60_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 50_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 50_000.0);
    }

    #[test]
    fn test_tick_online_mode_target_zero_ramps_down() {
        let tick_context = setup_test_environment(1.0, 100.0, 20.0);
        let mut reactor = Reactor {
            generator: State::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(0.0), tick_context.game_vars);

        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 30_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 10_000.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 0.0);
        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.power_output, 0.0);
    }
}
