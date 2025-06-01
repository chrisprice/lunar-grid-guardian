use uom::si::f32::{Power, Ratio, Time};

pub enum EventAlertStatus {
    NoAlert,
    UnacknowledgedAlert,
    AcknowledgedAlert,
}

pub struct LifeSupportDisplay {
    pub power_level: Power,
    pub colony_damage: Ratio,
    pub emergency_restrictions_status: bool,
}

pub struct CommsDisplay {
    pub micrometeorites_alert_status: EventAlertStatus,
    pub lunar_quakes_alert_status: EventAlertStatus,
    pub solar_flares_alert_status: EventAlertStatus,
    pub online_status: bool,
}

pub struct OperationsDisplay {
    pub power_level: Power,
    pub life_support_boost_count: u32,
    pub battery_boost_count: u32,
    pub coolant_boost_count: u32,
    pub repair_boost_count: u32,
    pub online_status: bool,
    pub pending_docking_indicator: bool,
    pub next_supply_drop_timer: Time,
}

pub struct DemandManagement {
    pub life_support: LifeSupportDisplay,
    pub comms: CommsDisplay,
    pub operations: OperationsDisplay,
}
