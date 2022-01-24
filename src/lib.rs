//! [Gaussian integers](https://en.wikipedia.org/wiki/Gaussian_integer).

#[allow(dead_code)]
use num_complex::Complex;
use num_traits::{PrimInt, Signed};

/// A Gaussian integer.
#[derive(Debug, PartialEq, Eq)]
pub struct GaussianInt<T: PrimInt>(Complex<T>);

impl<T: PrimInt> GaussianInt<T> {
    pub fn new(r: T, i: T) -> Self {
        Self(Complex::new(r, i))
    }
}

impl<T: PrimInt> std::ops::Add for GaussianInt<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.0.re + other.0.re, self.0.im + other.0.im)
    }
}

impl<T: PrimInt> std::ops::Sub for GaussianInt<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.0.re - other.0.re, self.0.im - other.0.im)
    }
}

impl<T: PrimInt + Signed> GaussianInt<T> {
    /// Returns the complex conjugate.
    pub fn conj(&self) -> Self {
        Self::new(self.0.re, -self.0.im)
    }

    /// Test for [Gaussian primality].
    ///
    /// A Gaussian integer *a* + *b*i is a *Gaussian prime* if and only if either:
    ///
    /// 1. one of *a*, *b* is zero,
    ///    and the absolute value of the other
    ///    is a prime number of the form 4*n* + 3
    ///    (with *n* a nonnegative integer)
    /// 2. both *a* and *b* are nonzero,
    ///    and *a*^2 + *b*^2 is a prime number
    ///    (which will not be of the form 4*n* + 3).
    ///
    /// [Gaussian primality]: https://en.wikipedia.org/wiki/Gaussian_integer#Gaussian_primes
    pub fn is_gaussian_prime(&self) -> bool {
        let a = self.0.re;
        let b = self.0.im;

        let condition_1 = match (a.is_zero(), b.is_zero()) {
            (true, true) => false,
            (false, false) => false,
            (true, false) => {
                let other = b.to_i32().unwrap();
                is_prime(other) && (other - 3) % 4 == 0
            }
            (false, true) => {
                let other = a.to_i32().unwrap();
                is_prime(other) && (other - 3) % 4 == 0
            }
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
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = GaussianInt::new(1, 1);
        assert_eq!(c.0, Complex::new(1, 1));
    }

    #[test]
    fn addition() {
        let c1 = GaussianInt::new(1, 1);
        let c2 = GaussianInt::new(1, 1);
        assert_eq!(c1 + c2, GaussianInt::new(2, 2));

        let c1 = GaussianInt::new(-15, 3);
        let c2 = GaussianInt::new(8, 7);
        assert_eq!(c1 + c2, GaussianInt::new(-7, 10));
    }

    #[test]
    fn subtraction() {
        let c1 = GaussianInt::new(1, 1);
        let c2 = GaussianInt::new(1, 1);
        assert_eq!(c1 - c2, GaussianInt::new(0, 0));

        let c1 = GaussianInt::new(-15, 3);
        let c2 = GaussianInt::new(8, 7);
        assert_eq!(c1 - c2, GaussianInt::new(-23, -4));
    }

    #[test]
    fn from_i32() {
        let c = GaussianInt::new(5, 0);
        assert_eq!(5i32, c.into());
    }

    #[test]
    fn from_isize() {
        let c = GaussianInt::new(5, 0);
        assert_eq!(5isize, c.into());
    }

    #[test]
    fn conjugate() {
        let c: GaussianInt<i32> = GaussianInt::new(5, 5);
        let conj: GaussianInt<i32> = GaussianInt::new(5, -5);
        assert_eq!(c.conj(), conj);
    }

    #[test]
    fn is_gaussian_prime() {
        let c = GaussianInt::new(2, 0);
        assert_eq!(c.is_gaussian_prime(), false);
        let c = GaussianInt::new(3, 0);
        assert_eq!(c.is_gaussian_prime(), true);
        let c = GaussianInt::new(5, 0);
        assert_eq!(c.is_gaussian_prime(), false);
        let c = GaussianInt::new(7, 0);
        assert_eq!(c.is_gaussian_prime(), true);
        let c = GaussianInt::new(11, 0);
        assert_eq!(c.is_gaussian_prime(), true);
    }
}
