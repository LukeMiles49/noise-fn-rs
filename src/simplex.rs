use super::{NoiseDomain, HashNoise, helpers::{SeedOnlyNoise, EmptyConfig}};

use sized_matrix::{Vector, Dot};

use higher_order_functions::{Map, Zip, Section};

use lazy_static::lazy_static;

/// A simplex noise function loosely based on Stefan Gustavson's open source implementation.
#[derive(Copy, Clone)]
pub struct Simplex {
	inner: HashNoise,
}

impl Simplex {
	pub fn new() -> EmptyConfig<Self> {
		EmptyConfig::new()
	}
}

impl SeedOnlyNoise for Simplex {
	type Seed = u64;
	type Value = f64;
	
	fn seed(seed: u64) -> Self {
		Self {
			inner: HashNoise::seed(seed)
		}
	}
}

// FIXME: Change to const when sqrt is supported

fn f(n: u32) -> f64 {
	let n = n as f64;
	((n + 1.).sqrt() - 1.) / n
}

fn g(n: u32) -> f64 {
	let n = n as f64;
	(n + 1. - (n + 1.).sqrt()) / (n * (n + 1.))
}

lazy_static! {
	static ref F2: f64 = f(2);
	static ref G2: f64 = g(2);
	static ref F3: f64 = f(3);
	static ref G3: f64 = g(3);
	static ref F4: f64 = f(4);
	static ref G4: f64 = g(4);
	
	static ref GRAD3: [Vector<f64, 3>; 12] = [
		[1., 1., 0.], [-1., 1., 0.], [1., -1., 0.], [-1., -1., 0.],
		[1., 0., 1.], [-1., 0., 1.], [1., 0., -1.], [-1., 0., -1.],
		[0., 1., 1.], [0., -1., 1.], [0., 1., -1.], [0., -1., -1.],
	].map(Vector::vector);
	
	static ref GRAD4: [Vector<f64, 4>; 32] = [
		[0., 1., 1., 1.], [0., 1., 1., -1.], [0., 1., -1., 1.], [0., 1., -1., -1.],
		[0., -1., 1., 1.], [0., -1., 1., -1.], [0., -1., -1., 1.], [0., -1., -1., -1.],
		[1., 0., 1., 1.], [1., 0., 1., -1.], [1., 0., -1., 1.], [1., 0., -1., -1.],
		[-1., 0., 1., 1.], [-1., 0., 1., -1.], [-1., 0., -1., 1.], [-1., 0., -1., -1.],
		[1., 1., 0., 1.], [1., 1., 0., -1.], [1., -1., 0., 1.], [1., -1., 0., -1.],
		[-1., 1., 0., 1.], [-1., 1., 0., -1.], [-1., -1., 0., 1.], [-1., -1., 0., -1.],
		[1., 1., 1., 0.], [1., 1., -1., 0.], [1., -1., 1., 0.], [1., -1., -1., 0.],
		[-1., 1., 1., 0.], [-1., 1., -1., 0.], [-1., -1., 1., 0.], [-1., -1., -1., 0.],
	].map(Vector::vector);
}

fn simplex_factor_2(inner: &HashNoise, base: Vector<i64, 2>, rel: Vector<f64, 2>, base_offset: Vector<i64, 2>, rel_offset: f64) -> f64 {
	let base = base + base_offset;
	let rel = rel.zip_with(base_offset, |r, o| r - o as f64 + rel_offset);
	
	let t = 0.5 - rel.dot(rel);
	if t < 0. { 0.0 }
	else {
		let p = inner.noise(base) % 12;
		t.powi(4) * rel.dot(GRAD3[p as usize].section(0))
	}
}

impl NoiseDomain<Vector<f64, 2>> for Simplex {
	fn noise(&self, pos: Vector<f64, 2>) -> f64 {
		let s = (pos[0] + pos[1]) * *F2;
		let base = pos.map(|x| (x + s).floor());
		
		let t = (base[0] + base[1]) * *G2;
		let rel = pos - base + Vector::vector([t, t]);
		
		let base = base.map(|x| x as i64);
		
		return 70.0 * (
			simplex_factor_2(&self.inner, base, rel, Vector::vector([0, 0]), 0.) +
			simplex_factor_2(&self.inner, base, rel,
				if rel[0] >= rel[1] {
					Vector::vector([1, 0])
				} else {
					Vector::vector([0, 1])
				}, *G2) +
			simplex_factor_2(&self.inner, base, rel, Vector::vector([1, 1]), 2. * *G2)
		)
	}
}

fn simplex_factor_3(inner: &HashNoise, base: Vector<i64, 3>, rel: Vector<f64, 3>, base_offset: Vector<i64, 3>, rel_offset: f64) -> f64 {
	let base = base + base_offset;
	let rel = rel.zip_with(base_offset, |r, o| r - o as f64 + rel_offset);
	
	let t = 0.6 - rel.dot(rel);
	if t < 0. { 0.0 }
	else {
		let p = inner.noise(base) % 12;
		t.powi(4) * rel.dot(GRAD3[p as usize])
	}
}

