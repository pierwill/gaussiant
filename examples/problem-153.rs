//! See https://projecteuler.net/problem=153

use gaussiant::GaussianInt;

fn main() {
    let n = 5;
    let set = gaussiant::get_g_ints(n);
    for x in set {
        if x != GaussianInt::zero() && x.0.re != 0 {
            if x.is_divisor_of(GaussianInt::new(n, 0)) {
                println!("{x}");
            }
        }
    }
}
