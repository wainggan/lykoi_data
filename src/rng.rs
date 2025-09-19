/*!
[Wichmann-Hill](https://en.wikipedia.org/wiki/Wichmann%E2%80%93Hill) psuedo-rng.
*/

pub struct WichHill {
	seed0: u32,
	seed1: u32,
	seed2: u32,
}
impl WichHill {
	pub fn new(seed: u32) -> Self {
		Self {
			seed0: seed.max(1),
			seed1: seed + 1,
			seed2: seed + 2,
		}
	}

	pub fn new_entropy() -> Option<Self> {
		getrandom::u32().ok().map(|x| WichHill::new(x))
	}

	pub fn new_time() -> Self {
		let a = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap_or(std::time::Duration::from_millis(0x6969696969696969))
			.as_millis()
			.wrapping_pow(7)
			.wrapping_pow(5) as u32;
		WichHill::new(a)
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


pub struct XorShift64(u64);
impl XorShift64 {
	pub fn new(seed: u64) -> Self {
		Self(seed)
	}
	
	pub fn new_entropy() -> Option<Self> {
		getrandom::u64().ok().map(|x| XorShift64(x))
	}
	
	pub fn new_time() -> Self {
		let a = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap_or(std::time::Duration::from_millis(0x6969696969696969))
			.as_millis()
			.wrapping_pow(7)
			.wrapping_pow(5) as u64;
		Self(a)
	}

	pub fn next(&mut self) -> f64 {
		let mut x = self.0;
		x ^= x << 13;
		x ^= x >> 7;
		x ^= x << 17;
		self.0 = x;
		return x as f64 / u64::MAX as f64;
	}

	pub fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.next() * (x1 - x0)
	}
}



