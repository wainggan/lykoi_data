/*!
[Wichmann-Hill](https://en.wikipedia.org/wiki/Wichmann%E2%80%93Hill) psuedo-rng.
*/

pub struct WichHill {
	seed0: u32,
	seed1: u32,
	seed2: u32,
}
impl WichHill {
	pub fn new_raw(seed0: u32, seed1: u32, seed2: u32) -> Self {
		Self {
			seed0: seed0.max(1),
			seed1: seed1.max(1),
			seed2: seed2.max(1),
		}
	}

	pub fn new(seed: u32) -> Self {
		let mut rng = XorShift32::new(seed);
		Self::new_raw(rng.nextu(), rng.nextu(), rng.nextu())
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

pub struct XorShift32(u32);
impl XorShift32 {
	pub fn new(seed: u32) -> Self {
		Self(seed)
	}

	pub fn nextu(&mut self) -> u32 {
		let mut x = self.0;
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		self.0 = x;
		x
	}

	pub fn nextf(&mut self) -> f64 {
		self.nextu() as f64 / u32::MAX as f64
	}

	pub fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.nextf() * (x1 - x0)
	}
}

pub struct XorShift64(u64);
impl XorShift64 {
	pub fn new(seed: u64) -> Self {
		Self(seed)
	}

	pub fn nextu(&mut self) -> u64 {
		let mut x = self.0;
		x ^= x << 13;
		x ^= x >> 7;
		x ^= x << 17;
		self.0 = x;
		x
	}

	pub fn nextf(&mut self) -> f64 {
		self.nextu() as f64 / u64::MAX as f64
	}

	pub fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.nextf() * (x1 - x0)
	}
}

