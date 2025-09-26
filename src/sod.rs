
pub struct Sod {
	k1: f64,
	k2: f64,
	k3: f64,

	value: f64,
	value_vel: f64,
	x_last: f64,

	accurate: bool,
	critical: Option<f64>,
}
impl Sod {
	// f frequency
	// z bounce
	// r response
	pub fn new(f: f64, z: f64, r: f64) -> Self {
		let mut this = Self::util_base();
		this.set_weight(f, z, r);
		this
	}
	pub fn new_raw(k1: f64, k2: f64, k3: f64) -> Self {
		let mut this = Self::util_base();
		this.set_k(k1, k2, k3);
		this
	}

	fn util_base() -> Self {
		Self {
			k1: 0.0,
			k2: 0.0,
			k3: 0.0,
			value: 0.0,
			value_vel: 0.0,
			x_last: 0.0,
			accurate: false,
			critical: None,
		}
	}
	fn util_crit(&mut self) {
		self.critical = if self.accurate {
			Some(0.8 * ((4.0 * self.k2 + self.k1.powi(2)).sqrt() - self.k1))
		} else {
			None
		};
	}

	pub fn set_k(&mut self, k1: f64, k2: f64, k3: f64) {
		self.k1 = k1;
		self.k2 = k2;
		self.k3 = k3;
		self.util_crit();
	}

	pub fn set_weight(&mut self, f: f64, z: f64, r: f64) {
		use std::f64::consts::PI;
		self.k1 = z / (PI * f);
		self.k2 = 1.0 / (2.0 * PI * f.powi(2));
		self.k3 = (r * z) / (2.0 * PI * f);
		self.util_crit();
	}

	pub fn set_accuracy(&mut self, value: bool) {
		self.accurate = value;
		self.util_crit();
	}

	pub fn set_value(&mut self, value: f64) {
		self.value = value;
		self.x_last = value;
		self.value_vel = 0.0;
	}

	pub fn get_value(&self) -> f64 {
		self.value
	}

	// todo: this is buggy as hell
	pub fn update(&mut self, x: f64, x_vel: Option<f64>, dt: f64) {
		if dt <= 0.0 {
			return;// self.value;
		}

		let x_vel = if let Some(v) = x_vel {
			v
		} else {
			let v = (x - self.x_last) / dt;
			self.x_last = x;
			v
		};
		
		if self.accurate {
		    let iterations = (dt / self.critical.unwrap()).ceil() as u32;
		    let dt = dt / iterations as f64;
		    for _ in 0..iterations {
			    self.value += self.value_vel * dt;
			    let value_accel = (x + self.k3 * x_vel - self.value - self.k1 * self.value_vel) / self.k2;
				self.value_vel += dt * value_accel;
		    }
		} else {
		    self.value += self.value_vel * dt;
		    let newk2 = self.k2.max(1.1 * (dt * dt / 4.0 + dt * self.k1 / 2.0));
			let value_accel = (x + self.k3 * x_vel - self.value - self.k1 * self.value_vel) / newk2;
		    self.value_vel += dt * value_accel;
		}
	}
}

