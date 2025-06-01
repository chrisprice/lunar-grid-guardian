use crate::game_variables::GameVariables;
use crate::system::state::State;
use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::{Energy, Power, Ratio};

#[derive(Debug, Clone, Copy, Default)]
pub enum BatteryMode {
    #[default]
    Auto,
    Charge,
    Discharge,
}

#[derive(Debug, Clone, Copy)]
pub enum BatteryOperation {
    Charge,
    Discharge,
}

#[derive(Debug, Default)]
pub struct Battery {
    state: State,
    charge: Energy,
    mode: BatteryMode,
}

impl Battery {
    pub fn set_mode(&mut self, new_mode: BatteryMode) {
        self.mode = new_mode;
    }

    /// Processes a time step for the battery.
    ///
    /// # Arguments
    /// * `power_imbalance` - The current power imbalance on the grid (supply - demand).
    ///   Positive means supply > demand (surplus), negative means demand > supply (deficit).
    ///
    /// # Returns
    /// The amount of power consumed by the battery from the grid.
    /// Positive if charging, negative if discharging (supplying power).
    pub fn tick(&mut self, context: &TickContext, power_imbalance: Power) -> Power {
        self.state.tick(context);

        let operation = match self.mode {
            BatteryMode::Charge | BatteryMode::Auto if power_imbalance > Power::ZERO => {
                Some(BatteryOperation::Charge)
            }
            BatteryMode::Discharge | BatteryMode::Auto if power_imbalance < Power::ZERO => {
                Some(BatteryOperation::Discharge)
            }
            _ => None,
        };

        let State::Online { damage } = &self.state else {
            return Power::ZERO;
        };

        let effective_capacity: Energy = damage.apply(context.game_vars.battery_capacity);
        let max_transferable: Energy = context.tick_delta * power_imbalance.abs();

        let transfered: Energy = match operation {
            Some(BatteryOperation::Charge) => {
                let remaining_capacity: Energy = effective_capacity - self.charge;
                remaining_capacity.min(max_transferable)
            }
            Some(BatteryOperation::Discharge) => -self.charge.min(max_transferable),
            None => Energy::ZERO,
        };

        self.charge += transfered;

        transfered / context.tick_delta
    }

    pub fn display(
        &self,
        game_vars: &GameVariables,
    ) -> crate::display::generation_control::Battery {
        crate::display::generation_control::Battery {
            charge: self.charge / game_vars.battery_capacity,
            damage: self.state.effective_damage(),
        }
    }

