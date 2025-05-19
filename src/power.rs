use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Power(pub f32);

impl Power {
    pub fn new(value: f32) -> Self {
        Power(value)
    }
}

impl Add for Power {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Power(self.0 + other.0)
    }
}

impl AddAssign for Power {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Power {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Power(self.0 - other.0)
    }
}

impl SubAssign for Power {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Mul<f32> for Power {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Power(self.0 * rhs)
    }
}

impl Mul<Power> for f32 {
    type Output = Power;
    fn mul(self, rhs: Power) -> Power {
        Power(self * rhs.0)
    }
}

impl Div<f32> for Power {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        if rhs == 0.0 {
            Power(f32::NAN) // Or handle as an error
        } else {
            Power(self.0 / rhs)
        }
    }
}

impl Div<Power> for Power {
    type Output = f32;
    fn div(self, rhs: Power) -> f32 {
        if rhs.0 == 0.0 {
            f32::NAN // Or handle as an error
        } else {
            self.0 / rhs.0
        }
    }
}

impl Neg for Power {
    type Output = Self;
    fn neg(self) -> Self {
        Power(-self.0)
    }
}

impl Sum<Self> for Power {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Power::new(0.0), Add::add)
    }
}
