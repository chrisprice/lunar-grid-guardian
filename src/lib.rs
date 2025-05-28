use rand::RngCore;
use std::marker::PhantomData;
use uom::si::f32::Ratio;

pub mod damage;
pub mod event;
pub mod game_state;
pub mod game_variables;
pub mod lunar_phase;
pub mod system;
pub mod tick_context;

#[cfg(test)]
mod test;

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

/// Returns a random number generator.
pub fn rng() -> impl RngCore {
    rand::rng()
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
