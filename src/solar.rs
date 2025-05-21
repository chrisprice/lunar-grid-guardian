use crate::generator::GeneratorState;
use crate::lunar_phase::LunarPhase;
use crate::tick_context::TickContext;
use uom::si::f32::Power;
use uom::ConstZero;

#[derive(Debug, Default)]
pub struct SolarState {
    pub generator_state: GeneratorState,
    pub shields_active: bool,
}

impl SolarState {
    /// Ticks the solar state.
    /// Returns the amount of power generated.
    pub fn tick(
        &mut self,
        lunar_phase: &LunarPhase,
        context: &TickContext,
    ) -> Power {
        self.generator_state.tick(context);

        let GeneratorState::Online { damage } = self.generator_state else {
            return Power::ZERO;
        };

        match lunar_phase {
            LunarPhase::Day { .. } if !self.shields_active => {
                damage.apply(context.game_vars.solar_nominal_output)
            }
            LunarPhase::Day { .. } | LunarPhase::Night { .. } => Power::ZERO,
        }
    }
}
