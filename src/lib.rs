//! [Gaussian integers](https://en.wikipedia.org/wiki/Gaussian_integer).

#[allow(dead_code)]
use num_complex::Complex;
use num_traits::{PrimInt, Signed};

/// A Gaussian integer.
#[derive(Debug)]
pub struct GaussianInt<T: PrimInt>(Complex<T>);

impl<T: PrimInt> GaussianInt<T> {
    pub fn new(r: T, i: T) -> Self {
        GaussianInt(Complex::new(r, i))
    }
}

impl<T: PrimInt + Signed> GaussianInt<T> {
    /// Test for [Gaussian primality](https://en.wikipedia.org/wiki/Gaussian_integer#Gaussian_primes).
    ///
    /// A Gaussian integer *a* + *b*i is a *Gaussian prime* if and only if either:
    ///
    /// - one of *a*, *b* is zero,
    ///   and the absolute value of the other
    ///   is a prime number of the form 4*n* + 3
    ///   (with *n* a nonnegative integer)
    /// - both are nonzero and *a*^2 + *b*^2 is a prime number
    ///   (which will not be of the form 4*n* + 3).
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

pub fn is_prime(number: i32) -> bool {
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
    fn from_u32() {
        let c = GaussianInt::new(5, 0);
        assert_eq!(5i32, c.into());
    }

    #[test]
    fn from_usize() {
        let c = GaussianInt::new(5, 0);
        assert_eq!(5i32, c.into());
    }

    #[test]
    fn is_gaussian_prime() {
        let c = GaussianInt::new(3, 0);
        assert_eq!(c.is_gaussian_prime(), true);
    }
}
