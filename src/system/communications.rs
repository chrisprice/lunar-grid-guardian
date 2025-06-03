use uom::ConstZero;
use uom::si::f32::Power;

use crate::display::demand_management::CommsDisplay;
use crate::event::Event;
use crate::tick_context::TickContext;

/// Represents the state of the Communications system.
#[derive(Debug, Default)]
pub struct Communications {
    offline: bool,
}

impl Communications {
    /// Processes a time step for the Comms system.
    ///
    /// Returns the calculated power demand for the current tick.
    pub fn tick(&mut self, context: &TickContext) -> Power {
        if !self.offline {
            context.game_vars.comms_power_demand
        } else {
            Power::ZERO
        }
    }

    pub fn set_online(&mut self, online: bool) {
        self.offline = !online;
    }

    pub fn display(
        &self,
        micrometeorite_event: &Event,
        lunar_quake_event: &Event,
        solar_flare_event: &Event,
    ) -> CommsDisplay {
        CommsDisplay {
            micrometeorites_alert_status: micrometeorite_event.into(),
            lunar_quakes_alert_status: lunar_quake_event.into(),
            solar_flares_alert_status: solar_flare_event.into(),
            online_status: !self.offline,
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_variables::GameVariables;
    use crate::test::assert_quantities_eq;
    use uom::si::power::watt;

    #[test]
    fn test_comms_initial_state_online_power_demand() {
        let mut comms = Communications::default();
        let context = TickContext::new_static(
            &GameVariables {
                comms_power_demand: Power::new::<watt>(50.0),
                ..Default::default()
            },
            0.0,
            1.0,
        );

        let power_demand = comms.tick(&context);

        assert_quantities_eq(power_demand, 50.0);
        assert!(!comms.offline);
    }

    #[test]
    fn test_comms_offline_power_demand() {
        let mut comms = Communications { offline: true };
        let context = TickContext::new_static(
            &GameVariables {
                comms_power_demand: Power::new::<watt>(50.0),
                ..Default::default()
            },
            0.0,
            1.0,
        );

        let power_demand = comms.tick(&context);

        assert_quantities_eq(power_demand, 0.0);
        assert!(comms.offline);
    }
}
