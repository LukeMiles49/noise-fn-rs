use super::{Noise, NoiseDomain, Seedable, noise::Seeded};

use num_traits::{Unsigned, PrimInt, AsPrimitive};

/// Convert random uint values to floating point numbers in the range [0, 1).
#[derive(Copy, Clone)]
pub struct ToFloat<Inner> {
	inner: Inner,
}

impl<Inner: Noise> Noise for ToFloat<Inner> where Inner::Value: Unsigned + PrimInt + AsPrimitive<u64> {
	type Value = f64;
	type Unseeded = ToFloat<Inner::Unseeded>;
}

impl<Inner> ToFloat<Inner> {
	pub fn new(inner: Inner) -> ToFloat<Inner> {
		ToFloat { inner }
	}
}

impl<Inner: Seedable> Seedable for ToFloat<Inner> {
	type Seed = Inner::Seed;
	type Seeded = ToFloat<Inner::Seeded>;
	
	fn seed(self, seed: Self::Seed) -> Self::Seeded {
		ToFloat { inner: self.inner.seed(seed) }
	}
}

impl<Inner: Seeded> Seeded for ToFloat<Inner> {
	type Config = ToFloat<Inner::Config>;
}

impl<Arg, Inner: NoiseDomain<Arg>> NoiseDomain<Arg> for ToFloat<Inner> where Inner::Value: Unsigned + PrimInt + AsPrimitive<u64> {
	fn noise(&self, arg: Arg) -> Self::Value {
		bits_to_f64(self.inner.noise(arg))
	}
}

fn bits_to_f64<T: Unsigned + PrimInt + AsPrimitive<u64>>(value: T) -> f64 {
	if value.is_zero() { 0.0 }
	else {
		let shift = value.leading_zeros();
		let exponent = 1022 - shift;
		let fraction = value.unsigned_shl(shift).unsigned_shl(1).swap_bytes().as_().swap_bytes().unsigned_shr(12);
		f64::from_bits((u64::from(exponent) << 52) | fraction)
	}
}
