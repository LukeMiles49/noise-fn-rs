use super::{Noise, NoiseDomain, Seedable, noise::Seeded};

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

/// This is a workaround for allowing both seedable and non-seedable noise functions in combining types.
///
/// This will be replaced by making `Noise` and `Seedable` mutually exclusive once this is possible.
/// This will allow the combining types to distinguish between seedable and non-seedable noise functions and seed only the seedable ones.
#[derive(Copy, Clone)]
pub struct IgnoreSeed<Seed, Inner> {
	inner: Inner,
	_phantom: PhantomData<fn(Seed) -> Self>,
}

impl<Seed, Inner: Noise> Noise for IgnoreSeed<Seed, Inner> {
	type Value = Inner::Value;
	type Unseeded = Self;
}

impl<Seed, Inner> IgnoreSeed<Seed, Inner> {
	pub fn new(inner: Inner) -> Self {
		Self { inner, _phantom: PhantomData }
	}
}

impl<Seed, Inner> Seedable for IgnoreSeed<Seed, Inner> {
	type Seed = Seed;
	type Seeded = Self;
	
	fn seed(self, _: Self::Seed) -> Self {
		self
	}
}

impl<Seed, Inner> Seeded for IgnoreSeed<Seed, Inner> {
	type Config = Self;
}

impl<Arg, Seed, Inner: NoiseDomain<Arg>> NoiseDomain<Arg> for IgnoreSeed<Seed, Inner> {
	fn noise(&self, arg: Arg) -> Self::Value {
		self.inner.noise(arg)
	}
}
