use uom::si::f32::{Ratio, Time};

pub struct Battery {
    pub charge: Ratio,
    pub damage: Ratio,
}

pub struct Reactor {
    pub power: Ratio,
    pub coolant: Ratio,
    pub damage: Ratio,
}

pub struct Solar {
    pub power: Ratio,
    pub damage: Ratio,
    pub sunrise_countdown: Time,
    pub sunset_countdown: Time,
    pub solar_shields_active: bool,
}

pub struct GenerationControl {
    pub battery: Battery,
    pub reactor: Reactor,
    pub solar: Solar,
}