use crate::generator::GeneratorState;
use crate::lunar_phase::LunarPhase;
use crate::tick_context::TickContext;
use uom::si::f32::Power;
use uom::si::ratio::percent;
use uom::ConstZero;

#[derive(Debug, Default)]
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
        lunar_phase: &LunarPhase,
        context: &TickContext,
    ) -> Power {
        self.generator_state.tick(context);

        let GeneratorState::Online { damage_percentage } = self.generator_state else {
            return Power::ZERO;
        };

        match lunar_phase {
            LunarPhase::Day { .. } if !self.shields_active => {
                context.game_vars.solar_nominal_output * (1.0 - (damage_percentage.get::<percent>() / 100.0))
            }
            LunarPhase::Day { .. } | LunarPhase::Night { .. } => Power::ZERO,
        }
    }
}
