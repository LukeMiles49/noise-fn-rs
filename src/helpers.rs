use super::{Noise, Seedable, noise::Seeded};

use core::marker::PhantomData;

/// A type alias for the configuration struct of a seeded type.
pub type Config<Noise> = <Noise as Seeded>::Config;

/// A helper trait which uses [`EmptyConfig`] to automatically implement seeding.
pub trait SeedOnlyNoise {
	type Seed;
	type Value;
	
	fn seed(seed: Self::Seed) -> Self;
}

/// A struct used to automatically implement seeding for [`SeedOnlyNoise`].
#[derive(Copy, Clone)]
pub struct EmptyConfig<Noise: SeedOnlyNoise> {
	_phantom: PhantomData<fn(Noise::Seed) -> Noise>,
}

impl<Noise: SeedOnlyNoise> EmptyConfig<Noise> {
	pub fn new() -> Self {
		Self { _phantom: PhantomData }
	}
}

impl<Noise: SeedOnlyNoise> Seedable for EmptyConfig<Noise> {
	type Seed = Noise::Seed;
	type Seeded = Noise;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		Self::Seeded::seed(seed)
	}
}

impl<TNoise: SeedOnlyNoise> Noise for TNoise {
	type Value = TNoise::Value;
	type Unseeded = EmptyConfig<TNoise>;
}

impl<Noise: SeedOnlyNoise> Seeded for Noise {
	type Config = EmptyConfig<Noise>;
}
