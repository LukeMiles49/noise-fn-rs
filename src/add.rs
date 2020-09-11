use super::{Noise, NoiseDomain, Seedable, noise::{Seeded, SplitSeed}};

use core::ops::Add;

/// A noise function which sums two other noise functions.
///
/// `noise(x) = inner1.noise(x) + inner2.noise(x)`
#[derive(Copy, Clone)]
pub struct AddNoise<Inner1, Inner2> {
	inner1: Inner1,
	inner2: Inner2,
}

impl<Inner1: Noise, Inner2: Noise> Noise for AddNoise<Inner1, Inner2> where Inner1::Value: Add<Inner2::Value> {
	type Value = <Inner1::Value as Add<Inner2::Value>>::Output;
	type Unseeded = AddNoise<Inner1::Unseeded, Inner2::Unseeded>;
}

impl<Inner1, Inner2> AddNoise<Inner1, Inner2> {
	pub fn new(inner1: Inner1, inner2: Inner2) -> AddNoise<Inner1, Inner2> {
		AddNoise { inner1, inner2 }
	}
}

impl<Seed: SplitSeed, Inner1: Seedable<Seed = Seed>, Inner2: Seedable<Seed = Seed>> Seedable for AddNoise<Inner1, Inner2> {
	type Seed = Seed;
	type Seeded = AddNoise<Inner1::Seeded, Inner2::Seeded>;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		AddNoise { inner1: self.inner1.seed(seed.split(0)), inner2: self.inner2.seed(seed.split(1)) }
	}
}

/*
TODO: Implement Seedable for partially seedable AddNoise types once traits can be mutually exclusive.

impl<Inner1: Seedable, Inner2: Noise> Seedable for AddNoise<Inner1, Inner2> {
	type Seed = Inner1::Seed;
	type Seeded = AddNoise<Inner1::Seeded, Inner2>;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		AddNoise { inner1: self.inner1.seed(seed), inner2: self.inner2 }
	}
}

impl<Inner1: Noise, Inner2: Seedable> Seedable for AddNoise<Inner1, Inner2> {
	type Seed = Inner2::Seed;
	type Seeded = AddNoise<Inner1, Inner2::Seeded>;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		AddNoise { inner1: self.inner1, inner2: self.inner2.seed(seed) }
	}
}
*/

impl<Seed: SplitSeed, Inner1: Seeded, Inner2: Seeded> Seeded for AddNoise<Inner1, Inner2> where
	Inner1::Config: Seedable<Seed = Seed>,
	Inner2::Config: Seedable<Seed = Seed>,
{
	type Config = AddNoise<Inner1::Config, Inner2::Config>;
}

impl<Arg: Copy, Inner1: NoiseDomain<Arg>, Inner2: NoiseDomain<Arg>> NoiseDomain<Arg> for AddNoise<Inner1, Inner2> where
	Inner1::Value: Add<Inner2::Value>,
{
	fn noise(&self, arg: Arg) -> Self::Value {
		self.inner1.noise(arg) + self.inner2.noise(arg)
	}
}
