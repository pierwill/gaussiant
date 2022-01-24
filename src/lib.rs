//! Gaussian integers.
//!
//! https://en.wikipedia.org/wiki/Gaussian_integer

#[allow(dead_code)]
use num_complex::Complex;
use num_traits::PrimInt;

/// A Gaussian integer.
#[derive(Debug)]
pub struct GaussianInt<T: PrimInt>(Complex<T>);

impl<T: PrimInt> GaussianInt<T> {
    pub fn new(r: T, i: T) -> Self {
        GaussianInt(Complex::new(r, i))
    }
}

impl<T: PrimInt> From<GaussianInt<T>> for u32 {
    fn from(g: GaussianInt<T>) -> Self {
        g.0.re.to_u32().unwrap()
    }
}

impl<T: PrimInt> From<GaussianInt<T>> for usize {
    fn from(g: GaussianInt<T>) -> Self {
        g.0.re.to_usize().unwrap()
    }
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
        assert_eq!(5u32, c.into());
    }

    #[test]
    fn from_usize() {
        let c = GaussianInt::new(5, 0);
        assert_eq!(5u32, c.into());
    }
}
