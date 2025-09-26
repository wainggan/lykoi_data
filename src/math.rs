/*!
math utilities.
*/

/**
horrendous general number trait. for implementing functions generic over number types.
*/
pub trait Number: Copy {
	fn add(self, other: Self) -> Self;
	fn sub(self, other: Self) -> Self;
	fn mul(self, other: Self) -> Self;
	fn div(self, other: Self) -> Self;

	fn eq(self, other: Self) -> bool;
	fn ne(self, other: Self) -> bool {
		!self.eq(other)
	}
	fn lt(self, other: Self) -> bool;
	fn gt(self, other: Self) -> bool {
		other.lt(self)
	}
	fn le(self, other: Self) -> bool {
		other.lt(self) || self.eq(other)
	}
	fn ge(self, other: Self) -> bool {
		other.gt(self) || self.eq(other)
	}

	fn min(self, other: Self) -> Self {
		if self.lt(other) {
			self
		} else {
			other
		}
	}
	fn max(self, other: Self) -> Self {
		if self.gt(other) {
			self
		} else {
			other
		}
	}

	fn to_u8(self) -> u8;
	fn to_u16(self) -> u16;
	fn to_u32(self) -> u32;
	fn to_u64(self) -> u64;
	fn to_u128(self) -> u128;

	fn to_i8(self) -> i8;
	fn to_i16(self) -> i16;
	fn to_i32(self) -> i32;
	fn to_i64(self) -> i64;
	fn to_i128(self) -> i128;

	fn to_f32(self) -> f32;
	fn to_f64(self) -> f64;
}

macro_rules! impl_number {
	($name:ident) => {
		impl Number for $name {
			fn add(self, other: Self) -> Self {
				self + other
			}
			fn sub(self, other: Self) -> Self {
				self - other
			}
			fn mul(self, other: Self) -> Self {
				self * other
			}
			fn div(self, other: Self) -> Self {
				self / other
			}

			fn eq(self, other: Self) -> bool {
				self == other
			}
			fn lt(self, other: Self) -> bool {
				self < other
			}

			fn to_u8(self) -> u8 {
				self as u8
			}
			fn to_u16(self) -> u16 {
				self as u16
			}
			fn to_u32(self) -> u32 {
				self as u32
			}
			fn to_u64(self) -> u64 {
				self as u64
			}
			fn to_u128(self) -> u128 {
				self as u128
			}

			fn to_i8(self) -> i8 {
				self as i8
			}
			fn to_i16(self) -> i16 {
				self as i16
			}
			fn to_i32(self) -> i32 {
				self as i32
			}
			fn to_i64(self) -> i64 {
				self as i64
			}
			fn to_i128(self) -> i128 {
				self as i128
			}

			fn to_f32(self) -> f32 {
				self as f32
			}
			fn to_f64(self) -> f64 {
				self as f64
			}
		}
	};
}

impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);
impl_number!(i128);

impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(u128);

impl_number!(f32);
impl_number!(f64);


pub fn approach<T: Number>(x: T, y: T, amount: T) -> T {
	if x.lt(y) {
		x.add(amount).min(y)
	} else {
		x.sub(amount).max(y)
	}
}

pub fn map(value: f64, start_low: f64, start_high: f64, target_low: f64, target_high: f64) -> f64 {
    (value - start_low) / (start_high - start_low) * (target_high - target_low) + target_low
}

pub fn lerp(x: f64, y: f64, t: f64) -> f64 {
	x + (y - x) * t.clamp(0.0, 1.0)
}

pub fn hermite(t: f64) -> f64 {
	let t = t.clamp(0.0, 1.0);
	t * t * (3.0 - 2.0 * t)
}

pub fn herp(x: f64, y: f64, t: f64) -> f64 {
	lerp(x, y, hermite(t))
}

pub enum Tween {
	SineIn,
	SineOut,
	SineInOut,
	QuadIn,
	QuadOut,
	QuadInOut,
	CubicIn,
	CubicOut,
	CubicInOut,
	QuartIn,
	QuartOut,
	QuartInOut,
	QuintIn,
	QuintOut,
	QuintInOut,
	ExpoIn,
	ExpoOut,
	ExpoInOut,
	CircIn,
	CircOut,
	CircInOut,
	BackIn,
	BackOut,
	BackInOut,
	ElasticIn,
	ElasticOut,
	ElasticInOut,
	BounceIn,
	BounceOut,
	BounceInOut,
}

