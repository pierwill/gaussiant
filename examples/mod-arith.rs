//! See <https://en.wikipedia.org/wiki/Modular_arithmetic#Properties>.

use gaussiant::{gaussint, GaussianInt};

fn main() {
    let a = gaussint!(2, -1);
    let b = gaussint!(2, 3);
    let n = gaussint!(1, 1);

    let a1 = a;
    let b1 = b;
    let a2 = gaussint!(4, 1);
    let b2 = gaussint!(4, -3);

    // a1 ≡ b1 (mod n)
    assert!(a.congruent(b, n));
    // a1 ≡ b1 (mod n)
    assert!(a1.congruent(b1, n));
    // a2 ≡ b2 (mod n)
    assert!(a2.congruent(b2, n));

    // If a1 ≡ b1 (mod n) and a2 ≡ b2 (mod n), or if a ≡ b (mod n), then:
    // ==================================================================

    // a1 + a2 ≡ b1 + b2 (mod n) (compatibility with addition)
    assert!((a1 + a2).congruent(b1 + b2, n));

    // a1 – a2 ≡ b1 – b2 (mod n) (compatibility with subtraction)
    assert!((a1 - a2).congruent(b1 - b2, n));

    // a + k ≡ b + k (mod n) for any integer k (compatibility with translation)
    let k = gaussint!(10);
    assert!((a + k).congruent(b + k, n));

    // k a ≡ k b (mod n) for any integer k (compatibility with scaling)
    assert!((a * k).congruent(b * k, n));

    // a1 + a2 ≡ b1 + b2 (mod n) (compatibility with addition)
    assert!((a1 + a2).congruent(b1 + b2, n));

    // a1 – a2 ≡ b1 – b2 (mod n) (compatibility with subtraction)
    assert!((a1 + a2).congruent(b1 + b2, n));

    // a1 a2 ≡ b1 b2 (mod n) (compatibility with multiplication)
    assert!((a1 * a2).congruent(b1 * b2, n));

    // a^k ≡ b^k (mod n) for any non-negative integer k (compatibility with exponentiation)
    // TODO

    // p(a) ≡ p(b) (mod n), for any polynomial p(x) with integer coefficients
    // (compatibility with polynomial evaluation)
    // TODO?
}
