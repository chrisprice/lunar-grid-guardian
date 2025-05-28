use std::marker::PhantomData;
use uom::si::energy::kilowatt_hour;
use uom::si::f32::{Energy, Frequency, Power, PowerRate, Ratio, Time};
use uom::si::frequency::hertz;
use uom::si::power::watt;
use uom::si::power_rate::watt_per_second;
use uom::si::ratio::percent;
use uom::si::time::{day, minute, second};

/// Game balancing variables as specified in README.md Table 1.
#[derive(Clone)]
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
    /// Reactor - Power Ramp Rate
    pub reactor_power_ramp_rate: PowerRate,
    /// Reactor Max Coolant Energy Capacity (e.g., kWh)
    pub reactor_max_coolant_energy_capacity: Energy,
    /// Reactor Coolant Recharge Rate (e.g., kW)
    pub reactor_coolant_recharge_rate: Power,
    /// Reactor Coolant Effectiveness Reduction Rate (% effectiveness loss per % reactor damage)
    pub coolant_effectiveness_reduction_rate: Ratio,
    /// Reactor Thermal Efficiency Factor (Ratio, e.g., 0.8 means 80% efficient, 20% waste heat)
    pub reactor_thermal_efficiency_factor: Ratio,
    /// Reactor Critical Thermal Energy (e.g., kWh)
    pub reactor_critical_thermal_energy: Energy,
    /// Life Support - Colony Damage Increase Rate (Emergency) (percentage points per second)
    pub colony_damage_rate_emergency: Ratio,
    /// Life Support - Base Power Demand (Power units)
    pub life_support_base_power_demand: Power,
    /// Life Support - Power Demand Increase Rate (ticks once per day))
    pub life_support_power_demand_increase: PowerRate,
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
    /// Supply Drop - Timer Interval (seconds)
    pub supply_drop_interval: Time,
    /// Supply Drop - Docking Duration (seconds)
    pub supply_drop_docking_duration: Time,
    pub boost_life_support_amount: Ratio,
    pub boost_battery_amount: Energy,
    pub boost_coolant_energy_capacity: Energy,
    pub boost_repair_amount: Ratio,
    /// Probability of a micrometeorite event occurring per tick.
    pub micrometeorite_event_probability: Ratio,
    /// Probability of a lunar quake event occurring per tick.
    pub lunar_quake_event_probability: Ratio,
    /// Probability of a solar flare event occurring per tick.
    pub solar_flare_event_probability: Ratio,
    /// Event Duration (seconds)
    pub event_duration: Time,
    /// Event Schedule Offset (seconds)
    pub event_schedule_offset: Time,
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
            reactor_power_ramp_rate: PowerRate::new::<watt_per_second>(10.0),
            reactor_max_coolant_energy_capacity: Energy::new::<kilowatt_hour>(50.0),
            reactor_coolant_recharge_rate: Power::new::<watt>(10000.0), // 10kW
            coolant_effectiveness_reduction_rate: Ratio::new::<percent>(0.5),
            reactor_thermal_efficiency_factor: Ratio::new::<percent>(80.0),
            reactor_critical_thermal_energy: Energy::new::<kilowatt_hour>(100.0),
            colony_damage_rate_emergency: Ratio::new::<percent>(0.5),
            life_support_base_power_demand: Power::new::<watt>(100.0),
            life_support_power_demand_increase: Power::new::<watt>(5.0) / Time::new::<day>(1.0),
            comms_power_demand: Power::new::<watt>(20.0),
            micrometeorite_damage_solar: Ratio::new::<percent>(10.0),
            lunar_quake_damage_reactor: Ratio::new::<percent>(15.0),
            lunar_quake_damage_battery: Ratio::new::<percent>(10.0),
            solar_flare_damage_solar_array: Ratio::new::<percent>(20.0),
            solar_flare_spike_damage_battery: Ratio::new::<percent>(10.0),
            operations_base_power_demand: Power::new::<watt>(30.0),
            operations_docking_spike_power: Power::new::<watt>(100.0),
            supply_drop_interval: Time::new::<second>(60.0),
            supply_drop_docking_duration: Time::new::<second>(10.0),
            boost_life_support_amount: Ratio::new::<percent>(25.0),
            boost_battery_amount: Energy::new::<kilowatt_hour>(50.0),
            boost_coolant_energy_capacity: Energy::new::<kilowatt_hour>(10.0),
            boost_repair_amount: Ratio::new::<percent>(50.0),
            micrometeorite_event_probability: Ratio::new::<percent>(1.0),
            lunar_quake_event_probability: Ratio::new::<percent>(1.0),
            solar_flare_event_probability: Ratio::new::<percent>(1.0),
            event_duration: Time {
                dimension: PhantomData,
                units: PhantomData,
                value: 3.0,
            },
            event_schedule_offset: Time {
                dimension: PhantomData,
                units: PhantomData,
                value: 10.0,
            },
        }
    }
}
