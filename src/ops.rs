use crate::GaussianInt;
use num_integer::Integer;
use num_traits::{PrimInt, Signed};

impl<T: PrimInt + Integer> std::ops::Add for GaussianInt<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::from(self.0 + other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::Sub for GaussianInt<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::from(self.0 - other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::Mul for GaussianInt<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self::from(self.0 * other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::Div for GaussianInt<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self::from(self.0 / other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::Rem for GaussianInt<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        Self::from(self.0 % other.0)
    }
}

impl<T: PrimInt + Integer + Signed> std::ops::Neg for GaussianInt<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::from(-self.0)
    }
}

// assignment ops

impl<T: PrimInt + Integer> std::ops::AddAssign for GaussianInt<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self::from(self.0 + other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::SubAssign for GaussianInt<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::from(self.0 - other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::MulAssign for GaussianInt<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self::from(self.0 * other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::DivAssign for GaussianInt<T> {
    fn div_assign(&mut self, other: Self) {
        *self = Self::from(self.0 / other.0)
    }
}

impl<T: PrimInt + Integer> std::ops::RemAssign for GaussianInt<T> {
    fn rem_assign(&mut self, other: Self) {
        *self = Self::from(self.0 % other.0)
    }
}
