use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = gaussint!(1, 1);
        assert_eq!(c.0, Complex::new(1, 1));

        let c: GaussianInt<i128> = gaussint!(1, i128::pow(2, 100));
        assert_eq!(c.0, Complex::new(1, i128::pow(2, 100)));
    }

    #[test]
    fn gauss_macro() {
        let z = gaussint!(1, 47);
        assert_eq!(z, gaussint!(1, 47));
        let z = gaussint!(1);
        assert_eq!(z, GaussianInt::new(1, 0));
    }

    #[test]
    fn from_complex() {
        let c = Complex::new(5, 5);
        let g = gaussint!(5, 5);
        assert_eq!(GaussianInt::from(c), g);
    }

    #[test]
    fn addition() {
        let c1 = gaussint!(1, 1);
        let c2 = gaussint!(1, 1);
        assert_eq!(c1 + c2, gaussint!(2, 2));

        let c1 = gaussint!(-15, 3);
        let c2 = gaussint!(8, 7);
        assert_eq!(c1 + c2, gaussint!(-7, 10));
    }

    #[test]
    fn add_assign() {
        let mut c1 = gaussint!(1, 1);
        let c2 = gaussint!(2, 3);
        c1 += c2;
        assert_eq!(c1, gaussint!(3, 4));
    }

    #[test]
    fn subtraction() {
        let c1 = gaussint!(1, 1);
        let c2 = gaussint!(1, 1);
        assert_eq!(c1 - c2, gaussint!(0, 0));

        let c1 = gaussint!(-15, 3);
        let c2 = gaussint!(8, 7);
        assert_eq!(c1 - c2, gaussint!(-23, -4));
    }

    #[test]
    fn norm() {
        let c = gaussint!(1, 1);
        assert_eq!(c.norm(), gaussint!(2, 0));

        let c = gaussint!(4, 5);
        assert_eq!(c.norm(), gaussint!(41, 0));
    }

    #[test]
    fn multiplication() {
        let c1 = gaussint!(1, 1);
        let c2 = gaussint!(1, -1);
        assert_eq!(c1 * c2, gaussint!(2, 0));

        let c1 = gaussint!(3, 2);
        let c2 = gaussint!(2, 3);
        assert_eq!(c1 * c2, gaussint!(0, 13));

        let c1 = gaussint!(2, 1);
        let c2 = gaussint!(0, -1);
        assert_eq!(c1 * c2, gaussint!(1, -2));
    }

    #[test]
    fn division() {
        // See https://projecteuler.net/problem=153
        let c1 = gaussint!(5, 0);
        let c2 = gaussint!(1, 2);
        assert_eq!(c1 / c2, gaussint!(1, -2));
    }

    #[test]
    fn remainder() {
        let c1 = gaussint!(2, 2);
        let c2 = gaussint!(2, 2);
        assert_eq!(c1 % c2, GaussianInt::zero());

        let c1 = gaussint!(1, 2);
        let c2 = gaussint!(3, 4);
        assert!(c1 % c2 != GaussianInt::zero());
    }

    #[test]
    fn neg() {
        let z = gaussint!(2, 2);
        assert_eq!(z + -z, GaussianInt::zero());
    }

    #[test]
    fn divides() {
        let five = gaussint!(5, 0);
        assert!(gaussint!(1, 0).divides(five));
        assert!(gaussint!(1, 2).divides(five));
        assert!(gaussint!(1, -2).divides(five));
        assert!(gaussint!(2, 1).divides(five));
        assert!(gaussint!(2, -1).divides(five));
        assert!(five.divides(five));
    }

    #[test]
    fn from_isize() {
        let c = gaussint!(5, 0);
        assert_eq!(5_isize, c.into());
    }

    #[test]
    fn conjugate() {
        let c: GaussianInt<isize> = gaussint!(5, 5);
        let conj: GaussianInt<isize> = gaussint!(5, -5);
        assert_eq!(c.conj(), conj);
    }

    // See https://en.wikipedia.org/wiki/Table_of_Gaussian_integer_factorizations
    #[test]
    fn is_gaussian_prime_misc() {
        let c = gaussint!(6, 1);
        assert_eq!(c.is_gaussian_prime(), true);

        let c = gaussint!(1, 6);
        assert_eq!(c.is_gaussian_prime(), true);

        let c = gaussint!(7, 0);
        assert_eq!(c.is_gaussian_prime(), true);

        let c = gaussint!(3, 20);
        assert_eq!(c.is_gaussian_prime(), true);

        let c = gaussint!(-927, -980);
        assert_eq!(c.is_gaussian_prime(), true);

        let c = gaussint!(999, 994);
        assert_eq!(c.is_gaussian_prime(), true);
    }

    #[test]
    fn is_gaussian_prime_all_5() {
        // Test all a + bi where |a|,|b| <= 5.
        assert_eq!(gaussint!(-5, -5).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-5, -4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-5, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-5, -2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-5, -1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-5, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-5, 1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-5, 2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-5, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-5, 4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-5, 5).is_gaussian_prime(), false);

        assert_eq!(gaussint!(-4, -5).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-4, -4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-4, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-4, -2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-4, -1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-4, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-4, 1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-4, 2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-4, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-4, 4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-4, 5).is_gaussian_prime(), true);

        assert_eq!(gaussint!(-3, -5).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-3, -4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-3, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-3, -2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-3, -1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-3, 0).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-3, 1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-3, 2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-3, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-3, 4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-3, 5).is_gaussian_prime(), false);

        assert_eq!(gaussint!(-2, -5).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-2, -4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-2, -3).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-2, -2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-2, -1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-2, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-2, 1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-2, 2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-2, 3).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-2, 4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-2, 5).is_gaussian_prime(), true);

        assert_eq!(gaussint!(-1, -5).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-1, -4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-1, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-1, -2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-1, -1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-1, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-1, 1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-1, 2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-1, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(-1, 4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(-1, 5).is_gaussian_prime(), false);

        assert_eq!(gaussint!(0, -5).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, -4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, -3).is_gaussian_prime(), true);
        assert_eq!(gaussint!(0, -2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, -1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, 1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, 2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, 3).is_gaussian_prime(), true);
        assert_eq!(gaussint!(0, 4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(0, 5).is_gaussian_prime(), false);

        assert_eq!(gaussint!(1, -5).is_gaussian_prime(), false);
        assert_eq!(gaussint!(1, -4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(1, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(1, -2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(1, -1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(1, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(1, 1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(1, 2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(1, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(1, 4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(1, 5).is_gaussian_prime(), false);

        assert_eq!(gaussint!(2, -5).is_gaussian_prime(), true);
        assert_eq!(gaussint!(2, -4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(2, -3).is_gaussian_prime(), true);
        assert_eq!(gaussint!(2, -2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(2, -1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(2, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(2, 1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(2, 3).is_gaussian_prime(), true);
        assert_eq!(gaussint!(2, 4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(2, 5).is_gaussian_prime(), true);

        assert_eq!(gaussint!(3, -5).is_gaussian_prime(), false);
        assert_eq!(gaussint!(3, -4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(3, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(3, -2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(3, -1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(3, 0).is_gaussian_prime(), true);
        assert_eq!(gaussint!(3, 1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(3, 2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(3, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(3, 4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(3, 5).is_gaussian_prime(), false);

        assert_eq!(gaussint!(4, -5).is_gaussian_prime(), true);
        assert_eq!(gaussint!(4, -4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(4, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(4, -2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(4, -1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(4, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(4, 1).is_gaussian_prime(), true);
        assert_eq!(gaussint!(4, 2).is_gaussian_prime(), false);
        assert_eq!(gaussint!(4, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(4, 4).is_gaussian_prime(), false);
        assert_eq!(gaussint!(4, 5).is_gaussian_prime(), true);

        assert_eq!(gaussint!(5, -5).is_gaussian_prime(), false);
        assert_eq!(gaussint!(5, -4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(5, -3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(5, -2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(5, -1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(5, 0).is_gaussian_prime(), false);
        assert_eq!(gaussint!(5, 1).is_gaussian_prime(), false);
        assert_eq!(gaussint!(5, 2).is_gaussian_prime(), true);
        assert_eq!(gaussint!(5, 3).is_gaussian_prime(), false);
        assert_eq!(gaussint!(5, 4).is_gaussian_prime(), true);
        assert_eq!(gaussint!(5, 5).is_gaussian_prime(), false);
    }

    #[test]
    fn is_rational() {
        let c = gaussint!(7, 0);
        assert!(c.is_rational());

        let c = gaussint!(5, 1);
        assert!(!c.is_rational());
    }

    #[test]
    fn to_polar() {
        let c = gaussint!(5, 0);
        assert_eq!(c.to_polar(), (5f64, 0f64));

        let c = gaussint!(0, 1);
        assert_eq!(c.to_polar(), (1f64, std::f64::consts::PI / 2f64));

        let c = gaussint!(0, -1);
        assert_eq!(c.to_polar(), (1f64, -std::f64::consts::PI / 2f64));
    }
    #[test]
    fn congruence() {
        let c1 = gaussint!(5, 0);
        let c2 = gaussint!(1, 0);
        let c3 = gaussint!(4, 0);
        assert!(c1.congruent(c2, c3));

        // 2 + 5i ≡ i mod 1 + 2i
        let c1 = gaussint!(2, 5);
        let c2 = gaussint!(0, 1);
        let c3 = gaussint!(1, 2);
        assert!(c1.congruent(c2, c3));
    }

    #[test]
    fn is_even() {
        let z = gaussint!(4, 0);
        assert!(z.is_even());

        let z = gaussint!(-3, 1);
        assert!(z.is_even());
    }

    #[test]
    fn is_odd() {
        let z = gaussint!(1, 0);
        assert!(z.is_odd());

        let z = gaussint!(2, 1);
        assert!(z.is_odd());
    }

    #[test]
    fn units() {
        let u = GaussianInt::units();
        let mut sum = GaussianInt::zero();
        for x in u {
            sum = sum + x;
        }
        assert_eq!(gaussint!(0), sum);
    }

    #[test]
    fn associated() {
        let z1 = gaussint!(1, 0);
        let z2 = gaussint!(-1, 0);
        assert!(z1.is_associated(z2));

        // 1+i and 1-i are both prime.
        // They are also one another's conjugates *and* associates.
        //
        // See ../examples/prime-conjugate.rs
        let z1 = gaussint!(1, 1);
        let z2 = gaussint!(1, -1);
        assert!(z1.is_gaussian_prime());
        assert!(z2.is_gaussian_prime());
        assert!(z1.is_associated(z2));
        assert!(z2.is_associated(z1));

        let z1 = gaussint!(2, 1);
        let z2 = gaussint!(-1, 2);
        assert!(z1.is_associated(z2));

        let z1 = gaussint!(2, 1);
        let z2 = gaussint!(-1, -2);
        assert!(!z1.is_associated(z2));
    }
}