impl NoiseDomain<Vector<f64, 3>> for Simplex {
	fn noise(&self, pos: Vector<f64, 3>) -> f64 {
		let s = (pos[0] + pos[1] + pos[2]) * *F3;
		let base = pos.map(|x| (x + s).floor());
		
		let t = (base[0] + base[1] + base[2]) * *G3;
		let rel = pos - base + Vector::vector([t, t, t]);
		
		let base = base.map(|x| x as i64);
		
		return 32.0 * (
			simplex_factor_3(&self.inner, base, rel, Vector::vector([0, 0, 0]), 0.) +
			simplex_factor_3(&self.inner, base, rel,
				if rel[0] >= rel[1] && rel[0] >= rel[2] {
					Vector::vector([1, 0, 0])
				} else if rel[1] >= rel[0] && rel[1] >= rel[2] {
					Vector::vector([0, 1, 0])
				} else {
					Vector::vector([0, 0, 1])
				}, *G3) +
			simplex_factor_3(&self.inner, base, rel,
				if rel[0] >= rel[2] && rel[1] >= rel[2] {
					Vector::vector([1, 1, 0])
				} else if rel[0] >= rel[1] && rel[2] >= rel[1] {
					Vector::vector([1, 0, 1])
				} else {
					Vector::vector([0, 1, 1])
				}, 2. * *G3) +
			simplex_factor_3(&self.inner, base, rel, Vector::vector([1, 1, 1]), 3. * *G3)
		)
	}
}

fn simplex_factor_4(inner: &HashNoise, base: Vector<i64, 4>, rel: Vector<f64, 4>, base_offset: Vector<i64, 4>, rel_offset: f64) -> f64 {
	let base = base + base_offset;
	let rel = rel.zip_with(base_offset, |r, o| r - o as f64 + rel_offset);
	
	let t = 0.6 - rel.dot(rel);
	if t < 0. { 0.0 }
	else {
		let p = inner.noise(base) % 32;
		t.powi(4) * rel.dot(GRAD4[p as usize])
	}
}

impl NoiseDomain<Vector<f64, 4>> for Simplex {
	fn noise(&self, pos: Vector<f64, 4>) -> f64 {
		let s = (pos[0] + pos[1] + pos[2] + pos[3]) * *F4;
		let base = pos.map(|x| (x + s).floor());
		
		let t = (base[0] + base[1] + base[2] + base[3]) * *G4;
		let rel = pos - base + Vector::vector([t, t, t, t]);
		
		let base = base.map(|x| x as i64);
		
		return 27.0 * (
			simplex_factor_4(&self.inner, base, rel, Vector::vector([0, 0, 0, 0]), 0.) +
			simplex_factor_4(&self.inner, base, rel,
				if rel[0] >= rel[1] && rel[0] >= rel[2] && rel[0] >= rel[3] {
					Vector::vector([1, 0, 0, 0])
				} else if rel[1] >= rel[0] && rel[1] >= rel[2] && rel[1] >= rel[3] {
					Vector::vector([0, 1, 0, 0])
				} else if rel[2] >= rel[0] && rel[2] >= rel[1] && rel[2] >= rel[3] {
					Vector::vector([0, 0, 1, 0])
				} else {
					Vector::vector([0, 0, 0, 1])
				}, *G4) +
			simplex_factor_4(&self.inner, base, rel,
				if rel[0] >= rel[2] && rel[0] >= rel[3] && rel[1] >= rel[2] && rel[1] >= rel[3] {
					Vector::vector([1, 1, 0, 0])
				} else if rel[0] >= rel[1] && rel[0] >= rel[3] && rel[2] >= rel[1] && rel[2] >= rel[3] {
					Vector::vector([1, 0, 1, 0])
				} else if rel[0] >= rel[2] && rel[0] >= rel[1] && rel[3] >= rel[2] && rel[3] >= rel[1] {
					Vector::vector([1, 0, 0, 1])
				} else if rel[2] >= rel[0] && rel[2] >= rel[3] && rel[1] >= rel[0] && rel[1] >= rel[3] {
					Vector::vector([0, 1, 1, 0])
				} else if rel[3] >= rel[2] && rel[3] >= rel[0] && rel[1] >= rel[2] && rel[1] >= rel[0] {
					Vector::vector([0, 1, 0, 1])
				} else {
					Vector::vector([0, 0, 1, 1])
				}, 2. * *G4) +
			simplex_factor_4(&self.inner, base, rel,
				if rel[0] >= rel[3] && rel[1] >= rel[3] && rel[2] >= rel[3] {
					Vector::vector([1, 1, 1, 0])
				} else if rel[0] >= rel[2] && rel[1] >= rel[2] && rel[3] >= rel[2] {
					Vector::vector([1, 1, 0, 1])
				} else if rel[0] >= rel[1] && rel[2] >= rel[1] && rel[3] >= rel[1] {
					Vector::vector([1, 0, 1, 1])
				} else {
					Vector::vector([0, 1, 1, 1])
				}, 3. * *G4) +
			simplex_factor_4(&self.inner, base, rel, Vector::vector([1, 1, 1, 1]), 4. * *G4)
		)
	}
}
