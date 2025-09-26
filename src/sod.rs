
pub struct Sod {
	k1: f64,
	k2: f64,
	k3: f64,

	// most recent value
	x: f64,
	// value speed
	x_vel: f64,
	// last value
	x_last: f64,

	// cache
	crit: Option<f64>,
}
impl Sod {
	// f frequency
	// z bounce
	// r response
	pub fn new() -> Self {
		Self {
			k1: 0.0,
			k2: 0.0,
			k3: 0.0,

			x: 0.0,
			x_vel: 0.0,
			x_last: 0.0,
			
			crit: None,
		}
	}

	fn util_refresh(&mut self) {
		if let Some(ref mut crit) = self.crit {
			*crit = 0.8 * ((4.0 * self.k2 + self.k1.powi(2)).sqrt() - self.k1);
		}
	}

	pub fn set_k(&mut self, k1: f64, k2: f64, k3: f64) {
		self.k1 = k1;
		self.k2 = k2;
		self.k3 = k3;
		self.util_refresh();
	}

	pub fn set_weight(&mut self, f: f64, z: f64, r: f64) {
		use std::f64::consts::PI;
		self.k1 = z / (PI * f);
		self.k2 = 1.0 / (2.0 * PI * f.powi(2));
		self.k3 = (r * z) / (2.0 * PI * f);
		self.util_refresh();
	}

	pub fn set_accuracy(&mut self, value: bool) {
		self.crit = if value {
			Some(0.0)
		} else {
			None
		};
		self.util_refresh();
	}

	pub fn set_value(&mut self, value: f64) {
		self.x = value;
		self.x_vel = 0.0;
		self.x_last = value;
	}

	pub fn get_value(&self) -> f64 {
		self.x
	}

	// todo: this is buggy as hell
	pub fn tick(&mut self, x: f64, x_vel: Option<f64>, dt: f64) {
		if dt <= 0.0 {
			return;
		}

		let x_vel = if let Some(v) = x_vel {
			v
		} else {
			let v = (x - self.x_last) / dt;
			self.x_last = x;
			v
		};
		
		if let Some(crit) = self.crit {
		    let iterations = (dt / crit).ceil() as u32;

		    let dt = dt / iterations as f64;
			
		    for _ in 0..iterations {
			    self.x += self.x_vel * dt;

			    let x_accel = (x + self.k3 * x_vel - self.x - self.k1 * self.x_vel) / self.k2;

				self.x_vel += dt * x_accel;
		    }
		} else { // faster approximation
			self.x += self.x_vel * dt;
		    
			let k2_new = self.k2.max(1.1 * (dt * dt / 4.0 + dt * self.k1 / 2.0));

			let x_accel = (x + self.k3 * x_vel - self.x - self.k1 * self.x_vel) / k2_new;
		    
			self.x_vel += dt * x_accel;
		}
	}
}

