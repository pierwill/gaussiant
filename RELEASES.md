## unreleased

- Add `get_pos_g_ints` function to return iterator of Gaussian integers with positive real parts.

## v0.6.0 (2022-01-31)

- Implement `num_traits::One` and `num_traits::Zero` for `GaussianInt`.
- Require integer types of `Gaussiant<T>` to implement [`num_integer::Integer`].
- Implement assignment operators in [`num_traits::NumAssignOps`].

[`num_integer::Integer`]: https://docs.rs/num-integer/latest/num_integer/trait.Integer.html
[`num_traits::NumAssignOps`]: https://docs.rs/num-traits/latest/num_traits/trait.NumAssignOps.html

## v0.5.1 (2022-01-30)

- Add new example to show basic plotting.
- Edit documentation.

## v0.5.0 (2022-01-29)

- Use [`primal`](https://crates.io/crates/primal) in `is_gaussian_prime` algorithm.
  Much, much faster!
- Added a new example `prime1mod4` to compute a certain kind of Gaussian prime.
- Rename `is_divisor_of` method to `divides`.
- Add `units` function to return ±1, ±*i*.
- Add `is_associated` method.

## v0.4.0 (2022-01-28)

<!-- Releasing software is fun! -->

Add `gaussint` macro for creating new `GaussianInt`s.

## v0.3.1 (2022-01-28)

Fix the status badge on crates.io.

## v0.3.0 (2022-01-28)

- Add a method to test for congruence between two Gaussian integers, modulo a third.
- Add methods to test for "even" and "odd" Gaussian integers.

## Earlier

Lost to time...
