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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::damage::Damage;
    use crate::game_variables::GameVariables;
    use crate::generator::GeneratorState;
    use crate::lunar_phase::LUNAR_PHASE_DURATION;
    use crate::tick_context::TickContext;
    use std::f32::consts::PI;
    use uom::si::f32::{Power, Ratio, Time};
    use uom::si::power::watt;
    use uom::si::ratio::ratio;
    use uom::si::time::second;

    fn solar_state_online(damage_value: f32, shields_active: bool) -> SolarState {
        SolarState {
            generator_state: GeneratorState::Online {
                damage: Damage::new(Ratio::new::<ratio>(damage_value)),
            },
            shields_active,
        }
    }

    fn solar_state_default_online_shields_off() -> SolarState {
        solar_state_online(0.0, false)
    }

    fn assert_power_approx_eq(
        actual_power: Power,
        expected_watts: f32,
        tolerance: f32,
        context_message: &str,
    ) {
        let actual_watts = actual_power.get::<watt>();
        assert!(
            (actual_watts - expected_watts).abs() < tolerance,
            "{}: Expected power ~{:.6} W, but got {:.6} W",
            context_message,
            expected_watts,
            actual_watts
        );
    }

    fn default_game_vars(
        solar_nominal_output_watts: f32,
        mission_time_lunar_time_ratio_val: f32,
    ) -> GameVariables {
        let mut gv = GameVariables::default();
        gv.solar_nominal_output = Power::new::<watt>(solar_nominal_output_watts);
        gv.mission_time_per_lunar_time = Ratio::new::<ratio>(mission_time_lunar_time_ratio_val);
        gv
    }

    fn create_day_tick_context<'a>(
        game_vars: &'a GameVariables,
        elapsed_ratio_target: f32,
    ) -> TickContext<'a> {
        let lunar_elapsed_target_secs = elapsed_ratio_target * LUNAR_PHASE_DURATION.value;
        let mission_time_secs =
            lunar_elapsed_target_secs * game_vars.mission_time_per_lunar_time.get::<ratio>();

        TickContext {
            mission_time: Time::new::<second>(mission_time_secs),
            game_vars,
            tick_delta: Time::new::<second>(1.0),
        }
    }

    fn create_night_tick_context<'a>(
        game_vars: &'a GameVariables,
        elapsed_ratio_in_night_target: f32,
    ) -> TickContext<'a> {
        let lunar_time_into_night_phase_secs =
            elapsed_ratio_in_night_target * LUNAR_PHASE_DURATION.value;
        let total_lunar_elapsed_secs =
            LUNAR_PHASE_DURATION.value + lunar_time_into_night_phase_secs;
        let mission_time_secs =
            total_lunar_elapsed_secs * game_vars.mission_time_per_lunar_time.get::<ratio>();

        TickContext {
            mission_time: Time::new::<second>(mission_time_secs),
            game_vars,
            tick_delta: Time::new::<second>(1.0),
        }
    }

    const NOMINAL_SOLAR_OUTPUT: f32 = 1000.0;
    const MISSION_TIME_PER_LUNAR_TIME_RATIO: f32 = 1.0;

    #[test]
    fn test_solar_power_at_sunrise() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.0);
        let mut solar_state = solar_state_default_online_shields_off();
        let power = solar_state.tick(&context);
        assert_power_approx_eq(power, 0.0, 1e-6, "Power at sunrise");
    }

    #[test]
    fn test_solar_power_at_midday() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.5);
        let mut solar_state = solar_state_default_online_shields_off();
        let power = solar_state.tick(&context);
        let expected_power = NOMINAL_SOLAR_OUTPUT * (0.5 * PI).sin();
        assert_power_approx_eq(power, expected_power, 1e-6, "Power at midday");
    }

    #[test]
    fn test_solar_power_at_sunset() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 1.0);
        let mut solar_state = solar_state_default_online_shields_off();
        let power = solar_state.tick(&context);
        assert_power_approx_eq(power, 0.0, 1e-6, "Power at sunset");
    }

    #[test]
    fn test_solar_power_just_before_sunset() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.999);
        let mut solar_state = solar_state_default_online_shields_off();
        let power = solar_state.tick(&context);
        let expected_intensity = (0.999 * PI).sin();
        let expected_power = NOMINAL_SOLAR_OUTPUT * expected_intensity;
        assert_power_approx_eq(power, expected_power, 1e-3, "Power just before sunset");
    }

    #[test]
    fn test_solar_power_at_quarter_day() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.25);
        let mut solar_state = solar_state_default_online_shields_off();
        let power = solar_state.tick(&context);
        let expected_intensity = (0.25 * PI).sin();
        let expected_power = NOMINAL_SOLAR_OUTPUT * expected_intensity;
        assert_power_approx_eq(power, expected_power, 1e-6, "Power at quarter day");
    }

    #[test]
    fn test_solar_power_at_three_quarter_day() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.75);
        let mut solar_state = solar_state_default_online_shields_off();
        let power = solar_state.tick(&context);
        let expected_intensity = (0.75 * PI).sin();
        let expected_power = NOMINAL_SOLAR_OUTPUT * expected_intensity;
        assert_power_approx_eq(power, expected_power, 1e-6, "Power at three quarter day");
    }

    #[test]
    fn test_solar_power_during_night() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_night_tick_context(&game_vars, 0.5);
        let mut solar_state = solar_state_default_online_shields_off();
        let power = solar_state.tick(&context);
        assert_power_approx_eq(power, 0.0, 1e-6, "Power during night");
    }

    #[test]
    fn test_solar_power_with_shields_active_during_day() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.5);
        let mut solar_state = solar_state_online(0.0, true);
        let power = solar_state.tick(&context);
        assert_power_approx_eq(power, 0.0, 1e-6, "Power with shields active during day");
    }

    #[test]
    fn test_solar_power_generator_offline() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.5);
        let mut solar_state = SolarState {
            generator_state: GeneratorState::Offline,
            shields_active: false,
        };
        let power = solar_state.tick(&context);
        assert_power_approx_eq(power, 0.0, 1e-6, "Power with generator offline");
    }

    #[test]
    fn test_solar_power_with_damage() {
        let game_vars = default_game_vars(NOMINAL_SOLAR_OUTPUT, MISSION_TIME_PER_LUNAR_TIME_RATIO);
        let context = create_day_tick_context(&game_vars, 0.5);
        let damage_percentage = 0.25;
        let mut solar_state = solar_state_online(damage_percentage, false);
        let power = solar_state.tick(&context);
        let intensity_factor = (0.5 * PI).sin();
        let expected_power_before_damage = NOMINAL_SOLAR_OUTPUT * intensity_factor;
        let expected_power_after_damage = expected_power_before_damage * (1.0 - damage_percentage);
        assert_power_approx_eq(
            power,
            expected_power_after_damage,
            1e-6,
            "Power with damage",
        );
    }
}
