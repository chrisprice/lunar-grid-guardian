use crate::game_variables::GameVariables;
use uom::si::f32::Time;
use uom::si::time::second;

pub struct TickContext<'a> {
    pub game_vars: &'a GameVariables,
    pub mission_time: Time,
    pub tick_delta: Time,
}

impl<'a> TickContext<'a> {
    pub fn new(game_vars: &'a GameVariables, mission_time_s: f32, tick_delta_s: f32) -> Self {
        Self {
            game_vars,
            mission_time: Time::new::<second>(mission_time_s),
            tick_delta: Time::new::<second>(tick_delta_s),
        }
    }

    #[cfg(test)]
    pub fn new_static(
        game_vars: &GameVariables,
        mission_time_s: f32,
        tick_delta_s: f32,
    ) -> TickContext<'static> {
        let static_game_vars = Box::leak(Box::new(game_vars.clone()));
        TickContext {
            game_vars: static_game_vars,
            mission_time: Time::new::<second>(mission_time_s),
            tick_delta: Time::new::<second>(tick_delta_s),
        }
    }
}
