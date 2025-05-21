use crate::battery::Battery;
use crate::event_state::EventState;
use crate::game_variables::GameVariables;
use crate::generator::GeneratorState;
use crate::lunar_phase::{LUNAR_DAY_SECONDS, LunarPhase};
use crate::operations::OperationsState;
use crate::solar::SolarState;
use crate::tick_context::TickContext;
use uom::si::f32::{Frequency, Power, Ratio, Time};
use uom::si::frequency::hertz;
use uom::si::ratio::percent;
use uom::si::time::second;
use uom::ConstZero;

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

    // Damage status (0-100%, where 0% is no damage)
    pub colony_damage: Ratio, // 0-100%

    pub comms_online: bool,
    pub operations_online: bool,
    pub life_support_emergency: bool,

    // Solar specific state encapsulated
    pub solar: SolarState,

    // Battery specific state
    pub battery: Battery,

    // Reactor specific state (0-100%)
    pub reactor_state: GeneratorState,
    pub reactor_coolant: Ratio,
    pub reactor_power: Power,
    pub reactor_temperature: f32,

    // Operations/boosts
    pub boost_life_support: u32,
    pub boost_battery: u32,
    pub boost_coolant: u32,
    pub boost_repair: u32,

    // Event states
    pub micrometeorite_event: EventState,
    pub lunar_quake_event: EventState,
    pub solar_flare_event: EventState,

    // Supply drop and docking status
    pub supply_drop_flow_state: OperationsState,
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
            colony_damage: Ratio::ZERO,
            comms_online: true,
            operations_online: true,
            life_support_emergency: false,
            solar: SolarState::default(),
            battery: Battery::default(),
            reactor_state: GeneratorState::default(),
            reactor_coolant: Ratio::new::<percent>(100.0),
            reactor_power: Power::ZERO,
            reactor_temperature: 0.0,
            boost_life_support: 0,
            boost_battery: 0,
            boost_coolant: 0,
            boost_repair: 0,
            micrometeorite_event: EventState::Dormant,
            lunar_quake_event: EventState::Dormant,
            solar_flare_event: EventState::Dormant,
            supply_drop_flow_state: OperationsState::default(),
        }
    }

    /// Returns the current lunar phase and time in cycle, derived from mission time and scaling factor.
    pub fn lunar_phase_and_time(&self) -> LunarPhase {
        let lunar_seconds = self.mission_time.get::<second>() / self.game_vars.mission_time_scale_factor;
        let time_in_cycle = lunar_seconds % LUNAR_DAY_SECONDS;
        if time_in_cycle < (LUNAR_DAY_SECONDS / 2.0) {
            LunarPhase::Day {
                elapsed: time_in_cycle,
            }
        } else {
            LunarPhase::Night {
                elapsed: time_in_cycle,
            }
        }
    }

    /// Returns true if the game is over, based on colony damage or frequency deviation.
    pub fn is_game_over(&self) -> bool {
        self.colony_damage.get::<percent>() >= 100.0
            || (self.tick_frequency_hz() - self.game_vars.nominal_frequency)
                .abs()
                .get::<hertz>()
                > 0.5
    }

    /// Derives the next frequency_hz value based on the swing equation and current state.
    pub fn tick_frequency_hz(&self) -> Frequency {
        let power_imbalance = self.total_grid_demand - self.total_grid_supply;
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
    pub fn tick_operations(&mut self, context: &TickContext) {
        let docking_completed = self.supply_drop_flow_state.tick(context);

        if docking_completed {
            todo!("Award player with a random boost");
        }
    }

    pub fn tick(&mut self) {
        self.mission_time += Time::new::<second>(1.0);

        let context = &TickContext {
            game_vars: self.game_vars,
            mission_time: self.mission_time,
            tick_delta: self.mission_time - self.last_tick_time,
        };

        // Event state ticks
        self.micrometeorite_event.tick(context);
        self.lunar_quake_event.tick(context);
        self.solar_flare_event.tick(context);

        // Solar system tick (handles repair) and get power generation
        let lunar_phase = self.lunar_phase_and_time();
        let solar_power = self.solar.tick(&lunar_phase, context);

        // Reactor system tick (handles repair)
        self.reactor_state.tick(context);
        // TODO: Get reactor power output, e.g.,
        // let reactor_power_output = calculate_reactor_power(&self.reactor_state, self.reactor_power, game_vars);

        // Battery tick
        // Calculate power imbalance before battery acts
        let power_imbalance = self.total_grid_demand - self.total_grid_supply;
        let power_consumed_by_battery = self.battery.tick(context, power_imbalance);
        // If battery consumes power (charges), it increases demand.
        // If battery supplies power (discharges), it increases supply.
        if power_consumed_by_battery.value > 0.0 {
            self.total_grid_demand += power_consumed_by_battery;
        } else if power_consumed_by_battery.value < 0.0 {
            self.total_grid_supply += -power_consumed_by_battery; // Add the absolute value
        }
        self.frequency_hz = self.tick_frequency_hz();

        self.tick_operations(context);

        self.last_tick_time = self.mission_time;
    }
}
