use super::{NoiseDomain, perm_table::PermTable, helpers::{EmptyConfig, SeedOnlyNoise}};

use sized_matrix::Vector;

/// Seeded pseudorandom bytes using a permutation table.
#[derive(Copy, Clone)]
pub struct WhiteNoise {
	perm_table: PermTable,
}

impl WhiteNoise {
	pub fn new() -> EmptyConfig<Self> {
		EmptyConfig::new()
	}
}

impl SeedOnlyNoise for WhiteNoise {
	type Seed = u64;
	type Value = u8;
	
	fn seed(seed: u64) -> Self {
		Self {
			perm_table: PermTable::new(seed)
		}
	}
}

impl<const N: usize> NoiseDomain<Vector<u8, N>> for WhiteNoise {
	fn noise(&self, pos: Vector<u8, N>) -> u8 {
		let mut acc: u8 = 0;
		for i in 0..N {
			acc = self.perm_table.0[acc.wrapping_add(pos[i]) as usize];
		}
		acc
	}
}
