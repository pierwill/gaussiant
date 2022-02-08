//! If a rational prime *p* is congruent to 1 modulo 4, then it is the product of a Gaussian
//! prime *q* and its conjugate, both of which are non-associated Gaussian primes
//! (neither is the product of the other by a unit).
//!
//! This code finds such primes and their factors. For more information on the
//! details, see [this page].
//!
//! [this page]: https://en.wikipedia.org/wiki/Table_of_Gaussian_integer_factorizations
use gaussiant::GaussianInt;
use primal::Primes;

const MAX: usize = usize::pow(10, 5);

fn main() {
    let primes: Vec<usize> = Primes::all()
        .take_while(|p| *p < MAX)
        .into_iter()
        .collect::<Vec<_>>();

    // get primes p such that p ≡ 1 mod 4
    let mut primes_1_mod_4: Vec<usize> = vec![];
    for prime in primes.iter() {
        if (prime - 1) % 4 == 0 {
            primes_1_mod_4.push(*prime);
        }
    }

    // find q
    for p in primes {
        let upper_bound = (p as f64).sqrt().floor() + 1.0;
        let set: Vec<_> = gaussiant::get_pos_g_ints(upper_bound as isize).collect();

        for z in &set {
            let conditions = !z.is_associated(z.conj())
                && z.0.re.is_positive()
                && z.0.im.is_positive()
                && z.0.re.abs() > z.0.im.abs();
            if GaussianInt::new(p as isize, 0) == *z * z.conj() && conditions {
                println!("{p} = {z} * {}", z.conj());
            }
        }
    }
}