pub fn tween(kind: Tween, t: f64) -> f64 {
	use std::f64::consts::PI;
	let t = t.clamp(0.0, 1.0);
	// implementations borrowed from https://easings.net/
	match kind {
		Tween::SineIn =>
			1.0 - (t * PI / 2.0).cos(),
		Tween::SineOut =>
			(t * PI / 2.0).sin(),
		Tween::SineInOut =>
			-((PI * t).cos() - 1.0) / 2.0,
		Tween::QuadIn =>
			t * t,
		Tween::QuadOut =>
			1.0 - (1.0 - t).powi(2),
		Tween::QuadInOut =>
			if t < 0.5 {
				2.0 * t * t
			} else {
				1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
			},
		Tween::CubicIn =>
			t * t * t,
		Tween::CubicOut =>
			1.0 - (1.0 - t).powi(3),
		Tween::CubicInOut =>
			if t < 0.5 {
				4.0 * t * t * t
			} else {
				1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
			},
		Tween::QuartIn =>
			t * t * t * t,
		Tween::QuartOut =>
			1.0 - (1.0 - t).powi(4),
		Tween::QuartInOut =>
			if t < 0.5 {
				8.0 * t * t * t * t
			} else {
				1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
			},
		Tween::QuintIn =>
			t * t * t * t * t,
		Tween::QuintOut =>
			1.0 - (1.0 - t).powi(5),
		Tween::QuintInOut =>
			if t < 0.5 {
				16.0 * t * t * t * t * t
			} else {
				1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
			},
		Tween::ExpoIn =>
			if t == 0.0 {
				0.0
			} else {
				2.0f64.powf(10.0 * t - 10.0)
			},
		Tween::ExpoOut =>
			if t == 1.0 {
				1.0
			} else {
				1.0 - 2.0f64.powf(-10.0 * t)
			},
		Tween::ExpoInOut =>
			if t == 0.0 {
				0.0
			} else if t == 1.0 {
				1.0
			} else if t < 0.5 {
				2.0f64.powf(20.0 * t - 10.0) / 2.0
			} else {
				(2.0 - 2.0f64.powf(-20.0 * t + 10.0)) / 2.0
			},
		Tween::CircIn =>
			1.0 - (1.0 - t * t).sqrt(),
		Tween::CircOut =>
			(1.0 - (t - 1.0).powi(2)).sqrt(),
		Tween::CircInOut =>
			if t < 0.5 {
				(1.0 - (1.0 - (2.0 * t).powi(2))) / 2.0
			} else {
				((1.0 - (-2.0 * t + 2.0).powi(2)) + 1.0) / 2.0
			},
		Tween::BackIn => {
			const C: f64 = 1.70158;
			(C + 1.0) * t * t * t - C * t * t
		},
		Tween::BackOut => {
			const C: f64 = 1.70158;
			1.0 + (C + 1.0) * (t - 1.0).powi(3) + C * (t - 1.0).powi(2)
		},
		Tween::BackInOut => {
			const C1: f64 = 1.70158;
			const C2: f64 = C1 * 1.525;
			if t < 0.5 {
				(2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2) / 2.0
			} else {
				(2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (2.0 * t - 2.0) + C2) / 2.0
			}
		},
		Tween::ElasticIn => {
			const C: f64 = (2.0 * PI) / 3.0;
			if t == 0.0 {
				0.0
			} else if t == 1.0 {
				1.0
			} else {
				-(2.0f64.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * C).sin()
			}
		},
		Tween::ElasticOut => {
			const C: f64 = (2.0 * PI) / 3.0;
			if t == 0.0 {
				0.0
			} else if t == 1.0 {
				1.0
			} else {
				2.0f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C).sin() + 1.0
			}
		},
		Tween::ElasticInOut => {
			const C: f64 = (2.0 * PI) / 4.5;
			if t == 0.0 {
				0.0
			} else if t == 1.0 {
				1.0
			} else if t < 0.5 {
				-(2.0f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C).sin()) / 2.0
			} else {
				(2.0f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C).sin()) / 2.0 + 1.0
			}
		},
		Tween::BounceIn =>
			1.0 - tween(Tween::BounceOut, 1.0 - t),
		Tween::BounceOut => {
			const N: f64 = 7.5625;
			const D: f64 = 2.75;
			if t < 1.0 / D {
				N * t * t
			} else if t < 2.0 / D {
				let a = t - 1.5;
				N * (a / D) * a + 0.75
			} else if t < 2.5 / D {
				let a = t - 2.25;
				N * (a / D) * a + 0.9375
			} else {
				let a = t - 2.625;
				N * (a / D) * t + 0.984375
			}
		},
		Tween::BounceInOut =>
			if t < 0.5 {
				(1.0 - tween(Tween::BounceOut, 1.0 - 2.0 * t)) / 2.0
			} else {
				(1.0 + tween(Tween::BounceOut, 2.0 * t - 1.0)) / 2.0
			},
	}
}

pub fn terp(kind: Tween, x: f64, y: f64, t: f64) -> f64 {
	lerp(x, y, tween(kind, t))
}


#[cfg(test)]
mod test {
    use crate::math;

	#[test]
	fn test_math() {
		assert_eq!(math::approach(10, 20, 2), 12);
		assert_eq!(math::approach(10, 20, 12), 20);
		assert_eq!(math::approach(20, 10, 12), 10);

		assert_eq!(math::approach(10.0, 20.0, 2.0), 12.0);
		assert_eq!(math::approach(10.0, 20.0, 12.0), 20.0);
		assert_eq!(math::approach(20.0, 10.0, 12.0), 10.0);

		assert_eq!(math::map(4.0, 0.0, 10.0, 10.0, 20.0), 14.0);
	}
}

