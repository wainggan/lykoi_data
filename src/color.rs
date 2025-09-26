
/**
enum of color representations.
*/
#[derive(Debug, Clone, Copy)]
pub enum Color {
	RGB {
		r: f32,
		g: f32,
		b: f32,
	},
	OkLab {
		l: f32,
		a: f32,
		b: f32,
	},
}
impl Color {
	pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
		Self::RGB {
			r,
			g,
			b,
		}
	}
	pub fn new_oklab(l: f32, a: f32, b: f32) -> Self {
		Self::OkLab {
			l,
			a,
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
				Some(Self::new_rgb(r, g, b))
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
				Some(Self::new_rgb(r, g, b))
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
				Some(Self::new_rgb(r, g, b))
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
				Some(Self::new_rgb(r, g, b))
			},
			_ => None
		}
	}

	pub fn get_rgb(self) -> Option<(f32, f32, f32)> {
		match self {
			Self::RGB { r, g, b } => Some((r, g, b)),
			Self::OkLab { .. } => None,
		}
	}
	pub fn borrow_rgb(&self) -> Option<(&f32, &f32, &f32)> {
		match self {
			Self::RGB { r, g, b } => Some((r, g, b)),
			Self::OkLab { .. } => None,
		}
	}
	pub fn borrow_mut_rgb(&mut self) -> Option<(&mut f32, &mut f32, &mut f32)> {
		match self {
			Self::RGB { r, g, b } => Some((r, g, b)),
			Self::OkLab { .. } => None,
		}
	}

	pub fn get_oklab(self) -> Option<(f32, f32, f32)> {
		match self {
			Self::OkLab { l, a, b } => Some((l, a, b)),
			Self::RGB { .. } => None,
		}
	}
	pub fn borrow_oklab(&self) -> Option<(&f32, &f32, &f32)> {
		match self {
			Self::OkLab { l, a, b } => Some((l, a, b)),
			Self::RGB { .. } => None,
		}
	}
	pub fn borrow_mut_oklab(&mut self) -> Option<(&mut f32, &mut f32, &mut f32)> {
		match self {
			Self::OkLab { l, a, b } => Some((l, a, b)),
			Self::RGB { .. } => None,
		}
	}

	pub fn to_rgb(self) -> Self {
		match self {
			Self::OkLab { l, a, b } => {
				let l = l + 0.3963377774 * a + 0.2158037573 * b;
				let m = l - 0.1055613458 * a - 0.0638541728 * b;
				let s = l - 0.0894841775 * a - 1.2914855480 * b;

				let l = l * l * l;
				let m = m * m * m;
				let s = s * s * s;

				Self::RGB {
					r:  4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
					g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
					b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
				}
			},
			Self::RGB { .. } => self,
		}
	}
	pub fn to_oklab(self) -> Self {
		match self {
			Self::RGB { r, g, b } => {
				let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
				let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
				let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

				let l = l.powf(1.0 / 3.0);
				let m = m.powf(1.0 / 3.0);
				let s = s.powf(1.0 / 3.0);

				Self::OkLab {
					l: 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
					a: 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
					b: 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
				}
			},
			Self::OkLab { .. } => self,
		}
	}
}

#[cfg(test)]
mod test {
    use crate::color;

	#[test]
	fn test_base() {
		let c = color::Color::from_hex_str("#000").expect("parse failed");
		assert_eq!(c.get_rgb().unwrap(), (0.0, 0.0, 0.0));
		let c = color::Color::from_hex_str("#fff0").expect("parse failed");
		assert_eq!(c.get_rgb().unwrap(), (1.0, 1.0, 1.0));
		let c = color::Color::from_hex_str("#ff00ff").expect("parse failed");
		assert_eq!(c.get_rgb().unwrap(), (1.0, 0.0, 1.0));
	}
}

