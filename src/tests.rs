use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = GaussianInt::new(1, 1);
        assert_eq!(c.0, Complex::new(1, 1));
    }

    #[test]
    fn from_complex() {
        let c = Complex::new(5, 5);
        let g = GaussianInt::new(5, 5);
        assert_eq!(GaussianInt::from(c), g);
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
    fn norm() {
        let c = GaussianInt::new(1, 1);
        assert_eq!(c.norm(), GaussianInt::new(2, 0));
    }

    #[test]
    fn multiplication() {
        let c1 = GaussianInt::new(1, 1);
        let c2 = GaussianInt::new(1, -1);
        assert_eq!(c1 * c2, GaussianInt::new(2, 0));

        let c1 = GaussianInt::new(3, 2);
        let c2 = GaussianInt::new(2, 3);
        assert_eq!(c1 * c2, GaussianInt::new(0, 13));
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
