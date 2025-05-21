use uom::si::f32::{Energy, Power, Time, Frequency, Ratio};
use uom::si::energy::kilowatt_hour;
use uom::si::power::watt;
use uom::si::time::{day, minute, second};
use uom::si::frequency::hertz;
use uom::si::ratio::percent;

use crate::ConstOne;

/// Game balancing variables as specified in README.md Table 1.
pub struct GameVariables {
    /// System Inertia Constant (seconds)
    pub system_inertia_h: Time,
    /// Nominal System Power Capacity (Power units)
    pub system_nominal_power_pnom: Power,
    /// Nominal System Frequency (Hertz)
    pub nominal_frequency: Frequency,
    /// Mission Timer - Scaling Factor
    pub mission_time_per_lunar_time: Ratio,
    /// Repair Duration
    pub repair_time: Time,
    /// Solar Array - Nominal Output (Power units)
    pub solar_nominal_output: Power,
    /// Battery Capacity (kWh)
    pub battery_capacity: Energy,
    /// Reactor - Nominal Output (Power units)
    pub reactor_nominal_output: Power,
    /// Reactor Max Coolant Capacity (percentage 0-100)
    pub reactor_max_coolant_percentage: Ratio,
    /// Reactor Coolant Refill Rate (percentage points per second)
    pub reactor_coolant_refill_rate: Ratio,
    /// Reactor Coolant Effectiveness Reduction Rate (% effectiveness loss per % reactor damage)
    pub coolant_effectiveness_reduction_rate: Ratio,
    /// Life Support - Colony Damage Repair Rate (percentage points per second)
    pub colony_damage_repair_rate: Ratio,
    /// Life Support - Colony Damage Increase Rate (Emergency) (percentage points per second)
    pub colony_damage_rate_emergency: Ratio,
    /// Life Support - Base Power Demand (Power units)
    pub life_support_base_power_demand: Power,
    /// Life Support - Power Demand Increase Rate (Power units per in-game day)
    pub life_support_power_demand_increase: Power,
    /// Comms - Power Demand (Power units)
    pub comms_power_demand: Power,
    /// Micrometeorite Damage (to unshielded Solar) (damage %)
    pub micrometeorite_damage_solar: Ratio,
    /// Lunar Quake Damage (to Reactor) (damage %)
    pub lunar_quake_damage_reactor: Ratio,
    /// Lunar Quake Damage (to Batteries) (damage %)
    pub lunar_quake_damage_battery: Ratio,
    /// Solar Flare Damage (to unshielded Solar Arrays) (damage %)
    pub solar_flare_damage_solar_array: Ratio,
    /// Solar Flare Power Spike Damage (to Batteries via Solar Arrays) (damage %)
    pub solar_flare_spike_damage_battery: Ratio,
    /// Operations - Base Power Demand (Power units)
    pub operations_base_power_demand: Power,
    /// Operations - Docking Power Spike Magnitude (Power units)
    pub operations_docking_spike_power: Power,
    /// Operations - Docking Power Spike Duration (seconds)
    pub operations_docking_spike_duration: Time,
    /// Supply Drop - Timer Interval (seconds)
    pub supply_drop_interval: Time,
    /// Supply Drop - Docking Duration (seconds)
    pub supply_drop_docking_duration: Time,
    pub boost_life_support_amount: Ratio,
    pub boost_battery_amount: Ratio,
    pub boost_coolant_amount: Ratio,
    pub boost_repair_amount: Ratio,
}

impl Default for GameVariables {
    fn default() -> Self {
        GameVariables {
            system_inertia_h: Time::new::<second>(10.0),
            system_nominal_power_pnom: Power::new::<watt>(1000.0),
            nominal_frequency: Frequency::new::<hertz>(50.0),
            mission_time_per_lunar_time: Time::new::<minute>(1.0) / Time::new::<day>(29.5),
            repair_time: Time::new::<second>(2.0),
            solar_nominal_output: Power::new::<watt>(100.0),
            battery_capacity: Energy::new::<kilowatt_hour>(200.0),
            reactor_nominal_output: Power::new::<watt>(500.0),
            reactor_max_coolant_percentage: Ratio::ONE,
            reactor_coolant_refill_rate: Ratio::new::<percent>(1.0),
            coolant_effectiveness_reduction_rate: Ratio::new::<percent>(0.5),
            colony_damage_repair_rate: Ratio::new::<percent>(0.1),
            colony_damage_rate_emergency: Ratio::new::<percent>(0.5),
            life_support_base_power_demand: Power::new::<watt>(100.0),
            life_support_power_demand_increase: Power::new::<watt>(5.0),
            comms_power_demand: Power::new::<watt>(20.0),
            micrometeorite_damage_solar: Ratio::new::<percent>(10.0),
            lunar_quake_damage_reactor: Ratio::new::<percent>(15.0),
            lunar_quake_damage_battery: Ratio::new::<percent>(10.0),
            solar_flare_damage_solar_array: Ratio::new::<percent>(20.0),
            solar_flare_spike_damage_battery: Ratio::new::<percent>(10.0),
            operations_base_power_demand: Power::new::<watt>(30.0),
            operations_docking_spike_power: Power::new::<watt>(100.0),
            operations_docking_spike_duration: Time::new::<second>(5.0),
            supply_drop_interval: Time::new::<second>(60.0),
            supply_drop_docking_duration: Time::new::<second>(10.0),
            boost_life_support_amount: Ratio::new::<percent>(10.0),
            boost_battery_amount: Ratio::new::<percent>(20.0),
            boost_coolant_amount: Ratio::new::<percent>(20.0),
            boost_repair_amount: Ratio::new::<percent>(10.0),
        }
    }
}
