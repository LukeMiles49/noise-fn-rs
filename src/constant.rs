use super::{Noise, NoiseDomain};

/// A noise function returning a constant value.
#[derive(Copy, Clone)]
pub struct Constant<Value: Copy> {
	value: Value,
}

impl<Value: Copy> Constant<Value> {
	pub fn new(value: Value) -> Self {
		Self { value }
	}
}

impl<Value: Copy> Noise for Constant<Value> {
	type Value = Value;
	type Unseeded = Self;
}

impl<Arg, Value: Copy> NoiseDomain<Arg> for Constant<Value> {
	fn noise(&self, _: Arg) -> Self::Value {
		self.value
	}
}
