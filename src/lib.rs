//! Some simple noise functions and traits to allow easy integration with your own.
//!
//! ```rust
//! use noise_fn::{Seedable, NoiseDomain, Octaves, Simplex};
//! use sized_matrix::Vector;
//!
//! let seed = 12345;
//!
//! let octaves = Octaves::<_, 4>::new(Simplex::new(), 0.5, 0.5).seed(seed);
//!
//! let value2D = octaves.noise(Vector::vector([1.2, -3.5]));
//! let value3D = octaves.noise(Vector::vector([1.2, -3.5, 2.8]));
//! ```
//!
//! To use this, add it as a dependency to your Cargo.toml:
//! ```toml
//! [dependencies]
//! noise_fn = "^0.1.0"
//! ```

#![no_std]

#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(external_doc)]

#![doc(html_root_url = "https://docs.rs/noise_fn/0.1.0")]

pub mod noise;
pub mod helpers;
pub mod perm_table;
mod white;
mod simplex;
mod scale;
mod sum;
mod octaves;

pub use {
	noise::{
		Noise,
		Seedable,
		NoiseDomain,
	},
	helpers::Config,
	white::WhiteNoise,
	simplex::Simplex,
	scale::ScaleNoise,
	sum::SumNoise,
	octaves::Octaves,
};

// Include the readme and changelog as hidden documentation so they're tested by cargo test
#[doc(include = "../README.md")]
#[doc(include = "../CHANGELOG.md")]
type _Doctest = ();
