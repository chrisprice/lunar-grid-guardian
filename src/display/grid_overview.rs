use uom::si::f32::{Frequency, Power, Ratio, Time};

pub struct GridOverview {
    pub total_grid_demand: Power,
    pub total_grid_supply: Power,
    pub frequency: Frequency,
    pub colony_health: Ratio,
    pub mission_timer: Time,
}
