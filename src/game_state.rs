use crate::battery::Battery;
use crate::event_state::EventState;
use crate::game_variables::GameVariables;
use crate::generator::GeneratorState;
use crate::lunar_phase::{LUNAR_DAY_SECONDS, LunarPhase};
use crate::operations::OperationsState;
use crate::power::Power; // Added import
use crate::solar::SolarState;

pub struct GameState {
    /// Real time in seconds since the start of the mission.
    pub mission_time_seconds: u32,
    /// Last tick time in seconds since the start of the mission.
    pub last_tick_time_seconds: u32,

    // Grid metrics
    pub total_grid_supply: Power,
    pub total_grid_demand: Power,
    pub frequency_hz: f32,

    // Health status (0-100%)
    pub colony_health: f32, // 0-100%

    pub comms_online: bool,
    pub operations_online: bool,
    pub life_support_emergency: bool,

    // Solar specific state encapsulated
    pub solar: SolarState,

    // Battery specific state
    pub battery: Battery,

    // Reactor specific state (0-100%)
    pub reactor_state: GeneratorState,
    pub reactor_coolant: f32,
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

impl GameState {
    pub fn new() -> Self {
        GameState {
            mission_time_seconds: 0,
            last_tick_time_seconds: 0,
            total_grid_supply: Power::new(0.0),
            total_grid_demand: Power::new(0.0),
            frequency_hz: 50.0,
            colony_health: 100.0,
            solar: SolarState::new(), // Updated to use new SolarState
            battery: Battery::new(),
            reactor_state: GeneratorState::new(),
            reactor_coolant: 100.0,
            comms_online: true,
            operations_online: true,
            life_support_emergency: false,
            reactor_power: Power::new(50.0),
            reactor_temperature: 25.0,
            boost_life_support: 0,
            boost_battery: 0,
            boost_coolant: 0,
            boost_repair: 0,
            micrometeorite_event: EventState::Dormant,
            lunar_quake_event: EventState::Dormant,
            solar_flare_event: EventState::Dormant,
            supply_drop_flow_state: OperationsState::new(),
        }
    }

    /// Returns the current lunar phase and time in cycle, derived from mission time and scaling factor.
    pub fn lunar_phase_and_time(&self, game_vars: &GameVariables) -> LunarPhase {
        let lunar_seconds = self.mission_time_seconds as f32 / game_vars.mission_time_scale_factor;
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

    /// Returns true if the game is over, based on colony health or frequency deviation.
    pub fn is_game_over(&self, game_vars: &GameVariables) -> bool {
        // Game Over if colony health reaches 0% or frequency deviates by more than ±0.5Hz from 50Hz
        self.colony_health <= 0.0 || (self.tick_frequency_hz(game_vars) - 50.0).abs() > 0.5
    }

    /// Derives the next frequency_hz value based on the swing equation and current state.
    pub fn tick_frequency_hz(&self, game_vars: &GameVariables) -> f32 {
        // Based on README.md:
        // d/dt(Δf) = ΔP / (2H * Pnom / f0)
        // Δf = frequency deviation from 50Hz
        // ΔP = total_grid_supply - total_grid_demand
        // H = system_inertia_h
        // Pnom = system_nominal_power_pnom
        // f0 = 50.0

        let delta_p = self.total_grid_supply - self.total_grid_demand;
        let h = game_vars.system_inertia_h;
        let pnom = game_vars.system_nominal_power_pnom;
        let f0 = 50.0;

        // d/dt(Δf)
        // Ensure pnom is not zero to avoid division by zero if it can be.
        // The Power/Power division handles internal zero check for rhs.0
        let rocof = if pnom.0 == 0.0 && delta_p.0 != 0.0 {
             // If there's an imbalance but no nominal power, frequency change is undefined or infinite.
             // This case should be handled based on game design (e.g., rapid collapse).
             // For now, let's assume a large change if delta_p is non-zero.
            if delta_p.0 > 0.0 { f32::INFINITY } else { f32::NEG_INFINITY }
        } else if pnom.0 == 0.0 && delta_p.0 == 0.0 {
            0.0 // No imbalance, no nominal power, no change.
        } else {
            delta_p / (2.0 * h * (pnom / f0))
        };


        // New frequency = current + (rate of change * tick)
        self.frequency_hz + rocof * (self.mission_time_seconds - self.last_tick_time_seconds) as f32
    }
    pub fn tick_operations(&mut self) {
        let docking_completed = self.supply_drop_flow_state.tick(self.mission_time_seconds);

        if docking_completed {
            todo!("Award player with a random boost");
        }
    }

    pub fn tick(&mut self, game_vars: &GameVariables) {
        self.mission_time_seconds += 1;

        // Event state ticks
        self.micrometeorite_event.tick(self.mission_time_seconds);
        self.lunar_quake_event.tick(self.mission_time_seconds);
        self.solar_flare_event.tick(self.mission_time_seconds);

        // Solar system tick (handles repair) and get power generation
        let lunar_phase = self.lunar_phase_and_time(game_vars);
        let solar_power =
            self.solar
                .tick(self.mission_time_seconds, &lunar_phase, game_vars);

        // Reactor system tick (handles repair)
        self.reactor_state.tick(self.mission_time_seconds);
        // TODO: Get reactor power output, e.g.,
        // let reactor_power_output = calculate_reactor_power(&self.reactor_state, self.reactor_power, game_vars);

        // Battery tick
        // Calculate power imbalance before battery acts
        let power_imbalance = self.total_grid_demand - self.total_grid_supply;
        let power_consumed_by_battery =
            self.battery
                .tick(power_imbalance, self.mission_time_seconds, game_vars);
        // If battery consumes power (charges), it increases demand.
        // If battery supplies power (discharges), it increases supply.
        if power_consumed_by_battery.0 > 0.0 {
            self.total_grid_demand += power_consumed_by_battery;
        } else if power_consumed_by_battery.0 < 0.0 {
            self.total_grid_supply += -power_consumed_by_battery;
        }
        self.frequency_hz = self.tick_frequency_hz(game_vars);

        self.tick_operations();

        self.last_tick_time_seconds = self.mission_time_seconds;
    }
}
