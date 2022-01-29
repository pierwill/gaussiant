use gaussiant::GaussianIntSigns;

fn main() {
    let set = gaussiant::get_g_ints(100, GaussianIntSigns::BothPos);
    for z in set {
        if z.is_gaussian_prime() {
            println!("{z} is prime");
        }
    }
}
