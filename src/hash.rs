use super::{NoiseDomain, helpers::{EmptyConfig, SeedOnlyNoise}};

use core::hash::{Hash, Hasher};

use wyhash::WyHash;

/// Seeded pseudorandom values using a fast non-cryptographic hash function.
#[derive(Copy, Clone)]
pub struct HashNoise {
	seed: u64,
}

impl HashNoise {
	pub fn new() -> EmptyConfig<Self> {
		EmptyConfig::new()
	}
}

impl SeedOnlyNoise for HashNoise {
	type Seed = u64;
	type Value = u64;
	
	fn seed(seed: u64) -> Self {
		Self { seed }
	}
}

impl<Arg: Hash> NoiseDomain<Arg> for HashNoise {
	fn noise(&self, arg: Arg) -> u64 {
		let mut hasher = WyHash::with_seed(self.seed);
		arg.hash(&mut hasher);
		hasher.finish()
	}
}
