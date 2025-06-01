use crate::ConstOne;
use crate::game_variables::GameVariables;
use crate::system::state::State;
use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::{Energy, Power, Ratio};

#[derive(Debug, Default)]
pub struct Reactor {
    core_thermal_energy: Energy,
    coolant_available: Energy,
    state: State,
    power_output: Power,
    target_power_output: Power,
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
        self.state.tick(context);

        let ramp_amount = context.game_vars.reactor_power_ramp_rate * context.tick_delta;
        self.power_output = match &self.state {
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
                    .apply(self.coolant_available)
                    .min(self.core_thermal_energy);

                self.core_thermal_energy =
                    (self.core_thermal_energy - coolant_used).max(Energy::ZERO);
                self.coolant_available = (self.coolant_available - coolant_used).max(Energy::ZERO);

                if self.core_thermal_energy >= context.game_vars.reactor_critical_thermal_energy {
                    self.state = State::Offline;
                    return Power::ZERO;
                }

                power_output
            }
            State::Offline | State::Repairing { .. } => {
                (self.power_output - ramp_amount).max(Power::ZERO)
            }
        };

        let coolant_available = self.coolant_available
            + context.game_vars.reactor_coolant_recharge_rate * context.tick_delta;
        self.coolant_available = coolant_available
            .min(context.game_vars.reactor_max_coolant_available)
            .max(Energy::ZERO);

        self.power_output
    }

    pub fn display(&self, game_vars: &GameVariables) -> crate::display::generation_control::Reactor {
        crate::display::generation_control::Reactor {
            power: self.power_output / game_vars.reactor_nominal_output,
            coolant: self.coolant_available / game_vars.reactor_max_coolant_available,
            damage: self.state.effective_damage(),
        }
    }

    pub fn boost(&mut self, game_vars: &GameVariables) {
        self.coolant_available = (self.coolant_available + game_vars.boost_coolant_amount)
            .min(game_vars.reactor_max_coolant_available);
    }

    pub fn damage(&mut self, amount: Ratio) {
        self.state.damage(amount);
    }

    pub fn repair_boost(&mut self, game_vars: &GameVariables) {
        self.state.repair_boost(game_vars);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::Damage;
    use crate::game_variables::GameVariables;
    use crate::test::assert_quantities_eq;
    use crate::tick_context::TickContext;
    use uom::si::energy::{joule, kilowatt_hour};
    use uom::si::f32::{Power, PowerRate, Time};
    use uom::si::power::kilowatt;
    use uom::si::power_rate::kilowatt_per_second;
    use uom::si::ratio::percent;
    use uom::si::time::second;

    #[test]
    fn test_set_target_power_output_online_state() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut reactor = Reactor {
            state: State::Online {
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
    fn test_set_target_power_output_offline_state() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut reactor = Reactor {
            state: State::Offline,
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_quantities_eq(reactor.target_power_output, 75_000.0);
    }

    #[test]
    fn test_set_target_power_output_repairing_state() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut reactor = Reactor {
            state: State::Repairing {
                event_end: Time::new::<second>(10.0),
            },
            target_power_output: Power::new::<kilowatt>(50.0),
            ..Default::default()
        };
        reactor.set_target_power_output(Power::new::<kilowatt>(75.0), tick_context.game_vars);
        assert_quantities_eq(reactor.target_power_output, 75_000.0);
    }

    #[test]
    fn test_tick_state_offline_ramps_down_power() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut reactor = Reactor {
            state: State::Offline,
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
    fn test_tick_state_repairing_ramps_down_power() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let mut tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        tick_context.mission_time = Time::new::<second>(0.0);
        let mut reactor = Reactor {
            state: State::Repairing {
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
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut reactor = Reactor {
            state: State::Online {
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
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut reactor = Reactor {
            state: State::Online {
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
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(20.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut reactor = Reactor {
            state: State::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(50.0),
            coolant_available: Energy::new::<kilowatt_hour>(10.0),
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

    #[test]
    fn test_coolant_consumed_when_online() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let mut tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let initial_coolant = Energy::new::<kilowatt_hour>(20.0);
        let mut reactor = Reactor {
            state: State::Online {
                damage: Damage::default(),
            },
            power_output: Power::new::<kilowatt>(50.0),
            target_power_output: Power::new::<kilowatt>(50.0),
            coolant_available: initial_coolant,
            core_thermal_energy: Energy::ZERO, // Changed from 1.0 kWh
            ..Default::default()
        };

        let mut game_vars = (*tick_context.game_vars).clone();
        game_vars.reactor_thermal_efficiency_factor = Ratio::new::<percent>(80.0);
        game_vars.reactor_coolant_recharge_rate = Power::ZERO;
        tick_context.game_vars = &game_vars;

        reactor.tick(&tick_context);
        let expected_coolant_consumed = Power::new::<kilowatt>(50.0)
            * (Ratio::ONE - game_vars.reactor_thermal_efficiency_factor)
            * tick_context.tick_delta;
        assert_quantities_eq(
            reactor.coolant_available,
            (initial_coolant - expected_coolant_consumed).value,
        );
    }

    #[test]
    fn test_coolant_recharges_over_time() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let mut tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let initial_coolant = Energy::new::<kilowatt_hour>(5.0);
        let mut reactor = Reactor {
            state: State::Offline,
            coolant_available: initial_coolant,
            ..Default::default()
        };

        let mut game_vars = (*tick_context.game_vars).clone();
        game_vars.reactor_coolant_recharge_rate = Power::new::<kilowatt>(1.0);
        game_vars.reactor_max_coolant_available = Energy::new::<kilowatt_hour>(10.0);
        tick_context.game_vars = &game_vars;

        reactor.tick(&tick_context);
        let expected_recharge_amount =
            game_vars.reactor_coolant_recharge_rate * tick_context.tick_delta;
        assert_quantities_eq(
            reactor.coolant_available,
            (initial_coolant + expected_recharge_amount).value,
        );
    }

    #[test]
    fn test_coolant_recharge_capped_by_max() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let mut tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let max_coolant = Energy::new::<kilowatt_hour>(10.0);
        let initial_coolant = max_coolant - Energy::new::<joule>(100.0);

        let mut reactor = Reactor {
            state: State::Offline,
            coolant_available: initial_coolant,
            ..Default::default()
        };

        let mut game_vars = (*tick_context.game_vars).clone();
        game_vars.reactor_coolant_recharge_rate = Power::new::<kilowatt>(2.0);
        game_vars.reactor_max_coolant_available = max_coolant;
        tick_context.game_vars = &game_vars;

        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.coolant_available, max_coolant.value);
    }

    #[test]
    fn test_boost_increases_coolant() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            boost_coolant_amount: Energy::new::<kilowatt_hour>(5.0),
            ..Default::default()
        };
        let initial_coolant = Energy::new::<kilowatt_hour>(2.0);
        let mut reactor = Reactor {
            coolant_available: initial_coolant,
            ..Default::default()
        };

        reactor.boost(&game_vars);
        assert_quantities_eq(
            reactor.coolant_available,
            (initial_coolant + game_vars.boost_coolant_amount).value,
        );
    }

    #[test]
    fn test_boost_is_capped_by_max_coolant() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            boost_coolant_amount: Energy::new::<kilowatt_hour>(5.0),
            reactor_max_coolant_available: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let initial_coolant = Energy::new::<kilowatt_hour>(8.0);
        let mut reactor = Reactor {
            coolant_available: initial_coolant,
            ..Default::default()
        };

        reactor.boost(&game_vars);
        assert_quantities_eq(
            reactor.coolant_available,
            game_vars.reactor_max_coolant_available.value,
        );
    }

    #[test]
    fn test_boost_does_not_exceed_max_coolant_when_at_max() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            boost_coolant_amount: Energy::new::<kilowatt_hour>(5.0),
            reactor_max_coolant_available: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let initial_coolant = Energy::new::<kilowatt_hour>(10.0);
        let mut reactor = Reactor {
            coolant_available: initial_coolant, // Start at max coolant
            ..Default::default()
        };

        reactor.boost(&game_vars);
        assert_quantities_eq(
            reactor.coolant_available,
            game_vars.reactor_max_coolant_available.value,
        );
    }

    #[test]
    fn test_coolant_does_not_change_when_offline_and_no_recharge() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let mut tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let initial_coolant = Energy::new::<kilowatt_hour>(10.0);
        let mut reactor = Reactor {
            state: State::Offline,
            coolant_available: initial_coolant,
            ..Default::default()
        };

        let mut game_vars = (*tick_context.game_vars).clone();
        game_vars.reactor_coolant_recharge_rate = Power::ZERO;
        tick_context.game_vars = &game_vars;

        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.coolant_available, initial_coolant.value);
    }

    #[test]
    fn test_coolant_does_not_change_when_repairing_and_no_recharge() {
        let game_vars = GameVariables {
            reactor_nominal_output: Power::new::<kilowatt>(100.0),
            reactor_power_ramp_rate: PowerRate::new::<kilowatt_per_second>(10.0),
            repair_time: Time::new::<second>(10.0),
            ..Default::default()
        };
        let mut tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let initial_coolant = Energy::new::<kilowatt_hour>(10.0);
        let mut reactor = Reactor {
            state: State::Repairing {
                event_end: Time::new::<second>(100.0),
            },
            coolant_available: initial_coolant,
            ..Default::default()
        };

        let mut game_vars = (*tick_context.game_vars).clone();
        game_vars.reactor_coolant_recharge_rate = Power::ZERO;
        tick_context.game_vars = &game_vars;

        reactor.tick(&tick_context);
        assert_quantities_eq(reactor.coolant_available, initial_coolant.value);
    }
}
