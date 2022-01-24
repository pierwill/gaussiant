use gaussiant::GaussianInt;

const MAX: i32 = 1000;

fn main() {
    for a in 0..MAX {
        for b in 0..MAX {
            if GaussianInt::new(a, b).is_gaussian_prime() {
                println!("{}+{}i is prime", a, b);
            }
        }
    }
}
