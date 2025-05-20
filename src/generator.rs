use crate::game_variables::GameVariables;
use uom::si::f32::Time;
use uom::si::time::second;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneratorState {
    Online { damage_percentage: f32 },
    Offline,
    Repairing { event_end: Time },
}

impl GeneratorState {
    pub fn new() -> Self {
        GeneratorState::Online {
            damage_percentage: 0.0,
        }
    }
    /// Transitions the system to a Repairing state, calculating the finish time.
    /// Call this when a repair action is initiated.
    pub fn repair(self, current_mission_time: Time, game_vars: &GameVariables) -> Self {
        let initial_damage = match self {
            GeneratorState::Online { damage_percentage } => damage_percentage,
            GeneratorState::Offline => 100.0, // Assume 100% damage if Offline
            GeneratorState::Repairing { .. } => return self, // Already repairing
        };

        if initial_damage <= 0.0 {
            return GeneratorState::Online {
                damage_percentage: 0.0,
            }; // No damage to repair
        }

        let repair_duration_seconds = initial_damage * game_vars.repair_time_per_damage_unit.get::<second>();
        let event_end = current_mission_time + Time::new::<second>(repair_duration_seconds);

        GeneratorState::Repairing { event_end }
    }
    /// Applies damage to the system, increasing its damage percentage.
    /// If damage exceeds 100%, the system becomes Offline.
    pub fn damage(&mut self, amount: f32) {
        match self {
            GeneratorState::Online { damage_percentage } => {
                *damage_percentage = (*damage_percentage + amount).min(100.0);
                if *damage_percentage >= 100.0 {
                    *self = GeneratorState::Offline;
                }
            }
            GeneratorState::Repairing { .. } => { /* Ignore damage while repairing */ }
            GeneratorState::Offline => { /* Already offline */ }
        }
    }

    /// Checks if repair is complete and updates state accordingly.
    /// Call this on each game tick for systems that can be repaired.
    pub fn tick(&mut self, current_mission_time: Time) {
        if let GeneratorState::Repairing {
            event_end: repair_finish_time,
            ..
        } = self
        {
            if current_mission_time >= *repair_finish_time {
                *self = GeneratorState::Online {
                    damage_percentage: 0.0,
                };
            }
        }
    }
}
