use crate::game_variables::GameVariables;
use crate::generator::GeneratorState;
use crate::lunar_phase::LunarPhase;
use uom::si::f32::{Power, Time};
use uom::si::power::watt;

pub struct SolarState {
    pub generator_state: GeneratorState,
    pub shields_active: bool,
}

impl SolarState {
    pub fn new() -> Self {
        SolarState {
            generator_state: GeneratorState::new(),
            shields_active: false,
        }
    }

    /// Ticks the solar state.
    /// Returns the amount of power generated.
    pub fn tick(
        &mut self,
        mission_time: Time,
        lunar_phase: &LunarPhase,
        game_vars: &GameVariables,
    ) -> Power {
        self.generator_state.tick(mission_time);

        let GeneratorState::Online { damage_percentage } = self.generator_state else {
            return Power::new::<watt>(0.0);
        };

        match lunar_phase {
            LunarPhase::Day { .. } if !self.shields_active => {
                game_vars.solar_nominal_output * (1.0 - (damage_percentage / 100.0))
            }
            LunarPhase::Day { .. } | LunarPhase::Night { .. } => Power::new::<watt>(0.0),
        }
    }
}
