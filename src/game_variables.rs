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
    /// Lunar Time to Mission Time Ratio
    pub mission_time_per_lunar_time: Ratio,
    /// Repair Time (seconds)
    pub repair_time: Time,
    /// Solar Nominal Output (Power units)
    pub solar_nominal_output: Power,
    /// Battery Capacity (Energy units)
    pub battery_capacity: Energy,
    /// Reactor Nominal Output (Power units)
    pub reactor_nominal_output: Power,
    /// Reactor Power Ramp Rate (PowerRate units)
    pub reactor_power_ramp_rate: PowerRate,
    /// Reactor Max Coolant Available (e.g., kWh)
    pub reactor_max_coolant_available: Energy,
    /// Reactor Coolant Recharge Rate (e.g., kW)
    pub reactor_coolant_recharge_rate: Power,
    /// Coolant Effectiveness Reduction Rate (percentage)
    pub coolant_effectiveness_reduction_rate: Ratio,
    /// Reactor Thermal Efficiency Factor (percentage)
    pub reactor_thermal_efficiency_factor: Ratio,
    /// Reactor Critical Thermal Energy (Energy units)
    pub reactor_critical_thermal_energy: Energy,
    /// Colony Damage Rate (emergency, percentage)
    pub colony_damage_rate_emergency: Ratio,
    /// Colony Damage Repair Rate (percentage)
    pub colony_damage_repair_rate: Ratio,
    /// Life Support Base Power Demand (Power units)
    pub life_support_base_power_demand: Power,
    /// Life Support Power Demand Increase (PowerRate units)
    pub life_support_power_demand_increase: PowerRate,
    /// Communications Power Demand (Power units)
    pub comms_power_demand: Power,
    /// Micrometeorite Damage to Solar Array (percentage)
    pub micrometeorite_damage_solar: Ratio,
    /// Lunar Quake Damage to Reactor (percentage)
    pub lunar_quake_damage_reactor: Ratio,
    /// Lunar Quake Damage to Battery (percentage)
    pub lunar_quake_damage_battery: Ratio,
    /// Solar Flare Damage to Solar Array (percentage)
    pub solar_flare_damage_solar_array: Ratio,
    /// Solar Flare Spike Damage to Battery (percentage)
    pub solar_flare_spike_damage_battery: Ratio,
    /// Operations Base Power Demand (Power units)
    pub operations_base_power_demand: Power,
    /// Operations Docking Spike Power (Power units)
    pub operations_docking_spike_power: Power,
    /// Supply Drop Interval (seconds)
    pub supply_drop_interval: Time,
    /// Supply Drop Docking Duration (seconds)
    pub supply_drop_docking_duration: Time,
    /// Boost to Life Support Amount (percentage)
    pub boost_life_support_amount: Ratio,
    /// Boost to Battery Amount (Energy units)
    pub boost_battery_amount: Energy,
    /// Boost to Coolant Energy Capacity (Energy units)
    pub boost_coolant_energy_capacity: Energy,
    /// Boost to Repair Amount (percentage)
    pub boost_repair_amount: Ratio,
    /// Micrometeorite Event Probability (percentage)
    pub micrometeorite_event_probability: Ratio,
    /// Lunar Quake Event Probability (percentage)
    pub lunar_quake_event_probability: Ratio,
    /// Solar Flare Event Probability (percentage)
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
            reactor_max_coolant_available: Energy::new::<kilowatt_hour>(50.0),
            reactor_coolant_recharge_rate: Power::new::<watt>(10000.0), // 10kW
            coolant_effectiveness_reduction_rate: Ratio::new::<percent>(0.5),
            reactor_thermal_efficiency_factor: Ratio::new::<percent>(80.0),
            reactor_critical_thermal_energy: Energy::new::<kilowatt_hour>(100.0),
            colony_damage_rate_emergency: Ratio::new::<percent>(0.5),
            colony_damage_repair_rate: Ratio::new::<percent>(0.1),
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
