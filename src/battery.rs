use crate::game_variables::GameVariables;
use crate::generator::GeneratorState;
use uom::si::f32::Power;
use uom::si::power::watt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BatteryMode {
    Auto,
    Charge,
    Discharge,
}

#[derive(Debug)]
pub struct Battery {
    pub generator_state: GeneratorState,
    pub charge_percentage: f32, // 0.0 to 100.0
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
    /// * `mission_time_seconds` - The total mission time in seconds, for internal state ticking.
    /// * `game_vars` - Game balancing variables, including battery capacity.
    ///
    /// # Returns
    /// The amount of power consumed by the battery from the grid.
    /// Positive if charging, negative if discharging (supplying power).
    pub fn tick(
        &mut self,
        power_imbalance: Power,
        mission_time_seconds: u32,
        game_vars: &GameVariables,
    ) -> Power {
        self.generator_state.tick(mission_time_seconds);

        // Assuming dt_seconds is always 1.0
        let dt_hours = 1.0 / 3600.0;

        let GeneratorState::Online { damage_percentage } = self.generator_state else {
            return Power::new::<watt>(0.0);
        };
        let mut power_consumed = Power::new::<watt>(0.0);

        match self.mode {
            BatteryMode::Charge => {
                if self.charge_percentage < 100.0 {
                    let energy_needed_to_full_kwh =
                        ((100.0 - self.charge_percentage) / 100.0) * game_vars.battery_capacity_kwh;
                    power_consumed = Power::new::<watt>(energy_needed_to_full_kwh / dt_hours);
                    self.charge_percentage = 100.0;
                }
            }
            BatteryMode::Discharge => {
                if self.charge_percentage > 0.0 {
                    let energy_to_empty_kwh =
                        (self.charge_percentage / 100.0) * game_vars.battery_capacity_kwh;
                    power_consumed = Power::new::<watt>(-(energy_to_empty_kwh / dt_hours)); // Negative as supplying power
                    self.charge_percentage = 0.0;
                }
            }
            BatteryMode::Auto if damage_percentage == 0.0 => {
                if power_imbalance.value > 0.0 && self.charge_percentage > 0.0 {
                    // Grid needs power, battery has charge
                    let energy_needed_by_grid_kwh = power_imbalance.value * dt_hours;
                    let energy_in_battery_kwh =
                        (self.charge_percentage / 100.0) * game_vars.battery_capacity_kwh;

                    let energy_to_discharge_kwh =
                        energy_needed_by_grid_kwh.min(energy_in_battery_kwh);

                    if energy_to_discharge_kwh > 0.0 {
                        self.charge_percentage -=
                            (energy_to_discharge_kwh / game_vars.battery_capacity_kwh) * 100.0;
                        self.charge_percentage = self.charge_percentage.max(0.0); // Ensure not negative
                        power_consumed = Power::new::<watt>(-(energy_to_discharge_kwh / dt_hours));
                    }
                } else if power_imbalance.value < 0.0 && self.charge_percentage < 100.0 {
                    // Grid has surplus, battery can charge
                    let surplus_energy_on_grid_kwh = -power_imbalance.value * dt_hours;
                    let remaining_capacity_kwh =
                        ((100.0 - self.charge_percentage) / 100.0) * game_vars.battery_capacity_kwh;

                    let energy_to_charge_kwh =
                        surplus_energy_on_grid_kwh.min(remaining_capacity_kwh);

                    if energy_to_charge_kwh > 0.0 {
                        self.charge_percentage +=
                            (energy_to_charge_kwh / game_vars.battery_capacity_kwh) * 100.0;
                        self.charge_percentage = self.charge_percentage.min(100.0); // Ensure not over 100
                        power_consumed = Power::new::<watt>(energy_to_charge_kwh / dt_hours);
                    }
                }
            }
            BatteryMode::Auto => {
                // If the battery is damaged, auto mode is disabled
                // and the battery will not charge or discharge.
            }
        }
        power_consumed
    }
}