    pub fn boost(&mut self, game_vars: &GameVariables) {
        if let State::Online { damage } = &self.state {
            let effective_capacity: Energy = damage.apply(game_vars.battery_capacity);
            let remaining_capacity: Energy = effective_capacity - self.charge;
            self.charge += remaining_capacity.min(game_vars.boost_battery_amount);
        }
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
    use crate::tick_context::TickContext;
    use uom::si::energy::kilowatt_hour;
    use uom::si::f32::{Energy, Power, Ratio};
    use uom::si::power::kilowatt;
    use uom::si::ratio::percent;

    const EPSILON: f32 = 1e-1;

    fn assert_energy_eq(actual_energy_kwh: Energy, expected_energy_val: f32, message: &str) {
        assert!(
            (actual_energy_kwh.get::<kilowatt_hour>() - expected_energy_val).abs() < EPSILON,
            "{}: Expected energy {} kWh but got {} kWh",
            message,
            expected_energy_val,
            actual_energy_kwh.get::<kilowatt_hour>()
        );
    }

    fn assert_power_eq(actual_power: Power, expected_power_kw: f32, message: &str) {
        assert!(
            (actual_power.get::<kilowatt>() - expected_power_kw).abs() < EPSILON,
            "{}: Expected power {} kW but got {} kW",
            message,
            expected_power_kw,
            actual_power.get::<kilowatt>()
        );
    }

    #[test]
    fn test_tick_generator_offline() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Offline,
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Auto,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0); // Surplus

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 0.0, "Power consumed");
        assert_energy_eq(battery.charge, 5.0, "Charge");
    }

    #[test]
    fn test_tick_charge_mode_with_surplus() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Charge,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0); // Surplus

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 1.0, "Power consumed");
        assert_energy_eq(battery.charge, 6.0, "Charge");
    }

    #[test]
    fn test_tick_charge_mode_reaches_full() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(9.5),
            mode: BatteryMode::Charge,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0); // Surplus

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 0.5, "Power consumed");
        assert_energy_eq(battery.charge, 10.0, "Charge");
    }

    #[test]
    fn test_tick_charge_mode_already_full() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(10.0),
            mode: BatteryMode::Charge,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0); // Surplus

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 0.0, "Power consumed");
        assert_energy_eq(battery.charge, 10.0, "Charge");
    }

    #[test]
    fn test_tick_charge_mode_with_deficit() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Charge,
        };
        let power_imbalance = Power::new::<kilowatt>(-1.0); // Deficit

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 0.0, "Power consumed");
        assert_energy_eq(battery.charge, 5.0, "Charge");
    }

    #[test]
    fn test_tick_charge_mode_with_damage() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::new(Ratio::new::<percent>(50.0)),
            },
            charge: Energy::new::<kilowatt_hour>(2.0),
            mode: BatteryMode::Charge,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0); // Surplus

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 1.0, "Power consumed");
        assert_energy_eq(battery.charge, 3.0, "Charge");
    }

    #[test]
    fn test_tick_discharge_mode_with_deficit() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Discharge,
        };
        let power_imbalance = Power::new::<kilowatt>(-1.0); // Deficit

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, -1.0, "Power consumed");
        assert_energy_eq(battery.charge, 4.0, "Charge");
    }

    #[test]
    fn test_tick_discharge_mode_reaches_empty() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(0.5),
            mode: BatteryMode::Discharge,
        };
        let power_imbalance = Power::new::<kilowatt>(-1.0); // Deficit

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, -0.5, "Power consumed");
        assert_energy_eq(battery.charge, 0.0, "Charge");
    }

    #[test]
    fn test_tick_discharge_mode_already_empty() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(0.0),
            mode: BatteryMode::Discharge,
        };
        let power_imbalance = Power::new::<kilowatt>(-1.0); // Deficit

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 0.0, "Power consumed");
        assert_energy_eq(battery.charge, 0.0, "Charge");
    }

    #[test]
    fn test_tick_discharge_mode_with_surplus() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Discharge,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0); // Surplus

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 0.0, "Power consumed");
        assert_energy_eq(battery.charge, 5.0, "Charge");
    }

    #[test]
    fn test_tick_discharge_mode_with_damage() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::new(Ratio::new::<percent>(50.0)),
            },
            charge: Energy::new::<kilowatt_hour>(6.0),
            mode: BatteryMode::Discharge,
        };
        let power_imbalance = Power::new::<kilowatt>(-1.0); // Deficit

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, -1.0, "Power consumed");
        assert_energy_eq(battery.charge, 5.0, "Charge");
    }

    #[test]
    fn test_tick_auto_mode_no_damage_surplus_charges() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Auto,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0); // Surplus

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 1.0, "Power consumed");
        assert_energy_eq(battery.charge, 6.0, "Charge");
    }

    #[test]
    fn test_tick_auto_mode_no_damage_deficit_discharges() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 3600.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Auto,
        };
        let power_imbalance = Power::new::<kilowatt>(-1.0);

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, -1.0, "Power consumed");
        assert_energy_eq(battery.charge, 4.0, "Charge");
    }

    #[test]
    fn test_tick_auto_mode_no_damage_balanced_no_action() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::default(),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Auto,
        };
        let power_imbalance = Power::new::<kilowatt>(0.0);

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 0.0, "Power consumed");
        assert_energy_eq(battery.charge, 5.0, "Charge");
    }

    #[test]
    fn test_tick_auto_mode_with_damage_surplus_charges_less_effectively() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::new(Ratio::new::<percent>(10.0)),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Auto,
        };
        let power_imbalance = Power::new::<kilowatt>(1.0);

        let power_consumed = battery.tick(&tick_context, power_imbalance);

        assert_power_eq(power_consumed, 1.0, "Power consumed");
        assert_energy_eq(battery.charge, 5.0 + 1.0 / 3600.0, "Charge");
    }

    #[test]
    fn test_tick_auto_mode_with_damage_deficit_discharges_normally() {
        let game_vars = GameVariables {
            battery_capacity: Energy::new::<kilowatt_hour>(10.0),
            ..Default::default()
        };
        let tick_context = TickContext::new_static(&game_vars, 0.0, 1.0);
        let mut battery = Battery {
            state: State::Online {
                damage: Damage::new(Ratio::new::<percent>(10.0)),
            },
            charge: Energy::new::<kilowatt_hour>(5.0),
            mode: BatteryMode::Auto,
        };
        let power_imbalance = Power::new::<kilowatt>(-1.0);

        let power_consumed = battery.tick(&tick_context, power_imbalance);
        assert_power_eq(power_consumed, -1.0, "Power consumed");
        assert_energy_eq(battery.charge, 5.0 - 1.0 / 3600.0, "Charge");
    }
}
