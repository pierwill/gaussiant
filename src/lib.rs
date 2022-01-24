//! Gaussian integers in Rust.
//!
//! A [Gaussian integer] is a complex number whose real and imaginary parts are both integers.
//!
//! The centerpiece of this crate is a method to test for a [Gaussian prime].
//!
//! [Gaussian integer]: https://en.wikipedia.org/wiki/Gaussian_integer
//! [Gaussian prime]: crate::GaussianInt#method.is_gaussian_prime
#![deny(missing_docs)]
#![allow(clippy::needless_return)]

use num_complex::Complex;
use num_traits::{PrimInt, Signed};

mod ops;

/// A Gaussian integer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GaussianInt<T: PrimInt>(pub Complex<T>);

impl<T: PrimInt> GaussianInt<T> {
    #[allow(missing_docs)]
    pub fn new(r: T, i: T) -> Self {
        Self(Complex::new(r, i))
    }

    /// Returns zero as a [`GaussianInt`].
    pub fn zero() -> Self {
        Self(Complex::new(T::zero(), T::zero()))
    }
}

impl<T: PrimInt + Signed> GaussianInt<T> {
    /// Returns the complex conjugate.
    pub fn conj(&self) -> Self {
        Self::new(self.0.re, -self.0.im)
    }

    /// Returns the norm.
    pub fn norm(self) -> Self {
        self * self.conj()
    }

    /// Tests for [Gaussian primality].
    ///
    /// A Gaussian integer *a* + *b*i is a *Gaussian prime* if and only if either:
    ///
    /// 1. one of *a*, *b* is zero,
    ///    and the absolute value of the other
    ///    is a prime number of the form 4*n* + 3
    ///    (with *n* a nonnegative integer)
    /// 2. both *a* and *b* are nonzero,
    ///    and *a*² + *b*² is a prime number
    ///    (which will not be of the form 4*n* + 3).
    ///
    /// [Gaussian primality]: https://en.wikipedia.org/wiki/Gaussian_integer#Gaussian_primes
    pub fn is_gaussian_prime(&self) -> bool {
        let a = self.0.re;
        let b = self.0.im;

        let condition_1 = match (a.is_zero(), b.is_zero()) {
            (true, false) => {
                let other = b.to_i32().unwrap();
                is_prime(other) && (other - 3) % 4 == 0
            }
            (false, true) => {
                let other = a.to_i32().unwrap();
                is_prime(other) && (other - 3) % 4 == 0
            }
            _ => false,
        };

        let condition_2 = match (a.is_zero(), b.is_zero()) {
            (false, false) => {
                let a = a.to_i32().unwrap();
                let b = b.to_i32().unwrap();
                let both_prime = is_prime(a) && is_prime(b);
                let sum_of_squares = i32::pow(a, 2) + i32::pow(b, 2);
                let sum_of_squares_is_4n_plus_3 = (sum_of_squares - 3) % 4 == 0;
                both_prime && is_prime(sum_of_squares) && !sum_of_squares_is_4n_plus_3
            }
            _ => false,
        };

        condition_1 || condition_2
    }

    /// Tests whether a Gaussian integer is a rational integer.
    pub fn is_rational(&self) -> bool {
        self.0.im == T::zero()
    }
}

impl<T: PrimInt> From<Complex<T>> for GaussianInt<T> {
    fn from(g: Complex<T>) -> Self {
        Self(g)
    }
}

impl<T: PrimInt> From<GaussianInt<T>> for i32 {
    fn from(g: GaussianInt<T>) -> Self {
        g.0.re.to_i32().unwrap()
    }
}

impl<T: PrimInt> From<GaussianInt<T>> for isize {
    fn from(g: GaussianInt<T>) -> Self {
        g.0.re.to_isize().unwrap()
    }
}

fn is_prime(number: i32) -> bool {
    for i in 2..(number / 2 + 1) {
        if number % i == 0 {
            return false;
        }
    }
    return number > 1;
}

#[cfg(test)]
mod tests;
