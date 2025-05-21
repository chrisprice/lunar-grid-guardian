use std::ops::Mul;
use uom::{
    si::{f32::Ratio, Dimension, Quantity, SI}, ConstZero,
};

use crate::ConstOne;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Damage {
    value: Ratio,
}

impl Damage {
    pub fn offline() -> Self {
        Self { value: Ratio::ONE }
    }
    pub fn new(mut value: Ratio) -> Self {
        if value < Ratio::ZERO {
            value = Ratio::ZERO;
        }
        if value > Ratio::ONE {
            value = Ratio::ONE;
        }
        Self { value }
    }
    pub fn inner(&self) -> Ratio {
        self.value
    }
    pub fn damage(&mut self, amount: Ratio) {
        self.value += amount;
        if self.value > Ratio::ONE {
            self.value = Ratio::ONE;
        }
    }
    pub fn repair(&mut self, amount: Ratio) {
        self.value -= amount;
        if self.value < Ratio::ZERO {
            self.value = Ratio::ZERO;
        }
    }
    pub fn is_offline(&self) -> bool {
        self.value == Ratio::ONE
    }
    pub fn is_undamaged(&self) -> bool {
        self.value == Ratio::ZERO
    }
    pub fn apply<D: Dimension + ?Sized>(
        &self,
        value: Quantity<D, SI<f32>, f32>,
    ) -> Quantity<D, SI<f32>, f32>
    where
        Quantity<D, SI<f32>, f32>: Mul<Ratio, Output = Quantity<D, SI<f32>, f32>>,
    {
        value * (Ratio::ONE - self.value)
    }
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
