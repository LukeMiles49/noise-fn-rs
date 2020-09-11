use super::{Noise, NoiseDomain, Seedable, noise::{Seeded, SplitSeed}, SumNoise, ScaleNoise};

use core::ops::{Mul, AddAssign};
use num_traits::Zero;
use higher_order_functions::Init;

/// A noise function formed by summing several octaves of another noise function.
#[derive(Copy, Clone)]
pub struct Octaves<Inner, const N: usize> {
	inner: SumNoise<ScaleNoise<Inner, f64, f64>, N>,
}

impl<Inner: Noise, const N: usize> Noise for Octaves<Inner, N> where
	Inner::Value: Mul<f64>,
	<Inner::Value as Mul<f64>>::Output: Zero + AddAssign,
{
	type Value = <SumNoise<ScaleNoise<Inner, f64, f64>, N> as Noise>::Value;
	type Unseeded = Octaves<Inner::Unseeded, N>;
}

impl<Inner: Copy, const N: usize> Octaves<Inner, N> {
	/// Create a noise function by summing several octaves of another noise function.
	///
	/// `lacunarity`: A multiplier for the frequency from one octave to the next.
	///
	/// `persistence`: A multiplier for the persistence from one octave to the next.
	///
	/// A good default is `Octaves::new(_, 0.5, 0.5)`.
	pub fn new(inner: Inner, lacunarity: f64, persistence: f64) -> Octaves<Inner, N> {
		Octaves { inner: SumNoise::new(<[_; N]>::init(|i| ScaleNoise::new(inner, lacunarity.powi(i as i32), persistence.powi(i as i32)))) }
	}
}

impl<Inner: Seedable, const N: usize> Seedable for Octaves<Inner, N> where
	Inner::Seed: SplitSeed,
{
	type Seed = <SumNoise<ScaleNoise<Inner, f64, f64>, N> as Seedable>::Seed;
	type Seeded = Octaves<Inner::Seeded, N>;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		Octaves { inner: self.inner.seed(seed) }
	}
}

impl<Inner: Seeded, const N: usize> Seeded for Octaves<Inner, N> where
	<Inner::Config as Seedable>::Seed: SplitSeed,
{
	type Config = Octaves<Inner::Config, N>;
}

impl<Arg: Copy, Inner: Noise, const N: usize> NoiseDomain<Arg> for Octaves<Inner, N> where
	Arg: Mul<f64>,
	Inner: NoiseDomain<Arg::Output>,
	Inner::Value: Mul<f64>,
	<Inner::Value as Mul<f64>>::Output: Zero + AddAssign,
{
	fn noise(&self, arg: Arg) -> Self::Value {
		self.inner.noise(arg)
	}
}
