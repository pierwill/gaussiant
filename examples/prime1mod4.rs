//! If a rational prime *p* is congruent to 1 modulo 4, then it is the product of a Gaussian
//! prime *q* and its conjugate, both of which are non-associated Gaussian primes
//! (neither is the product of the other by a unit).
//!
//! This code finds such primes and their factors. For more information on the
//! details, see [this page].
//!
//! [this page]: https://en.wikipedia.org/wiki/Table_of_Gaussian_integer_factorizations
use gaussiant::{gaussint, GaussianInt};
use primal::Primes;

const MAX: usize = 1000;

fn main() {
    let primes = Primes::all().take_while(|p| *p < MAX);

    // convert `usize`d primes to `GaussianInt`s
    let primes: Vec<GaussianInt<isize>> = primes
        .into_iter()
        .map(|p| gaussint!(p.try_into().unwrap()))
        .collect::<Vec<_>>();

    // get rational primes p such that p ≡ 1 mod 4
    let mut primes_1_mod_4: Vec<GaussianInt<isize>> = vec![];
    for prime in primes.iter() {
        if prime.congruent(gaussint!(1), gaussint!(4)) {
            primes_1_mod_4.push(*prime);
        }
    }

    // find q
    let set: Vec<_> = gaussiant::get_g_ints(MAX as isize).collect();
    for p in primes {
        for z in &set {
            let pos_real = z.0.re.is_positive();
            let pos_im = z.0.im.is_positive();
            let re_gt_im = z.0.re.abs() > z.0.im.abs();
            let conj_non_asso = !z.is_associated(z.conj());

            let conditions = pos_real && pos_im && re_gt_im && conj_non_asso;
            if p == *z * z.conj() && conditions {
                println!("{p} = {z} * {}", z.conj());
            }
        }
    }
}
