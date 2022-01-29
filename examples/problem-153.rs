//! See https://projecteuler.net/problem=153

use gaussiant::GaussianInt;

fn main() {
    let n = 100;
    let mut big_sum = 0;

    for _n in 1..=n {
        let set = gaussiant::get_g_ints(_n);
        let mut divisors = vec![];
        let mut sum = 0;

        for z in set {
            if z != GaussianInt::zero() && z.0.re != 0 {
                if z.divides(GaussianInt::new(_n, 0)) {
                    divisors.push(z);
                    sum += z.0.re;
                    sum += z.0.im;
                    big_sum += z.0.re;
                    big_sum += z.0.im;
                }
            }
        }

        // format divisors as string
        let mut output = String::new();
        for d in divisors {
            output.push_str(&(d.to_string() + " "));
        }

        println!("{_n}: {output} {sum}");
    }

    println!("big sum: {}", big_sum);
}
