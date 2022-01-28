// If p is congruent to 1 modulo 4, then it is the product of a Gaussian
// prime by its conjugate, both of which are non-associated Gaussian primes
// (neither is the product of the other by a unit).
use gaussiant::{gaussint, GaussianInt};

fn main() {
    let p = gaussint!(5);

    // 5 is congruent to 1 modulo 4.
    assert!(p.congruent(gaussint!(1), gaussint!(4)));

    // find q
    let set = gaussiant::get_g_ints(10);
    let mut possible_qs = vec![];

    for z in set {
        if p == z * z.conj() {
            possible_qs.push(z);

            println!("z = {z}\n====");
            println!("z *  1 = {}", z * gaussint!(1, 0));
            println!("z * -1 = {}", z * gaussint!(-1, 0));
            println!("z *  i = {}", z * gaussint!(0, 1));
            println!("z * -i = {}", z * gaussint!(0, -1));
            println!("\n");
        }
    }

    for q in &possible_qs {
        for u in GaussianInt::units() {
            &possible_qs.retain(|q| {
                let delete = *q * u == q.conj();
                delete
            })
        }
        println!("{possible_qs:?}");
    }
}
