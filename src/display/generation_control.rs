pub struct Battery {
    pub charge: u8,
    pub damage: u8,
}

pub struct Reactor {
    pub power: u8,
    pub coolant: u8,
    pub damage: u8,
}

pub struct Solar {
    pub power: u8,
    pub damage: u8,
    pub sunrise_countdown: u8,
    pub sunset_countdown: u8,
    pub solar_shields_active: bool,
}

pub struct GenerationControl {
    pub battery: Battery,
    pub reactor: Reactor,
    pub solar: Solar,
}