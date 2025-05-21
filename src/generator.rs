use crate::Damage;
use crate::game_variables::GameVariables;
use crate::tick_context::TickContext;
use uom::si::f32::Ratio;
use uom::si::f32::Time;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneratorState {
    Online { damage: Damage },
    Offline,
    Repairing { event_end: Time },
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self::Online {
            damage: Damage::default(),
        }
    }
}

impl GeneratorState {
    pub fn repair(self, current_mission_time: Time, game_vars: &GameVariables) -> Self {
        match self {
            Self::Repairing { .. } => self,
            Self::Online { damage } if damage.is_undamaged() => self,
            Self::Online { damage } => Self::Repairing {
                event_end: current_mission_time + damage.inverse_apply(game_vars.repair_time),
            },
            Self::Offline => {
                let damage = Damage::offline();
                Self::Repairing {
                    event_end: current_mission_time + damage.inverse_apply(game_vars.repair_time),
                }
            }
        }
    }
    pub fn damage(&mut self, amount: Ratio) {
        match self {
            Self::Online { damage } => {
                damage.damage(amount);
                if damage.is_offline() {
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
                    damage: Damage::default(),
                };
            }
        }
    }
}
