use uom::si::f32::Time;

pub enum SystemRAGStatus {
    Green,
    Amber,
    Red,
    ThrobbingAmber,
}

pub struct SystemStatusIndicators {
    pub solar: SystemRAGStatus,
    pub batteries: SystemRAGStatus,
    pub reactor: SystemRAGStatus,
    pub life_support: SystemRAGStatus,
    pub comms: SystemRAGStatus,
    pub operations: SystemRAGStatus,
}

pub struct SystemIntegrity {
    pub micrometeorites_active: bool,
    pub lunar_quake_active: bool,
    pub solar_flare_active: bool,
    pub micrometeorites_countdown: Option<Time>,
    pub lunar_quake_countdown: Option<Time>,
    pub solar_flare_countdown: Option<Time>,
    pub system_status_indicators: SystemStatusIndicators,
}
