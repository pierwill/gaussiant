//! See https://projecteuler.net/problem=153
#![allow(unused_imports)]

#[cfg(feature = "comfy-table")]
use comfy_table::{ColumnConstraint::*, Table, Width::*};

use gaussiant::GaussianInt;
use num_traits::Zero;

fn main() {
    let n = 1000;
    let mut big_sum = 0;

    #[cfg(feature = "comfy-table")]
    let mut table = Table::new();
    #[cfg(feature = "comfy-table")]
    {
        table.set_header(vec![
            "n",
            "Gaussian integer divisors with pos. real part",
            "Sum of parts of divisors",
        ]);
        let column = table.get_column_mut(1).expect("This should be column two");
        column.set_constraint(UpperBoundary(Fixed(50)));
        let column = table.get_column_mut(2).expect("This should be column two");
        column.set_constraint(UpperBoundary(Fixed(10)));
    }

    for _n in 1..=n {
        let set = gaussiant::get_pos_g_ints(_n);
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

        #[cfg(feature = "comfy-table")]
        table.add_row(vec![format!("{_n}"), format!("{output}"), format!("{sum}")]);
        #[cfg(not(feature = "comfy-table"))]
        println!("{_n}: {output} {sum}");
    }

    #[cfg(feature = "comfy-table")]
    println!("{table}");

    println!("big sum: {}", big_sum);
}
