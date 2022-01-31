use crate::GaussianInt;
use num_integer::Integer;
use num_traits::{PrimInt, Signed};

impl<T: PrimInt + Integer> std::ops::Add for GaussianInt<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.0.re + other.0.re, self.0.im + other.0.im)
    }
}

impl<T: PrimInt + Integer> std::ops::Sub for GaussianInt<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.0.re - other.0.re, self.0.im - other.0.im)
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

// impl<T: PrimInt + Integer> std::ops::AddAssign for GaussianInt<T> {}
// impl<T: PrimInt + Integer> std::ops::SubAssign for GaussianInt<T> {}
// impl<T: PrimInt + Integer> std::ops::MulAssign for GaussianInt<T> {}
// impl<T: PrimInt + Integer> std::ops::DivAssign for GaussianInt<T> {}
// impl<T: PrimInt + Integer> std::ops::RemAssign for GaussianInt<T> {}
