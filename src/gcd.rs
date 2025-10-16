use crate::GaussianInt;
use num_integer::Integer;
use num_traits::{PrimInt, Signed, Zero};

impl<T: PrimInt + Integer + Signed> GaussianInt<T> {
    /// Computes the greatest common divisor (GCD) of two Gaussian integers using the Euclidean algorithm.
    ///
    /// Returns a GCD up to multiplication by a unit (1, -1, i, -i). The GCD is normalized
    /// to have a positive real part when possible.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gaussiant::{GaussianInt, gaussint};
    ///
    /// let a = gaussint!(12, 0);
    /// let b = gaussint!(8, 0);
    /// let g = GaussianInt::gcd(a, b);
    /// assert_eq!(g, gaussint!(4, 0));
    ///
    /// // GCD of Gaussian integers
    /// let a = gaussint!(6, 3);
    /// let b = gaussint!(3, 6);
    /// let g = GaussianInt::gcd(a, b);
    /// // gcd(6+3i, 3+6i) = 3
    /// assert_eq!(g.norm(), 9);
    /// ```
    pub fn gcd(mut a: Self, mut b: Self) -> Self {
        // Handle zero cases
        if a.is_zero() {
            return b;
        }
        if b.is_zero() {
            return a;
        }

        // Euclidean algorithm: repeatedly replace (a, b) with (b, a mod b)
        while !b.is_zero() {
            let remainder = a % b;
            a = b;
            b = remainder;
        }

        // Normalize: prefer positive real part
        Self::normalize_gcd(a)
    }

    /// Helper: Normalizes GCD to have positive real part when possible.
    ///
    /// Multiplies by an appropriate unit (1, -1, i, -i) to ensure the GCD has
    /// a canonical form: real part positive, or if zero, imaginary part positive.
    fn normalize_gcd(mut g: Self) -> Self {
        // If real part is negative, multiply by -1
        if g.0.re < T::zero() {
            g = -g;
        }
        // If real part is zero and imaginary part is negative, multiply by -1
        else if g.0.re == T::zero() && g.0.im < T::zero() {
            g = -g;
        }
        g
    }
}
