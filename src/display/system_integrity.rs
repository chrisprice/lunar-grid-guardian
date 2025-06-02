use uom::si::f32::Time;

pub enum SystemRAGStatus {
    Green,
    Amber,
    Red,
    ThrobbingAmber,
}

pub struct SystemIntegrity {
    pub micrometeorites_active: bool,
    pub lunar_quake_active: bool,
    pub solar_flare_active: bool,
    pub micrometeorites_countdown: Option<Time>,
    pub lunar_quake_countdown: Option<Time>,
    pub solar_flare_countdown: Option<Time>,
    pub solar_rag_status: SystemRAGStatus,
    pub batteries_rag_status: SystemRAGStatus,
    pub reactor_rag_status: SystemRAGStatus,
    pub life_support_rag_status: SystemRAGStatus,
    pub comms_rag_status: SystemRAGStatus,
    pub operations_rag_status: SystemRAGStatus,
    pub life_support_boost_count: u32,
    pub battery_boost_count: u32,
    pub coolant_boost_count: u32,
    pub repair_boost_count: u32,
}
