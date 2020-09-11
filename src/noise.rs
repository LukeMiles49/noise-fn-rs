use core::hash::{Hash, Hasher, SipHasher};

/// A noise function producing `Value`s.
pub trait Noise: Sized {
	type Value;
	type Unseeded = Self;
}

/// A 'configuration' struct which can be seeded with a `Seed` to create an instance of the type `Seeded`.
pub trait Seedable: Sized {
	type Seed;
	type Seeded: Seeded<Config = Self>;
	
	/// Seed this with `seed` to get an instance of `Seeded`.
	fn seed(self, seed: Self::Seed) -> Self::Seeded;
}

/// A struct which can be created by seeding an instance of `Config`.
pub trait Seeded: Sized {
	type Config: Seedable<Seeded = Self>;
}

/// A domain over which a noise function operates.
pub trait NoiseDomain<Arg>: Noise {
	/// Get the value of the noise at a particular 'location'.
	fn noise(&self, arg: Arg) -> Self::Value;
}

/// A type of seed which can be split into multiple 'child' seeds.
pub trait SplitSeed {
	/// Create a unique child seed.
	fn split(&self, n: usize) -> Self;
}

impl SplitSeed for u64 {
	fn split(&self, n: usize) -> Self {
		// FIXME: Move to a non-deprecated Hasher once there's one in core.
		let mut hasher = SipHasher::new();
		self.hash(&mut hasher);
		n.hash(&mut hasher);
		hasher.finish()
	}
}
