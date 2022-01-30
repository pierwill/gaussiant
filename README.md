[![nightly](https://github.com/pierwill/gaussiant/actions/workflows/nightly.yaml/badge.svg)](https://github.com/pierwill/gaussiant/actions/workflows/nightly.yaml)
[![stable](https://github.com/pierwill/gaussiant/actions/workflows/stable.yml/badge.svg)](https://github.com/pierwill/gaussiant/actions/workflows/stable.yml)
[![documentation](https://docs.rs/gaussiant/badge.svg)](https://docs.rs/gaussiant)
[![crate](https://img.shields.io/crates/v/gaussiant.svg)](https://crates.io/crates/gaussiant)

A [Gaussian integer] is a complex number whose real and imaginary parts are both integers.

`gaussiant` provides the [`GaussianInt`] type,
which is a wrapper around [`num_complex::Complex`]
with additional methods for number theoretical computation.

## Example

If a prime number *p* is congruent to 3 modulo 4, then it is a Gaussian prime ([Wikipedia]).

```rust
use gaussiant::{GaussianInt, gaussint};

fn main() {
    let p = gaussint!(7);
    assert_eq!(
        p.congruent(gaussint!(3), gaussint!(4)),
        p.is_gaussian_prime()
    );
}
```

[`num_complex::Complex`]: https://docs.rs/num-complex/latest/num_complex/struct.Complex.html
[Gaussian integer]: https://en.wikipedia.org/wiki/Gaussian_integer
[`GaussianInt`]: https://docs.rs/gaussiant/latest/gaussiant/struct.GaussianInt.html
[Wikipedia]: https://en.wikipedia.org/wiki/Gaussian_integer#Gaussian_primes
