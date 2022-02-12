//! If a rational prime *p* is congruent to 1 modulo 4, then it is the product of a Gaussian
//! prime *q* and its conjugate, both of which are non-associated Gaussian primes
//! (neither is the product of the other by a unit).
//!
//! This code finds such primes and their factors.
//!
//! Run in release mode:
//!
//! ```
//! cargo run --example prime1mod4 --release
//! ```
use gaussiant::GaussianInt;
use num_integer::Integer;
use primal::Primes;

const MAX: usize = usize::pow(10, 6);

fn main() {
    let primes_1_mod_4: Vec<usize> = Primes::all()
        .take_while(|p| *p < MAX)
        .into_iter()
        .filter(|p| (p - 1) % 4 == 0)
        .collect();

    // find q
    for p in primes_1_mod_4 {
        let sqrt_p = (p as f64).sqrt().floor();
        let upper_bound = sqrt_p + 1.0;
        let lower_bound = sqrt_p / 2.0;
        let possible_qs: Vec<_> =
            get_possible_qs(lower_bound as isize, upper_bound as isize).collect();

        for z in &possible_qs {
            let conditions = !z.is_associated(z.conj()) && z.0.re > z.0.im;
            if GaussianInt::new(p as isize, 0) == *z * z.conj() && conditions {
                println!("{p} = {z} * {}", z.conj());
            }
        }
    }
}

fn get_possible_qs(l: isize, u: isize) -> impl Iterator<Item = GaussianInt<isize>> + 'static {
    let mut possible_zs: Vec<GaussianInt<_>> = vec![];
    for a in l..=u {
        // Since q must be a Gaussian prime, a and b must have opposite parities.
        let start = if a.is_even() { 1 } else { 0 };
        for b in (start..u).step_by(2) {
            let z = GaussianInt::new(a, b);
            possible_zs.push(z);
        }
    }
    possible_zs.into_iter()
}
