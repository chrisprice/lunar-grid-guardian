use crate::game_variables::GameVariables;
use crate::tick_context::TickContext;
use uom::ConstZero;
use uom::si::f32::Ratio;
use uom::si::f32::Time;
use uom::si::ratio::percent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneratorState {
    Online { damage_percentage: Ratio },
    Offline,
    Repairing { event_end: Time },
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorState {
    pub fn new() -> Self {
        Self::Online {
            damage_percentage: Ratio::ZERO,
        }
    }
    pub fn repair(self, current_mission_time: Time, game_vars: &GameVariables) -> Self {
        match self {
            Self::Repairing { .. } => self,
            Self::Online { damage_percentage } if damage_percentage <= Ratio::ZERO => {
                Self::Online { damage_percentage: Ratio::ZERO }
            }
            Self::Online { damage_percentage } => {
                let repair_duration =
                    damage_percentage.get::<percent>() * game_vars.repair_time_per_damage_unit;
                Self::Repairing {
                    event_end: current_mission_time + repair_duration,
                }
            }
            Self::Offline => {
                let initial_damage = Ratio::new::<percent>(100.0);
                let repair_duration =
                    initial_damage.get::<percent>() * game_vars.repair_time_per_damage_unit;
                Self::Repairing {
                    event_end: current_mission_time + repair_duration,
                }
            }
        }
    }
    pub fn damage(&mut self, amount: Ratio) {
        match self {
            Self::Online { damage_percentage } => {
                *damage_percentage += amount;
                if *damage_percentage >= Ratio::new::<percent>(100.0) {
                    *self = Self::Offline;
                }
            }
            Self::Offline | Self::Repairing { .. } => {}
        }
    }

    pub fn tick(&mut self, context: &TickContext) {
        if let Self::Repairing { event_end } = self {
            if context.mission_time >= *event_end {
                *self = Self::Online {
                    damage_percentage: Ratio::ZERO,
                };
            }
        }
    }
}
