use super::{Noise, NoiseDomain, Seedable, noise::{Seeded, SplitSeed}};

use core::ops::AddAssign;
use num_traits::Zero;
use higher_order_functions::{Zip, Init};

/// A noise function formed by summing an array of other noise functions.
#[derive(Copy, Clone)]
pub struct SumNoise<Inner, const N: usize> {
	inners: [Inner; N],
}

impl<Inner: Noise, const N: usize> Noise for SumNoise<Inner, N> where Inner::Value: Zero + AddAssign {
	type Value = Inner::Value;
	type Unseeded = SumNoise<Inner::Unseeded, N>;
}

impl<Inner, const N: usize> SumNoise<Inner, N> {
	pub fn new(inners: [Inner; N]) -> SumNoise<Inner, N> {
		SumNoise { inners }
	}
}

impl<Inner: Seedable, const N: usize> Seedable for SumNoise<Inner, N> where Inner::Seed: SplitSeed {
	type Seed = Inner::Seed;
	type Seeded = SumNoise<Inner::Seeded, N>;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		SumNoise { inners: self.inners.zip(Init::init(|i| i), |c, i| c.seed(seed.split(i))) }
	}
}

impl<Inner: Seeded, const N: usize> Seeded for SumNoise<Inner, N> where <Inner::Config as Seedable>::Seed: SplitSeed {
	type Config = SumNoise<Inner::Config, N>;
}

impl<Arg: Copy, Inner: NoiseDomain<Arg>, const N: usize> NoiseDomain<Arg> for SumNoise<Inner, N> where Inner::Value: Zero + AddAssign {
	fn noise(&self, arg: Arg) -> Self::Value {
		let mut sum = Inner::Value::zero();
		for i in 0..N {
			sum += self.inners[i].noise(arg);
		}
		sum
	}
}
