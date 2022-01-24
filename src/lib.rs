//! Gaussian integers.
//!
//! https://en.wikipedia.org/wiki/Gaussian_integer

#[allow(dead_code)]
use num_complex::Complex;
use num_traits::PrimInt;

/// A Gaussian integer.
#[derive(Debug)]
struct GaussianInt<T: PrimInt>(Complex<T>);

impl<T: PrimInt> GaussianInt<T> {
    fn new(r: T, i: T) -> Self {
        GaussianInt(Complex::new(r, i))
    }
}

// get a u32 from a GaussInt
impl<T: PrimInt> From<GaussianInt<T>> for u32 {
    fn from(g: GaussianInt<T>) -> Self {
        todo!();
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
}
