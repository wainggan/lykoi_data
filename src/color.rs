
/**
rgb color
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl RGB {
	#[inline]
	pub fn new(r: f32, g: f32, b: f32) -> Self {
		Self {
			r,
			g,
			b,
		}
	}

	pub fn from_hex_str(hex: &str) -> Option<Self> {
		let hex = hex.trim_start_matches("#");
		match hex.len() {
			3 => {
				let (r, hex) = hex.split_at(1);
				let (g, b) = hex.split_at(1);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let r = r as f32 * 17.0 / 255.0;
				let g = g as f32 * 17.0 / 255.0;
				let b = b as f32 * 17.0 / 255.0;
				Some(Self::new(r, g, b))
			},
			4 => {
				let (r, hex) = hex.split_at(1);
				let (g, hex) = hex.split_at(1);
				let (b, a) = hex.split_at(1);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let Ok(a)= u8::from_str_radix(a, 16) else {
					return None;
				};
				let r = r as f32 * 17.0 / 255.0;
				let g = g as f32 * 17.0 / 255.0;
				let b = b as f32 * 17.0 / 255.0;
				let _ = a as f32 * 17.0 / 255.0;
				Some(Self::new(r, g, b))
			},
			6 => {
				let (r, hex) = hex.split_at(2);
				let (g, b) = hex.split_at(2);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let r = r as f32 / 255.0;
				let g = g as f32 / 255.0;
				let b = b as f32 / 255.0;
				Some(Self::new(r, g, b))
			},
			8 => {
				let (r, hex) = hex.split_at(2);
				let (g, hex) = hex.split_at(2);
				let (b, a) = hex.split_at(2);
				let Ok(r)= u8::from_str_radix(r, 16) else {
					return None;
				};
				let Ok(g)= u8::from_str_radix(g, 16) else {
					return None;
				};
				let Ok(b)= u8::from_str_radix(b, 16) else {
					return None;
				};
				let Ok(a)= u8::from_str_radix(a, 16) else {
					return None;
				};
				let r = r as f32 / 255.0;
				let g = g as f32 / 255.0;
				let b = b as f32 / 255.0;
				let _ = a as f32 / 255.0;
				Some(Self::new(r, g, b))
			},
			_ => None
		}
	}

	#[inline]
	pub fn get(self) -> (f32, f32, f32) {
		(self.r, self.g, self.b)
	}
	#[inline]
	pub fn borrow(&self) -> (&f32, &f32, &f32) {
		(&self.r, &self.g, &self.b)
	}
	#[inline]
	pub fn borrow_mut(&mut self) -> (&mut f32, &mut f32, &mut f32) {
		(&mut self.r, &mut self.g, &mut self.b)
	}

	#[expect(clippy::excessive_precision, reason="numbers taken directly from oklab blog")]
	pub fn to_oklab(self) -> OkLab {
		let r = self.r;
		let g = self.g;
		let b = self.b;

		let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
		let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
		let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

		let l = l.powf(1.0 / 3.0);
		let m = m.powf(1.0 / 3.0);
		let s = s.powf(1.0 / 3.0);

		OkLab {
			l: 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
			a: 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
			b: 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
		}
	}
}

impl Default for RGB {
	fn default() -> Self {
		Self::new(0.0, 0.0, 0.0)
	}
}

/**
oklab color
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OkLab {
	pub l: f32,
	pub a: f32,
	pub b: f32,
}

impl OkLab {
	#[inline]
	pub fn new(l: f32, a: f32, b: f32) -> Self {
		Self {
			l,
			a,
			b,
		}
	}

	#[inline]
	pub fn get(self) -> (f32, f32, f32) {
		(self.l, self.a, self.b)
	}
	#[inline]
	pub fn borrow(&self) -> (&f32, &f32, &f32) {
		(&self.l, &self.a, &self.b)
	}
	#[inline]
	pub fn borrow_mut(&mut self) -> (&mut f32, &mut f32, &mut f32) {
		(&mut self.l, &mut self.a, &mut self.b)
	}

	#[expect(clippy::excessive_precision, reason="numbers taken directly from oklab blog")]
	pub fn to_rgb(self) -> RGB {
		let l = self.l;
		let a = self.a;
		let b = self.b;

		let l = l + 0.3963377774 * a + 0.2158037573 * b;
		let m = l - 0.1055613458 * a - 0.0638541728 * b;
		let s = l - 0.0894841775 * a - 1.2914855480 * b;

		let l = l * l * l;
		let m = m * m * m;
		let s = s * s * s;

		RGB {
			r:  4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
			g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
			b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
		}
	}
}

impl Default for OkLab {
	fn default() -> Self {
		Self::new(0.0, 0.0, 0.0)
	}
}


#[cfg(test)]
mod test {
    use crate::color;

	#[test]
	fn test_main() {
		let c = color::RGB::from_hex_str("#000").expect("parse failed");
		assert_eq!(c.get(), (0.0, 0.0, 0.0));
		let c = color::RGB::from_hex_str("#fff0").expect("parse failed");
		assert_eq!(c.get(), (1.0, 1.0, 1.0));
		let c = color::RGB::from_hex_str("#ff00ff").expect("parse failed");
		assert_eq!(c.get(), (1.0, 0.0, 1.0));
	}
}

