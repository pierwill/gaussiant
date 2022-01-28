use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = gauss!(1, 1);
        assert_eq!(c.0, Complex::new(1, 1));

        let c: GaussianInt<i128> = gauss!(1, i128::pow(2, 100));
        assert_eq!(c.0, Complex::new(1, i128::pow(2, 100)));
    }

    #[test]
    fn gauss_macro() {
        let z = gauss!(1, 47);
        assert_eq!(z, gauss!(1, 47));
    }

    #[test]
    fn from_complex() {
        let c = Complex::new(5, 5);
        let g = gauss!(5, 5);
        assert_eq!(GaussianInt::from(c), g);
    }

    #[test]
    fn addition() {
        let c1 = gauss!(1, 1);
        let c2 = gauss!(1, 1);
        assert_eq!(c1 + c2, gauss!(2, 2));

        let c1 = gauss!(-15, 3);
        let c2 = gauss!(8, 7);
        assert_eq!(c1 + c2, gauss!(-7, 10));
    }

    #[test]
    fn subtraction() {
        let c1 = gauss!(1, 1);
        let c2 = gauss!(1, 1);
        assert_eq!(c1 - c2, gauss!(0, 0));

        let c1 = gauss!(-15, 3);
        let c2 = gauss!(8, 7);
        assert_eq!(c1 - c2, gauss!(-23, -4));
    }

    #[test]
    fn norm() {
        let c = gauss!(1, 1);
        assert_eq!(c.norm(), gauss!(2, 0));

        let c = gauss!(4, 5);
        assert_eq!(c.norm(), gauss!(41, 0));
    }

    #[test]
    fn multiplication() {
        let c1 = gauss!(1, 1);
        let c2 = gauss!(1, -1);
        assert_eq!(c1 * c2, gauss!(2, 0));

        let c1 = gauss!(3, 2);
        let c2 = gauss!(2, 3);
        assert_eq!(c1 * c2, gauss!(0, 13));
    }

    #[test]
    fn division() {
        // See https://projecteuler.net/problem=153
        let c1 = gauss!(5, 0);
        let c2 = gauss!(1, 2);
        assert_eq!(c1 / c2, gauss!(1, -2));
    }

    #[test]
    fn remainder() {
        let c1 = gauss!(2, 2);
        let c2 = gauss!(2, 2);
        assert_eq!(c1 % c2, GaussianInt::zero());

        let c1 = gauss!(1, 2);
        let c2 = gauss!(3, 4);
        assert!(c1 % c2 != GaussianInt::zero());
    }

    #[test]
    fn neg() {
        let z = gauss!(2, 2);
        assert_eq!(z + -z, GaussianInt::zero());
    }

    #[test]
    fn is_divisor_of() {
        let five = gauss!(5, 0);
        assert!(gauss!(1, 0).is_divisor_of(five));
        assert!(gauss!(1, 2).is_divisor_of(five));
        assert!(gauss!(1, -2).is_divisor_of(five));
        assert!(gauss!(2, 1).is_divisor_of(five));
        assert!(gauss!(2, -1).is_divisor_of(five));
        assert!(five.is_divisor_of(five));
    }

    #[test]
    fn from_isize() {
        let c = gauss!(5, 0);
        assert_eq!(5_isize, c.into());
    }

    #[test]
    fn conjugate() {
        let c: GaussianInt<isize> = gauss!(5, 5);
        let conj: GaussianInt<isize> = gauss!(5, -5);
        assert_eq!(c.conj(), conj);
    }

    #[test]
    fn is_gaussian_prime() {
        let c = gauss!(2, 0);
        assert_eq!(c.is_gaussian_prime(), false);

        let c = gauss!(3, 0);
        assert_eq!(c.is_gaussian_prime(), true);

        let c = gauss!(5, 0);
        assert_eq!(c.is_gaussian_prime(), false);

        let c = gauss!(7, 0);
        assert_eq!(c.is_gaussian_prime(), true);

        let c = gauss!(11, 0);
        assert_eq!(c.is_gaussian_prime(), true);
    }

    #[test]
    fn is_rational() {
        let c = gauss!(7, 0);
        assert!(c.is_rational());

        let c = gauss!(5, 1);
        assert!(!c.is_rational());
    }

    #[test]
    fn to_polar() {
        let c = gauss!(5, 0);
        assert_eq!(c.to_polar(), (5f64, 0f64));

        let c = gauss!(0, 1);
        assert_eq!(c.to_polar(), (1f64, std::f64::consts::PI / 2f64));

        let c = gauss!(0, -1);
        assert_eq!(c.to_polar(), (1f64, -std::f64::consts::PI / 2f64));
    }
    #[test]
    fn congruence() {
        let c1 = gauss!(5, 0);
        let c2 = gauss!(25, 0);
        let c3 = gauss!(10, 0);
        assert!(c1.congruent(c2, c3));
    }

    #[test]
    fn is_even() {
        let z = gauss!(4, 0);
        assert!(z.is_even());

        let z = gauss!(-3, 1);
        assert!(z.is_even());
    }

    #[test]
    fn is_odd() {
        let z = gauss!(1, 0);
        assert!(z.is_odd());

        let z = gauss!(2, 1);
        assert!(z.is_odd());
    }
}
