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
//! noise_fn = "^0.1.2"
//! ```

#![no_std]

#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(external_doc)]

#![doc(html_root_url = "https://docs.rs/noise_fn/0.1.2")]

pub mod noise;
pub mod helpers;
pub mod perm_table;
mod white;
mod hash;
mod simplex;
mod scale;
mod add;
mod constant;
mod gradient;
mod sum;
mod octaves;
mod to_float;

pub use {
	noise::{
		Noise,
		Seedable,
		NoiseDomain,
	},
	helpers::Config,
	white::WhiteNoise,
	hash::HashNoise,
	simplex::Simplex,
	scale::ScaleNoise,
	add::AddNoise,
	constant::Constant,
	gradient::Gradient,
	sum::SumNoise,
	octaves::Octaves,
	to_float::ToFloat,
};

// Include the readme and changelog as hidden documentation so they're tested by cargo test
#[doc(include = "../README.md")]
#[doc(include = "../CHANGELOG.md")]
type _Doctest = ();
