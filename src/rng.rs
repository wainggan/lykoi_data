/*!
[Wichmann-Hill](https://en.wikipedia.org/wiki/Wichmann%E2%80%93Hill) psuedo-rng.
*/

pub struct Rng {
	seed0: u32,
	seed1: u32,
	seed2: u32,
}
impl Rng {
	pub fn new(seed: u32) -> Self {
		Self {
			seed0: seed,
			seed1: seed + 1,
			seed2: seed + 2,
		}
	}

	pub fn next(&mut self) -> f64 {
		self.seed0 = (self.seed0 * 171) % 30269;
		self.seed1 = (self.seed1 * 172) % 30307;
		self.seed2 = (self.seed2 * 170) % 30323;
		let x = self.seed0 as f64 / 30269.0 + self.seed1 as f64 / 30307.0 + self.seed2 as f64 / 30323.0;
		x - x.floor()
	}

	pub fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.next() * (x1 - x0)
	}
}

