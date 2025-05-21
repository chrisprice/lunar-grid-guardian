use std::marker::PhantomData;
use uom::si::f32::Ratio;

pub mod battery;
pub mod damage;
pub mod event_state;
pub mod game_state;
pub mod game_variables;
pub mod generator;
pub mod lunar_phase;
pub mod operations;
pub mod solar;
pub mod tick_context;

trait ConstOne {
    const ONE: Self;
}

impl ConstOne for Ratio {
    const ONE: Self = Ratio {
        dimension: PhantomData,
        units: PhantomData,
        value: 1.0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::ratio::ratio;
    #[test]
    fn test() {
        assert_eq!(Ratio::ONE, Ratio::new::<ratio>(1.0));
    }
}
