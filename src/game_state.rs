use crate::event::Event;
use crate::game_variables::GameVariables;
use crate::system::battery::Battery;
use crate::system::communications::Communications;
use crate::system::life_support::LifeSupport;
use crate::system::operations::Operations;
use crate::system::reactor::Reactor;
use crate::system::solar::Solar;
use crate::system::system::System;
use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::{Frequency, Power, Time};
use uom::si::frequency::hertz;
use uom::si::time::second;

pub struct GameState<'a> {
    /// Game variables for the current game.
    pub game_vars: &'a GameVariables,

    /// Real time since the start of the mission.
    pub mission_time: Time,
    /// Last tick time since the start of the mission.
    pub last_tick_time: Time,

    // Grid metrics
    pub total_grid_supply: Power,
    pub total_grid_demand: Power,
    pub frequency_hz: Frequency,

    // Supply
    pub solar: Solar,
    pub battery: Battery,
    pub reactor: Reactor,

    // Demand
    pub life_support: LifeSupport,
    pub operations: Operations,
    pub comms: Communications,

    // Boosts
    pub boost_life_support: u32,
    pub boost_battery: u32,
    pub boost_coolant: u32,
    pub boost_repair: u32,

    // Events
    pub micrometeorite: Event,
    pub lunar_quake: Event,
    pub solar_flare: Event,
}

impl<'a> GameState<'a> {
    pub fn new(game_vars: &'a GameVariables) -> Self {
        GameState {
            game_vars,
            mission_time: Time::ZERO,
            last_tick_time: Time::ZERO,
            total_grid_supply: Power::ZERO,
            total_grid_demand: Power::ZERO,
            frequency_hz: game_vars.nominal_frequency,
            solar: Solar::default(),
            battery: Battery::default(),
            reactor: Reactor::default(),
            life_support: LifeSupport::default(),
            operations: Operations::default(),
            comms: Communications::default(),
            boost_life_support: 0,
            boost_battery: 0,
            boost_coolant: 0,
            boost_repair: 0,
            micrometeorite: Event::new(game_vars.micrometeorite_event_probability),
            lunar_quake: Event::new(game_vars.lunar_quake_event_probability),
            solar_flare: Event::new(game_vars.solar_flare_event_probability),
        }
    }

    /// Returns true if the game is over, based on colony damage or frequency deviation.
    pub fn is_game_over(&self) -> bool {
        let System::Online { .. } = &self.life_support.system else {
            return true;
        };
        (self.tick_frequency_hz() - self.game_vars.nominal_frequency)
            .abs()
            .get::<hertz>()
            > 0.5
    }

    /// Derives the next frequency_hz value based on the swing equation and current state.
    pub fn tick_frequency_hz(&self) -> Frequency {
        let power_imbalance = self.total_grid_supply - self.total_grid_demand;
        let delta_p = power_imbalance;
        let h = self.game_vars.system_inertia_h;
        let pnom = self.game_vars.system_nominal_power_pnom;
        let f0 = self.game_vars.nominal_frequency;

        // Rate of Change of Frequency (RoCoF)
        let rocof_value = if pnom.value == 0.0 || h.value == 0.0 || f0.value == 0.0 {
            0.0 // Avoid division by zero
        } else {
            (delta_p / (2.0 * h * (pnom / f0))).value
        };
        let rocof = Frequency::new::<hertz>(rocof_value);

        // New frequency = current + (rate of change * tick duration)
        // Ensure last_tick_time is not greater than mission_time to prevent negative duration
        let tick_duration_seconds = if self.mission_time > self.last_tick_time {
            (self.mission_time - self.last_tick_time).get::<second>()
        } else {
            0.0 // Or handle as an error/log, but for tick logic, 0 duration is safer
        };
        self.frequency_hz + rocof * tick_duration_seconds
    }

    pub fn tick(&mut self) {
        self.mission_time += Time::new::<second>(1.0);

        let context = &TickContext {
            game_vars: self.game_vars,
            mission_time: self.mission_time,
            tick_delta: self.mission_time - self.last_tick_time,
        };

        if self.micrometeorite.tick(context) {
            self.solar
                .damage(self.game_vars.micrometeorite_damage_solar);
        }
        if self.lunar_quake.tick(context) {
            self.battery
                .generator
                .damage(self.game_vars.lunar_quake_damage_battery);
            self.reactor
                .generator
                .damage(self.game_vars.lunar_quake_damage_reactor);
        }
        if self.solar_flare.tick(context) {
            self.battery
                .generator
                .damage(self.game_vars.solar_flare_spike_damage_battery);
            self.solar
                .damage(self.game_vars.solar_flare_damage_solar_array);
        }

        self.total_grid_demand = {
            let comms_power_demand = self.comms.tick(context);
            let life_support_power_demand = self.life_support.tick(context);

            let operations_result = self.operations.tick(context);
            if operations_result.docking_completed {
                self.increment_random_boost();
            }

            operations_result.power_consumed + life_support_power_demand + comms_power_demand
        };

        self.total_grid_supply = {
            let solar_power = self.solar.tick(context);
            let reactor_output = self.reactor.tick(context);

            solar_power + reactor_output
        };

        // Battery
        let power_imbalance = self.total_grid_supply - self.total_grid_demand;
        let power_consumed_by_battery = self.battery.tick(context, power_imbalance);
        if power_consumed_by_battery.value > 0.0 {
            self.total_grid_demand += power_consumed_by_battery;
        } else if power_consumed_by_battery.value < 0.0 {
            self.total_grid_supply += -power_consumed_by_battery; // Add the absolute value
        }
        self.frequency_hz = self.tick_frequency_hz();

        self.last_tick_time = self.mission_time;
    }

    fn increment_random_boost(&mut self) {
        let random_boost_type = self.mission_time.get::<second>() as u32 % 4;
        match random_boost_type {
            0 => self.boost_life_support += 1,
            1 => self.boost_battery += 1,
            2 => self.boost_coolant += 1,
            3 => self.boost_repair += 1,
            _ => panic!("Unexpected random boost type"),
        }
    }

    pub fn use_life_support_boost(&mut self) {
        if self.boost_life_support > 0 {
            self.boost_life_support -= 1;
            self.life_support.boost(self.game_vars);
        }
    }

    pub fn use_battery_boost(&mut self) {
        if self.boost_battery > 0 {
            self.boost_battery -= 1;
            self.battery.boost(self.game_vars);
        }
    }

    pub fn use_coolant_boost(&mut self) {
        if self.boost_coolant > 0 {
            self.boost_coolant -= 1;
            self.reactor.boost(self.game_vars);
        }
    }

    pub fn use_repair_boost(&mut self) {
        if self.boost_repair > 0 {
            self.boost_repair -= 1;
            self.battery.generator.boost(self.game_vars);
            self.operations.system.boost(self.game_vars);
            self.reactor.generator.boost(self.game_vars);
            self.solar.generator.boost(self.game_vars);
        }
    }
}
