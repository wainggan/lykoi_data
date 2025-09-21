/*!
this module exposes some psuedo-rng implementations.

- [WichHill]
- [XorShift32]
- [XorShift64]
- [XorShift128p]
- [FibLFSR16]
*/

/**
[Wichmann-Hill](https://en.wikipedia.org/wiki/Wichmann%E2%80%93Hill) psuedo-rng.
*/
#[derive(Debug, Clone)]
pub struct WichHill {
	seed0: u32,
	seed1: u32,
	seed2: u32,
}
impl WichHill {
	#[inline]
	pub const fn new_raw(seed0: u32, seed1: u32, seed2: u32) -> Self {
		Self {
			// ensure each seed is at least >= 1
			// hopefully this isn't slow...
			seed0: if seed0 == 0 { 1 } else { seed0 },
			seed1: if seed1 == 0 { 1 } else { seed1 },
			seed2: if seed2 == 0 { 1 } else { seed2 },
		}
	}

	#[inline]
	pub const fn new(seed: u32) -> Self {
		let mut rng = XorShift32::new(seed);
		Self::new_raw(rng.nextu(), rng.nextu(), rng.nextu())
	}

	#[inline]
	pub const fn next(&mut self) -> f64 {
		self.seed0 = (self.seed0 * 171) % 30269;
		self.seed1 = (self.seed1 * 172) % 30307;
		self.seed2 = (self.seed2 * 170) % 30323;
		let x = self.seed0 as f64 / 30269.0 + self.seed1 as f64 / 30307.0 + self.seed2 as f64 / 30323.0;
		// listen. all I'm saying is that if on a whim I added `const` to these functions and it *worked*, then I'll
		// take it, even if this is bleeding-edge Rust from like 3 days ago
		x - x.floor()
	}

	#[inline]
	pub const fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.next() * (x1 - x0)
	}
}

/**
[32bit xorshift](https://en.wikipedia.org/wiki/Xorshift) psuedo-rng.
*/
#[derive(Debug, Clone)]
pub struct XorShift32(u32);
impl XorShift32 {
	#[inline]
	pub const fn new(seed: u32) -> Self {
		Self(seed)
	}

	#[inline]
	pub const fn nextu(&mut self) -> u32 {
		let mut x = self.0;
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		self.0 = x;
		x
	}

	#[inline]
	pub const fn nextf(&mut self) -> f64 {
		self.nextu() as f64 / u32::MAX as f64
	}

	#[inline]
	pub const fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.nextf() * (x1 - x0)
	}
}

/**
[64bit xorshift](https://en.wikipedia.org/wiki/Xorshift) psuedo-rng.
*/
#[derive(Debug, Clone)]
pub struct XorShift64(u64);
impl XorShift64 {
	#[inline]
	pub const fn new(seed: u64) -> Self {
		Self(seed)
	}

	#[inline]
	pub const fn nextu(&mut self) -> u64 {
		let mut x = self.0;
		x ^= x << 13;
		x ^= x >> 7;
		x ^= x << 17;
		self.0 = x;
		x
	}

	#[inline]
	pub const fn nextf(&mut self) -> f64 {
		self.nextu() as f64 / u64::MAX as f64
	}

	#[inline]
	pub const fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.nextf() * (x1 - x0)
	}
}

/**
[128bit non-linear xorshift](https://en.wikipedia.org/wiki/Xorshift#xorshift+) psuedo-rng. yields u64 values.
*/
#[derive(Debug, Clone)]
pub struct XorShift128p(u64, u64);
impl XorShift128p {
	#[inline]
	pub const fn new_raw(seed0: u64, seed1: u64) -> Self {
		Self(seed0, seed1)
	}

	#[inline]
	pub const fn new(seed: u64) -> Self {
		let mut rng = XorShift64::new(seed);
		Self::new_raw(rng.nextu(), rng.nextu())
	}

	#[inline]
	pub const fn nextu(&mut self) -> u64 {
		let mut t: u64 = self.0;
		let s: u64 = self.1;
		self.0 = s;
		t ^= t << 23;
		t ^= t >> 18;
		t ^= s ^ (s >> 5);
		self.1 = t;
		t + s
	}

	#[inline]
	pub const fn nextf(&mut self) -> f64 {
		self.nextu() as f64 / u64::MAX as f64
	}

	#[inline]
	pub const fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.nextf() * (x1 - x0)
	}
}

/**
[16bit fibonacci linear-feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Fibonacci_LFSRs) psuedo-rng.
*/
#[derive(Debug, Clone)]
pub struct FibLFSR16(u16, u16);
impl FibLFSR16 {
	#[inline]
	pub const fn new_raw(seed0: u16, seed1: u16) -> Self {
		Self(seed0, seed1)
	}

	#[inline]
	pub const fn new(seed: u32) -> Self {
		let s = seed.to_be_bytes();
		Self::new_raw(u16::from_be_bytes([s[0], s[1]]), u16::from_be_bytes([s[2], s[3]]))
	}
	
	#[inline]
	pub const fn nextu(&mut self) -> u16 {
		self.0 = ((self.1 >> 0) ^ (self.1 >> 2) ^ (self.1 >> 3) ^ (self.1 >> 5)) & 1;
        self.1 = (self.1 >> 1) | (self.0 << 15);
		self.1
	}

	#[inline]
	pub const fn nextf(&mut self) -> f64 {
		self.nextu() as f64 / u16::MAX as f64
	}

	#[inline]
	pub const fn range(&mut self, x0: f64, x1: f64) -> f64 {
		x0 + self.nextf() * (x1 - x0)
	}
}



