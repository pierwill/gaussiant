//! Gaussian integers in Rust.
//!
//! A [Gaussian integer] is a complex number whose real and imaginary parts are both integers.
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
use num_integer::Integer;
use num_traits::{One, PrimInt, Signed, Zero};

mod ops;

/// A [Gaussian integer] is a complex number whose real and imaginary parts are both integers.
///
/// [Gaussian integer]: https://en.wikipedia.org/wiki/Gaussian_integer
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GaussianInt<T: PrimInt + Integer>(pub Complex<T>);

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

impl<T: PrimInt + Integer> GaussianInt<T> {
    #[allow(missing_docs)]
    pub fn new(r: T, i: T) -> Self {
        Self(Complex::new(r, i))
    }

    /// Given a Gaussian integer z₀, called a *modulus*,
    /// two Gaussian integers z₁, z₂ are *congruent modulo z₀*,
    /// if their difference is a multiple of z₀.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::{GaussianInt, gaussint};
    /// # fn main() {
    /// // 2 + 5i ≡ i mod 1 + 2i
    /// let c1 = gaussint!(2, 5);
    /// let c2 = gaussint!(0, 1);
    /// let c3 = gaussint!(1, 2);
    /// assert!(c1.congruent(c2, c3));
    /// # }
    /// ```
    pub fn congruent(&self, other: Self, modulus: Self) -> bool {
        (*self - other) % modulus == Self::zero()
    }
}

impl<T: PrimInt + Integer + Signed> GaussianInt<T> {
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

    /// Returns `true` if `self` divides `other`.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let c1 = GaussianInt::new(5, 0);
    /// let c2 = GaussianInt::new(1, 2);
    /// assert!(c2.divides(c1));
    /// # }
    /// ```
    pub fn divides(&self, other: Self) -> bool {
        *self != Self::zero() && (other % *self) == Self::zero()
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

        // These numbers would cause integer overflow panics below.
        match (a.abs().to_isize().unwrap(), b.abs().to_isize().unwrap()) {
            (0, 0) => return false,
            (1, 1) => return true,
            (-1, -1) => return true,
            (2, 0) => return false,
            (0, 2) => return false,
            _ => {}
        }

        let condition_1 = match (a.is_zero(), b.is_zero()) {
            (true, false) => {
                let other = b.abs().to_u64().unwrap();
                primal::is_prime(other) && (other - 3) % 4 == 0
            }
            (false, true) => {
                let other = a.abs().to_u64().unwrap();
                primal::is_prime(other) && (other - 3) % 4 == 0
            }
            _ => false,
        };

        let condition_2 = match (a.is_zero(), b.is_zero()) {
            (false, false) => {
                let a = a.abs().to_u64().unwrap();
                let b = b.abs().to_u64().unwrap();
                let sum_of_squares = u64::pow(a, 2) + u64::pow(b, 2);
                let sum_of_squares_is_4n_plus_3 = (sum_of_squares - 3) % 4 == 0;
                primal::is_prime(sum_of_squares) && !sum_of_squares_is_4n_plus_3
            }
            _ => false,
        };

        condition_1 || condition_2
    }

    /// Returns an array of the units of ℤ\[*i*\], the ring of Gaussian integers.
    ///
    /// The units are 1, -1, *i*, -*i*.
    pub fn units() -> [Self; 4] {
        [
            Self::one(),                     //  1
            -Self::one(),                    // -1
            Self::new(T::zero(), T::one()),  //  i
            Self::new(T::zero(), -T::one()), // -i
        ]
    }

    /// Gaussian integers are called associates
    /// if they can be obtained from one another by multiplication by units.
    ///
    /// # Example
    ///
    /// ```
    /// # use gaussiant::GaussianInt;
    /// # fn main() {
    /// let z1 = GaussianInt::new(1, 1);
    /// let z2 = GaussianInt::new(1, -1);
    /// assert!(z1.is_associated(z2));
    /// # }
    /// ```
    pub fn is_associated(&self, other: Self) -> bool {
        for u in GaussianInt::units() {
            if (*self * u) == other {
                return true;
            }
        }
        false
    }

    /// Tests whether a Gaussian integer is "even."
    ///
    /// A Gaussian integer *z* is "even" if *z* ≡ 0 mod 1+*i*.
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
        let modulus = Self::new(T::one(), T::one());
        // self ≡ 0 mod 1+i
        self.congruent(Self::zero(), modulus)
    }

    /// Tests whether a Gaussian integer is "odd."
    ///
    /// A Gaussian integer *z* is "odd" if *z* ≡ 1 mod 1+*i*.
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
        let modulus = Self::new(T::one(), T::one());
        // self ≡ 1 mod 1+i
        self.congruent(Self::one(), modulus)
    }
}

impl<T: PrimInt + Integer + Signed> GaussianInt<T>
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

/// Returns an iterator of all Gaussian primes *a* + *b*i
/// where |a|,|b| ≤ `n`.
pub fn get_g_primes(n: isize) -> impl Iterator<Item = GaussianInt<isize>> + 'static {
    let mut primes: Vec<GaussianInt<_>> = vec![];
    for a in -n..=n {
        for b in -n..=n {
            let z = GaussianInt::new(a, b);
            if z.is_gaussian_prime() {
                primes.push(z);
            }
        }
    }
    primes.into_iter()
}

/// Returns an iterator of all Gaussian integers *a* + *b*i
/// where |*a*|,|*b*| ≤ `n`.
pub fn get_g_ints(n: isize) -> impl Iterator<Item = GaussianInt<isize>> + 'static {
    let mut integers: Vec<GaussianInt<_>> = vec![];
    for a in -n..=n {
        for b in -n..=n {
            let z = GaussianInt::new(a, b);
            integers.push(z);
        }
    }
    integers.into_iter()
}

/// Returns an iterator of all Gaussian integers *a* + *b*i
/// where *a* is positive (or zero) and |*b*| ≤ `n`.
pub fn get_pos_g_ints(n: isize) -> impl Iterator<Item = GaussianInt<isize>> + 'static {
    let mut pos_integers: Vec<GaussianInt<_>> = vec![];
    for a in 0..=n {
        for b in -n..=n {
            let z = GaussianInt::new(a, b);
            pos_integers.push(z);
        }
    }
    pos_integers.into_iter()
}

impl<T: PrimInt + Integer> One for GaussianInt<T> {
    fn one() -> Self {
        GaussianInt::new(T::one(), T::zero())
    }
}

impl<T: PrimInt + Integer> Zero for GaussianInt<T> {
    fn zero() -> Self {
        GaussianInt::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        *self == GaussianInt::zero()
    }
}

impl<T: PrimInt + Integer> From<Complex<T>> for GaussianInt<T> {
    fn from(z: Complex<T>) -> Self {
        Self(z)
    }
}

impl<T: PrimInt + Integer> From<GaussianInt<T>> for isize {
    fn from(g: GaussianInt<T>) -> Self {
        g.0.re.to_isize().unwrap()
    }
}

impl<T: PrimInt + Integer> fmt::Display for GaussianInt<T> {
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
