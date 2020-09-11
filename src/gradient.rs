use super::{Noise, NoiseDomain};

use sized_matrix::{Vector, Dot};

/// A noise function which returns a gradient.
///
/// `noise(x) = x.dot(direction)`
#[derive(Copy, Clone)]
pub struct Gradient<Value: Copy, const N: usize> where Vector<Value, N>: Dot<Output = Value> {
	direction: Vector<Value, N>,
}

impl<Value: Copy, const N: usize> Gradient<Value, N> where Vector<Value, N>: Dot<Output = Value> {
	pub fn new(direction: Vector<Value, N>) -> Self {
		Self { direction }
	}
}

impl<Value: Copy, const N: usize> Noise for Gradient<Value, N> where Vector<Value, N>: Dot<Output = Value> {
	type Value = Value;
	type Unseeded = Self;
}

impl<Value: Copy, const N: usize> NoiseDomain<Vector<Value, N>> for Gradient<Value, N> where Vector<Value, N>: Dot<Output = Value> {
	fn noise(&self, arg: Vector<Value, N>) -> Self::Value {
		arg.dot(self.direction)
	}
}
