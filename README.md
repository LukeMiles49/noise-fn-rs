# noise_fn

[Crate](https://crates.io/crates/noise_fn)

[Documentation](https://docs.rs/noise_fn)

[Repository](https://github.com/LukeMiles49/noise-fn-rs)

[Changelog](https://github.com/LukeMiles49/noise-fn-rs/blob/master/CHANGELOG.md)

Some simple noise functions and traits to allow easy integration with your own.

```rust
use noise_fn::{Seedable, NoiseDomain, Octaves, Simplex};
use sized_matrix::Vector;

let seed = 12345;

let octaves = Octaves::<_, 4>::new(Simplex::new(), 0.5, 0.5).seed(seed);

let value2D = octaves.noise(Vector::vector([1.2, -3.5]));
let value3D = octaves.noise(Vector::vector([1.2, -3.5, 2.8]));
```

To use this, add it as a dependency to your Cargo.toml:
```toml
[dependencies]
noise_fn = "^0.1.0"
```
