use crate::game_variables::GameVariables;
use crate::generator::GeneratorState;
use crate::tick_context::TickContext;
use uom::si::energy::{kilowatt_hour, watt_hour};
use uom::si::f32::{Energy, Power, Time};
use uom::si::power::{kilowatt, watt};
use uom::si::time::hour;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BatteryMode {
    Auto,
    Charge,
    Discharge,
}

#[derive(Debug)]
pub struct Battery {
    pub generator_state: GeneratorState,
    pub charge_percentage: f32,
    pub mode: BatteryMode,
}

impl Battery {
    pub fn new() -> Self {
        Battery {
            generator_state: GeneratorState::new(),
            charge_percentage: 100.0,
            mode: BatteryMode::Auto,
        }
    }

    pub fn set_mode(&mut self, new_mode: BatteryMode) {
        self.mode = new_mode;
    }

    /// Processes a time step for the battery.
    ///
    /// # Arguments
    /// * `power_imbalance` - The current power imbalance on the grid (demand - supply).
    ///                          Positive means demand > supply, negative means supply > demand.
    /// * `mission_time` - The total mission time, for internal state ticking.
    /// * `game_vars` - Game balancing variables, including battery capacity.
    ///
    /// # Returns
    /// The amount of power consumed by the battery from the grid.
    /// Positive if charging, negative if discharging (supplying power).
    pub fn tick(&mut self, context: &TickContext, power_imbalance: Power) -> Power {
        self.generator_state.tick(context);

        let GeneratorState::Online { damage_percentage } = self.generator_state else {
            return Power::new::<watt>(0.0);
        };
        let mut power_consumed = Power::new::<watt>(0.0);

        match self.mode {
            BatteryMode::Charge => {
                let effective_capacity =
                    context.game_vars.battery_capacity * (1.0 - (damage_percentage / 100.0));
                let max_charge_as_power_quantity: Power = effective_capacity / context.tick_delta;

                let max_charge_power_watts =
                    Power::new::<watt>(max_charge_as_power_quantity.get::<kilowatt>() * 1000.0);
                let actual_charge_power = (-power_imbalance).min(max_charge_power_watts);

                if actual_charge_power > Power::new::<watt>(0.0) {
                    let energy_added: Energy = actual_charge_power * context.tick_delta;
                    let current_energy_kwh =
                        context.game_vars.battery_capacity * (self.charge_percentage / 100.0);

                    let energy_added_kwh =
                        Energy::new::<kilowatt_hour>(energy_added.get::<watt_hour>() / 1000.0);
                    let new_energy_kwh = current_energy_kwh + energy_added_kwh;

                    self.charge_percentage =
                        ((new_energy_kwh / context.game_vars.battery_capacity).value * 100.0)
                            .min(100.0);
                    power_consumed = actual_charge_power;
                }
            }
            BatteryMode::Discharge => {
                let current_energy_available =
                    context.game_vars.battery_capacity * (self.charge_percentage / 100.0);
                let effective_current_energy =
                    current_energy_available * (1.0 - (damage_percentage / 100.0));
                let max_discharge_as_power_quantity: Power =
                    effective_current_energy / context.tick_delta;

                let max_discharge_power_watts =
                    Power::new::<watt>(max_discharge_as_power_quantity.get::<kilowatt>() * 1000.0);
                let actual_discharge_power = power_imbalance.min(max_discharge_power_watts);

                if actual_discharge_power > Power::new::<watt>(0.0) {
                    let energy_removed: Energy = actual_discharge_power * context.tick_delta;
                    let current_energy_kwh =
                        context.game_vars.battery_capacity * (self.charge_percentage / 100.0);

                    let energy_removed_kwh =
                        Energy::new::<kilowatt_hour>(energy_removed.get::<watt_hour>() / 1000.0);
                    let new_energy_kwh = current_energy_kwh - energy_removed_kwh;

                    self.charge_percentage =
                        ((new_energy_kwh / context.game_vars.battery_capacity).value * 100.0)
                            .max(0.0);
                    power_consumed = -actual_discharge_power;
                }
            }
            BatteryMode::Auto if damage_percentage == 0.0 => {
                if power_imbalance < Power::new::<watt>(0.0) {
                    let charge_power = -power_imbalance;
                    let energy_added: Energy = charge_power * context.tick_delta;
                    let current_energy_kwh =
                        context.game_vars.battery_capacity * (self.charge_percentage / 100.0);

                    let energy_added_kwh =
                        Energy::new::<kilowatt_hour>(energy_added.get::<watt_hour>() / 1000.0);
                    let new_energy_kwh = current_energy_kwh + energy_added_kwh;

                    self.charge_percentage =
                        ((new_energy_kwh / context.game_vars.battery_capacity).value * 100.0)
                            .min(100.0);
                    power_consumed = charge_power;
                } else if power_imbalance > Power::new::<watt>(0.0) {
                }
            }
            BatteryMode::Auto => {
                power_consumed = Power::new::<watt>(0.0);
            }
        }
        power_consumed
    }
}
