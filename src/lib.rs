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

#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[cfg(doctest)]
doctest!("../README.md", readme);

use std::fmt;

use num_complex::Complex;
use num_traits::{PrimInt, Signed};

mod ops;

/// A Gaussian integer.
///
/// This is a complex number whose real and imaginary parts are both integers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GaussianInt<T: PrimInt>(pub Complex<T>);

/// Creates a new [`GaussianInt`].
///
/// # Example
///
/// ```
/// use gaussiant::{GaussianInt, gaussint};
/// fn main() {
///     let z = gaussint!(1, 1);
///     let _z = gaussint!(1, -1);
///     assert_eq!(z * _z, gaussint!(2, 0));
/// }
/// ```
#[macro_export]
macro_rules! gaussint {
    ($a:expr,$b:expr) => {
        GaussianInt::new($a, $b)
    };
    ($n:expr) => {
        GaussianInt::new($n, 0)
    };
}

impl<T: PrimInt> GaussianInt<T> {
    #[allow(missing_docs)]
    pub fn new(r: T, i: T) -> Self {
        Self(Complex::new(r, i))
    }

    /// Returns zero as a [`GaussianInt`].
    pub fn zero() -> Self {
        Self(Complex::new(T::zero(), T::zero()))
    }

    /// Given a Gaussian integer z0, called a modulus,
    /// two Gaussian integers z1, z2 are *congruent modulo z0*,
    /// if their difference is a multiple of z0.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let c1 = GaussianInt::new(5, 0);
    /// let c2 = GaussianInt::new(25, 0);
    /// let c3 = GaussianInt::new(10, 0);
    /// assert!(c1.congruent(c2, c3));
    /// # }
    /// ```
    pub fn congruent(&self, other: Self, modulus: Self) -> bool {
        (*self - other) % modulus == Self::zero()
    }
}

impl<T: PrimInt + Signed> GaussianInt<T> {
    /// Returns the complex conjugate.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let z = GaussianInt::new(5, 4);
    /// assert_eq!(z.conj(), GaussianInt::new(5, -4));
    /// # }
    /// ```
    pub fn conj(&self) -> Self {
        Self::new(self.0.re, -self.0.im)
    }

    /// Returns the norm.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let z = GaussianInt::new(2, 7);
    /// assert_eq!(z.norm(), GaussianInt::new(53, 0));
    /// # }
    /// ```
    pub fn norm(&self) -> Self {
        *self * self.conj()
    }

    /// Returns `true` if `self` is a divisor of `other`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let c1 = GaussianInt::new(5, 0);
    /// let c2 = GaussianInt::new(1, 2);
    /// assert!(c2.is_divisor_of(c1));
    /// # }
    /// ```
    pub fn is_divisor_of(&self, other: Self) -> bool {
        (other % *self) == Self::zero() && !(other.0.re != T::zero() && other.0.im != T::zero())
    }

    /// Tests whether a Gaussian integer is a rational integer.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let z = GaussianInt::new(2, 7);
    /// let z2 = GaussianInt::new(0, -7);
    /// assert!((z + z2).is_rational());
    /// # }
    /// ```
    pub fn is_rational(&self) -> bool {
        self.0.im == T::zero()
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
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let z = GaussianInt::new(2, 7);
    /// assert!(z.is_gaussian_prime());
    /// # }
    /// ```
    pub fn is_gaussian_prime(&self) -> bool {
        let a = self.0.re;
        let b = self.0.im;

        let condition_1 = match (a.is_zero(), b.is_zero()) {
            (true, false) => {
                let other = b.to_isize().unwrap();
                is_prime(other) && (other - 3) % 4 == 0
            }
            (false, true) => {
                let other = a.to_isize().unwrap();
                is_prime(other) && (other - 3) % 4 == 0
            }
            _ => false,
        };

        let condition_2 = match (a.is_zero(), b.is_zero()) {
            (false, false) => {
                let a = a.to_isize().unwrap();
                let b = b.to_isize().unwrap();
                let sum_of_squares = isize::pow(a, 2) + isize::pow(b, 2);
                let sum_of_squares_is_4n_plus_3 = (sum_of_squares - 3) % 4 == 0;
                is_prime(sum_of_squares) && !sum_of_squares_is_4n_plus_3
            }
            _ => false,
        };

        condition_1 || condition_2
    }
}

impl GaussianInt<isize> {
    /// Tests whether a Gaussian integer is "even."
    ///
    /// See <https://en.wikipedia.org/wiki/Gaussian_integer#Examples>.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let z = GaussianInt::new(3, 1);
    /// assert!(z.is_even());
    /// # }
    /// ```
    pub fn is_even(&self) -> bool {
        self.congruent(Self::zero(), Self::new(1, 1))
    }

