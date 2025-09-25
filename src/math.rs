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

pub const fn map(value: f64, start_low: f64, start_high: f64, target_low: f64, target_high: f64) -> f64 {
    (value - start_low) / (start_high - start_low) * (target_high - target_low) + target_low
}

pub const fn lerp(x: f64, y: f64, t: f64) -> f64 {
	x + (y - x) * t.clamp(0.0, 1.0)
}

pub const fn hermite(t: f64) -> f64 {
	let t = t.clamp(0.0, 1.0);
	t * t * (3.0 - 2.0 * t)
}

pub const fn herp(x: f64, y: f64, t: f64) -> f64 {
	lerp(x, y, hermite(t))
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

