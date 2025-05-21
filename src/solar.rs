use crate::generator::GeneratorState;
use crate::lunar_phase::LunarPhase;
use crate::tick_context::TickContext;
use std::f32::consts::PI;
use uom::ConstZero;
use uom::si::f32::Power;
use uom::si::ratio::ratio;

#[derive(Debug, Default)]
pub struct SolarState {
    pub generator_state: GeneratorState,
    pub shields_active: bool,
}

impl SolarState {
    /// Ticks the solar state.
    /// Returns the amount of power generated.
    pub fn tick(&mut self, context: &TickContext) -> Power {
        self.generator_state.tick(context);

        let GeneratorState::Online { damage } = self.generator_state else {
            return Power::ZERO;
        };

        let lunar_phase = LunarPhase::from_tick_context(context);

        match lunar_phase {
            LunarPhase::Day { .. } if !self.shields_active => {
                let intensity_factor = (lunar_phase.elapsed_ratio().get::<ratio>() * PI).sin();
                let current_potential_power =
                    context.game_vars.solar_nominal_output * intensity_factor;
                damage.apply(current_potential_power)
            }
            LunarPhase::Day { .. } | LunarPhase::Night { .. } => Power::ZERO,
        }
    }
}