    /// Tests whether a Gaussian integer is "odd."
    ///
    /// See <https://en.wikipedia.org/wiki/Gaussian_integer#Examples>.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let z = GaussianInt::new(2, 1);
    /// assert!(z.is_odd());
    /// # }
    /// ```
    pub fn is_odd(&self) -> bool {
        let one = Self::new(1, 0);
        self.congruent(one, Self::new(1, 1))
        // *self % (Self::new(1, 1) + one) == Self::zero()
    }
}

impl<T: PrimInt + Signed> GaussianInt<T>
where
    f64: From<T>,
{
    /// Convert to polar form (r, theta), such that
    /// `self = r * exp(i * theta)`
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let c = GaussianInt::new(0, 1);
    /// // The polar form of *i* is (1, π/2).
    /// assert_eq!(c.to_polar(), (1f64, std::f64::consts::PI / 2f64));
    /// # }
    /// ```
    pub fn to_polar(&self) -> (f64, f64) {
        let a = self.0.re;
        let b = self.0.im;
        let a: f64 = a.into();
        let b: f64 = b.into();
        Complex::new(a, b).to_polar()
    }
}

/// Returns an iterator of all Gaussian primes with positive real parts
/// and with integer parts below *n*.
pub fn get_positive_primes(n: isize) -> impl Iterator<Item = GaussianInt<isize>> + 'static {
    let mut primes: Vec<GaussianInt<_>> = vec![];
    for a in 0..=n {
        for b in 0..=n {
            let z = GaussianInt::new(a, b);
            if z.is_gaussian_prime() {
                primes.push(z);
            }
        }
    }
    primes.into_iter()
}

/// Returns an iterator of all Gaussian integers with positive real parts
/// and with integer parts below *n*.
pub fn get_g_ints(n: isize) -> impl Iterator<Item = GaussianInt<isize>> + 'static {
    let mut primes: Vec<GaussianInt<_>> = vec![];
    for a in 0..=n {
        for b in -n..=n {
            let z = GaussianInt::new(a, b);
            primes.push(z);
        }
    }
    primes.into_iter()
}

impl<T: PrimInt> From<Complex<T>> for GaussianInt<T> {
    fn from(g: Complex<T>) -> Self {
        Self(g)
    }
}

impl<T: PrimInt> From<GaussianInt<T>> for isize {
    fn from(g: GaussianInt<T>) -> Self {
        g.0.re.to_isize().unwrap()
    }
}

fn is_prime(number: isize) -> bool {
    for i in 2..(number / 2 + 1) {
        if number % i == 0 {
            return false;
        }
    }
    return number > 1;
}

impl<T: PrimInt> fmt::Display for GaussianInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let zero = T::zero();
        if self.0.im < zero {
            write!(
                f,
                "{}{}i",
                self.0.re.to_isize().unwrap(),
                self.0.im.to_isize().unwrap()
            )
        } else {
            write!(
                f,
                "{}+{}i",
                self.0.re.to_isize().unwrap(),
                self.0.im.to_isize().unwrap()
            )
        }
    }
}

#[cfg(test)]
mod tests;
