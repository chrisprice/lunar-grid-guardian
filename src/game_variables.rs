use uom::si::f32::{Energy, Power, Time};
use uom::si::energy::kilowatt_hour;
use uom::si::power::watt;
use uom::si::time::second;

/// Game balancing variables as specified in README.md Table 1.
pub struct GameVariables {
    /// System Inertia Constant (seconds)
    pub system_inertia_h: Time,
    /// Nominal System Power Capacity (Power units)
    pub system_nominal_power_pnom: Power,
    /// Mission Timer - Scaling Factor (real seconds to lunar seconds)
    /// (1 real second = 1 lunar second / scaling factor)
    pub mission_time_scale_factor: f32,
    /// Repair Duration (seconds per % damage) - Consolidated for all repairable systems
    pub repair_time_per_damage_unit: Time,
    /// Solar Array - Nominal Output (Power units)
    pub solar_nominal_output: Power,
    /// Battery Capacity (kWh)
    pub battery_capacity: Energy,
    /// Reactor - Nominal Output (Power units)
    pub reactor_nominal_output: Power,
    /// Reactor Max Coolant Capacity (percentage 0-100)
    pub reactor_max_coolant_percentage: f32,
    /// Reactor Coolant Refill Rate (percentage points per second)
    pub reactor_coolant_refill_rate: f32,
    /// Reactor Coolant Effectiveness Reduction Rate (% effectiveness loss per % reactor damage)
    pub coolant_effectiveness_reduction_rate: f32,
    /// Life Support - Colony Health Rebuild Rate (percentage points per second)
    pub colony_status_rebuild_rate: f32,
    /// Life Support - Colony Health Drain Rate (Emergency) (percentage points per second)
    pub colony_status_drain_rate_emergency: f32,
    /// Life Support - Base Power Demand (Power units)
    pub life_support_base_power_demand: Power,
    /// Life Support - Power Demand Increase Rate (Power units per in-game day)
    pub life_support_power_demand_increase: Power,
    /// Comms - Power Demand (Power units)
    pub comms_power_demand: Power,
    /// Micrometeorite Damage (to unshielded Solar) (damage %)
    pub micrometeorite_damage_solar: f32,
    /// Lunar Quake Damage (to Reactor) (damage %)
    pub lunar_quake_damage_reactor: f32,
    /// Lunar Quake Damage (to Batteries) (damage %)
    pub lunar_quake_damage_battery: f32,
    /// Solar Flare Damage (to unshielded Solar Arrays) (damage %)
    pub solar_flare_damage_solar_array: f32,
    /// Solar Flare Power Spike Damage (to Batteries via Solar Arrays) (damage %)
    pub solar_flare_spike_damage_battery: f32,
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
    /// Boost amounts - these are not time-based, should remain f32 or be reviewed separately
    pub boost_life_support_amount: f32,
    pub boost_battery_amount: f32,
    pub boost_coolant_amount: f32,
    pub boost_repair_amount: f32,
}

impl Default for GameVariables {
    fn default() -> Self {
        GameVariables {
            system_inertia_h: Time::new::<second>(10.0),
            system_nominal_power_pnom: Power::new::<watt>(1000.0),
            mission_time_scale_factor: 1.0 / (29.5 * 24.0 * 60.0), // one lunar day every 1 minute
            repair_time_per_damage_unit: Time::new::<second>(2.0), // This value will be used for Solar, Battery, and Reactor repairs
            solar_nominal_output: Power::new::<watt>(100.0),
            battery_capacity: Energy::new::<kilowatt_hour>(200.0),
            reactor_nominal_output: Power::new::<watt>(500.0),
            reactor_max_coolant_percentage: 100.0,
            reactor_coolant_refill_rate: 1.0,
            coolant_effectiveness_reduction_rate: 0.5,
            colony_status_rebuild_rate: 0.1,
            colony_status_drain_rate_emergency: 0.5,
            life_support_base_power_demand: Power::new::<watt>(100.0),
            life_support_power_demand_increase: Power::new::<watt>(5.0),
            comms_power_demand: Power::new::<watt>(20.0),
            micrometeorite_damage_solar: 10.0,
            lunar_quake_damage_reactor: 15.0,
            lunar_quake_damage_battery: 10.0,
            solar_flare_damage_solar_array: 20.0,
            solar_flare_spike_damage_battery: 10.0,
            operations_base_power_demand: Power::new::<watt>(30.0),
            operations_docking_spike_power: Power::new::<watt>(100.0),
            operations_docking_spike_duration: Time::new::<second>(5.0),
            supply_drop_interval: Time::new::<second>(60.0),
            supply_drop_docking_duration: Time::new::<second>(10.0),
            boost_life_support_amount: 10.0,
            boost_battery_amount: 20.0,
            boost_coolant_amount: 20.0,
            boost_repair_amount: 10.0,
        }
    }
}
