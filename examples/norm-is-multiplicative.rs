use gaussiant::GaussianInt;

fn main() {
    let z = GaussianInt::new(5, 4);
    let w = GaussianInt::new(3, 2);
    assert_eq!(z.norm() * w.norm(), (z * w).norm());
    assert_eq!(z.norm() * w.norm(), (w * z).norm());
}
