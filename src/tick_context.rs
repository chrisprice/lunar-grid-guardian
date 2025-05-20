use uom::si::f32::Time;
use crate::game_variables::GameVariables;

pub struct TickContext<'a> {
    pub game_vars: &'a GameVariables,
    pub mission_time: Time,
    pub tick_delta: Time,
}
