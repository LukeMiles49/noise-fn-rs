use rand::{
	rngs::StdRng,
	Rng,
	SeedableRng,
};

/// A table consisting of a permutation of the numbers `0..=255`;
#[derive(Copy, Clone)]
pub struct PermTable(pub [u8; 256]);

impl PermTable {
	pub fn new(seed: u64) -> PermTable {
		let mut rng = StdRng::seed_from_u64(seed);
		let mut table = [0u8; 256];
		for i in 1..=255 {
			let j =
				if i < 255 { rng.gen_range(0, i + 1) }
				else { let mut j = [0; 1]; rng.fill(&mut j); j[0] };
			table[i as usize] = table[j as usize];
			table[j as usize] = i;
		}
		PermTable(table)
	}
}
