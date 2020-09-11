use super::{Noise, NoiseDomain, Seedable, noise::Seeded};

use core::ops::Mul;

/// A noise function which scales both the input and output values.
#[derive(Copy, Clone)]
pub struct ScaleNoise<Inner, InScale: Copy, OutScale: Copy> {
	inner: Inner,
	scale_in: InScale,
	scale_out: OutScale,
}

impl<Inner: Noise, InScale: Copy, OutScale: Copy> Noise for ScaleNoise<Inner, InScale, OutScale> where Inner::Value: Mul<OutScale> {
	type Value = <Inner::Value as Mul<OutScale>>::Output;
	type Unseeded = ScaleNoise<Inner::Unseeded, InScale, OutScale>;
}

impl<Inner, InScale: Copy, OutScale: Copy> ScaleNoise<Inner, InScale, OutScale> {
	/// Create a noise function where `noise(x) = inner.noise(x * scale_in) * scale_out`.
	pub fn new(inner: Inner, scale_in: InScale, scale_out: OutScale) -> ScaleNoise<Inner, InScale, OutScale> {
		ScaleNoise { inner, scale_in, scale_out }
	}
}

impl<Inner: Seedable, InScale: Copy, OutScale: Copy> Seedable for ScaleNoise<Inner, InScale, OutScale> {
	type Seed = Inner::Seed;
	type Seeded = ScaleNoise<Inner::Seeded, InScale, OutScale>;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		ScaleNoise { inner: self.inner.seed(seed), scale_in: self.scale_in, scale_out: self.scale_out }
	}
}

impl<Inner: Seeded, InScale: Copy, OutScale: Copy> Seeded for ScaleNoise<Inner, InScale, OutScale> {
	type Config = ScaleNoise<Inner::Config, InScale, OutScale>;
}

impl<Arg, Inner, InScale: Copy, OutScale: Copy> NoiseDomain<Arg> for ScaleNoise<Inner, InScale, OutScale> where
	Arg: Mul<InScale>,
	Inner: NoiseDomain<Arg::Output>,
	Inner::Value: Mul<OutScale>,
{
	fn noise(&self, arg: Arg) -> Self::Value {
		self.inner.noise(arg * self.scale_in) * self.scale_out
	}
}
