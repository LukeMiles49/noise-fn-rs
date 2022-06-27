# Changelog

This project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.2.0](https://crates.io/crates/noise_fn/0.2.0) - 2022-06-27

### Updated:
* Updated to [higher_order_functions](https://crates.io/crates/higher_order_functions) v0.2.0
* Updated to [sized_matrix](https://crates.io/crates/sized_matrix) v0.3.0
* Removed unnecessary unstable features

## [v0.1.2](https://crates.io/crates/noise_fn/0.1.2) - 2020-09-12

### Added:
* `HashNoise` noise function which hashes the input to give a pseudo-random `u64`.
* `ToFloat` which converts a uint noise function to one which outputs `f64`s in the range `[0.0, 1.0)`.

### Changed:
* Switched `Simplex` to using `HashNoise` instead of `WhiteNoise` internally for slightly better randomness.

## [v0.1.1](https://crates.io/crates/noise_fn/0.1.1) - 2020-09-11

### Added:
* `AddNoise`, `Constant`, and `Gradient` noise functions.
* `IgnoreSeed`, a workaround to allow seedable and non-seedable noise functions to be used together.

## [v0.1.0](https://crates.io/crates/noise_fn/0.1.0) - 2020-09-11

### Added:
* Initial implementation
