use std::{marker::PhantomData, ops::Mul};
use uom::{
    ConstZero,
    si::{Dimension, Quantity, SI, f32::Ratio},
};

pub mod battery;
pub mod event_state;
pub mod game_state;
pub mod game_variables;
pub mod generator;
pub mod lunar_phase;
pub mod operations;
pub mod solar;
pub mod tick_context;

pub const ONE: Ratio = Ratio {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Damage {
    value: Ratio,
}

impl Damage {
    pub fn offline() -> Self {
        Self { value: ONE }
    }
    pub fn new(mut value: Ratio) -> Self {
        if value < Ratio::ZERO {
            value = Ratio::ZERO;
        }
        if value > ONE {
            value = ONE;
        }
        Self { value }
    }
    pub fn inner(&self) -> Ratio {
        self.value
    }
    pub fn damage(&mut self, amount: Ratio) {
        self.value += amount;
        if self.value > ONE {
            self.value = ONE;
        }
    }
    pub fn repair(&mut self, amount: Ratio) {
        self.value -= amount;
        if self.value < Ratio::ZERO {
            self.value = Ratio::ZERO;
        }
    }
    pub fn is_offline(&self) -> bool {
        self.value == ONE
    }
    pub fn is_undamaged(&self) -> bool {
        self.value == Ratio::ZERO
    }
    /// Applies the damage to a value.
    pub fn apply<D: Dimension + ?Sized>(
        &self,
        value: Quantity<D, SI<f32>, f32>,
    ) -> Quantity<D, SI<f32>, f32>
    where
        Quantity<D, SI<f32>, f32>: Mul<Ratio, Output = Quantity<D, SI<f32>, f32>>,
    {
        value * (ONE - self.value)
    }
    /// Applies the inverse of the damage (i.e. 100% - self) to a value.
    pub fn inverse_apply<D: Dimension + ?Sized>(
        &self,
        value: Quantity<D, SI<f32>, f32>,
    ) -> Quantity<D, SI<f32>, f32>
    where
        Quantity<D, SI<f32>, f32>: Mul<Ratio, Output = Quantity<D, SI<f32>, f32>>,
    {
        value * self.value
    }
}

#[cfg(test)]
mod tests {
    use uom::si::ratio::ratio;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(ONE, Ratio::new::<ratio>(1.0));
    }
}
