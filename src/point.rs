/*!
mathematical vectors!
*/

use std::{array, fmt::Debug};

#[derive(Debug, Clone, Copy)]
pub struct Point<T, const N: usize>
where T: Copy + Debug {
	data: [T; N],
}
impl<T, const N: usize> Point<T, N>
where T: Copy + Debug {
	#[inline]
	pub fn new(data: [T; N]) -> Self {
		Self {
			data,
		}
	}

	#[inline]
	pub fn get(&self, index: usize) -> Option<T> {
		self.data.get(index).cloned()
	}
	#[inline]
	pub fn set(&mut self, index: usize, value: T) {
		self.data.get_mut(index).map(|x| *x = value);
	}

	#[inline]
	pub fn unpack(&self) -> &[T; N] {
		&self.data
	}
	#[inline]
	pub fn unpack_mut(&mut self) -> &mut [T; N] {
		&mut self.data
	}
	#[inline]
	pub fn unwrap(self) -> [T; N] {
		self.data
	}

	#[inline]
	pub fn unary(self, op: impl Fn(T) -> T) -> Self {
		let mut iter = self.data.iter()
			.map(|x| op(x.clone()));
		let array: [T; N] = array::from_fn(|_| iter.next().unwrap());
		Point::new(array)
	}
	#[inline]
	pub fn binary(self, other: Point<T, N>, op: impl Fn(T, T) -> T) -> Self {
		let mut iter = self.data.iter()
			.zip(other.data.iter())
			.map(|x| op(x.0.clone(), x.1.clone()));
		let array: [T; N] = array::from_fn(|_| iter.next().unwrap());
		Point::new(array)
	}
}

impl<T, const N: usize> Point<T, N>
where T: Copy + Debug + num_traits::Num {
	#[inline]
	pub fn add(self, other: Point<T, N>) -> Self {
		self.binary(other, |x, y| x + y)
	}
	#[inline]
	pub fn sub(self, other: Point<T, N>) -> Self {
		self.binary(other, |x, y| x - y)
	}
	#[inline]
	pub fn mul(self, other: Point<T, N>) -> Self {
		self.binary(other, |x, y| x * y)
	}
	#[inline]
	pub fn div(self, other: Point<T, N>) -> Self {
		self.binary(other, |x, y| x / y)
	}
	#[inline]
	pub fn rem(self, other: Point<T, N>) -> Self {
		self.binary(other, |x, y| x % y)
	}

	#[inline]
	pub fn dot(self, other: Point<T, N>) -> T {
		(self * other)
			.unwrap()
			.into_iter()
			.reduce(|acc, x| acc + x)
			.unwrap() // who the hell is running this on a 0 length vector?
	}
}

impl<T> Point<T, 3>
where T: Copy + Debug + num_traits::Float {
	#[inline]
	pub fn mag(self) -> T {
		(self * self)
			.unwrap()
			.into_iter()
			.reduce(|acc, x| acc + x)
			.unwrap() // ...
			.sqrt()
	}
}

impl<T> Point<T, 3>
where T: Copy + Debug + num_traits::Num {
	#[inline]
	pub fn cross(self, other: Point<T, 3>) -> Self {
		let lhs = self.unwrap();
		let rhs = other.unwrap();
		Point::new([
			lhs[1] * rhs[2] - lhs[2] * rhs[1],
			lhs[0] * rhs[2] - lhs[2] * rhs[0],
			lhs[0] * rhs[1] - lhs[1] * rhs[0],
		])
	}
}

impl<T, const N: usize> std::ops::Add for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	type Output = Point<T, N>;
	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		Point::add(self, rhs)
	}
}
impl<T, const N: usize> std::ops::AddAssign for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		*self = Point::add(*self, rhs)
	}
}

impl<T, const N: usize> std::ops::Sub for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	type Output = Point<T, N>;
	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		Point::sub(self, rhs)
	}
}
impl<T, const N: usize> std::ops::SubAssign for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		*self = Point::sub(*self, rhs)
	}
}

impl<T, const N: usize> std::ops::Mul for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	type Output = Point<T, N>;
	#[inline]
	fn mul(self, rhs: Self) -> Self::Output {
		Point::mul(self, rhs)
	}
}
impl<T, const N: usize> std::ops::MulAssign for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = Point::mul(*self, rhs)
	}
}

impl<T, const N: usize> std::ops::Div for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	type Output = Point<T, N>;
	#[inline]
	fn div(self, rhs: Self) -> Self::Output {
		Point::div(self, rhs)
	}
}
impl<T, const N: usize> std::ops::DivAssign for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	#[inline]
	fn div_assign(&mut self, rhs: Self) {
		*self = Point::div(*self, rhs)
	}
}

impl<T, const N: usize> std::ops::Rem for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	type Output = Point<T, N>;
	#[inline]
	fn rem(self, rhs: Self) -> Self::Output {
		Point::rem(self, rhs)
	}
}
impl<T, const N: usize> std::ops::RemAssign for Point<T, N>
where T: Copy + Debug + num_traits::Num {
	#[inline]
	fn rem_assign(&mut self, rhs: Self) {
		*self = Point::rem(*self, rhs)
	}
}

impl<T, const N: usize> Point<T, N>
where T: Copy + Debug + std::ops::Neg<Output = T> {
	#[inline]
	fn neg(self) -> Self {
		self.unary(|x| -x)
	}
}
impl<T, const N: usize> std::ops::Neg for Point<T, N>
where T: Copy + Debug +
std::ops::Neg<Output = T> {
	type Output = Point<T, N>;
	#[inline]
	fn neg(self) -> Self::Output {
		Point::neg(self)
	}
}

#[macro_export]
macro_rules! point {
	($($x:expr),+ $(,)?) => {
		$crate::point::Point::new([$($x),+])
	};
}

#[cfg(test)]
mod test {
	#[test]
	fn test_build_0() {
		let p0 = point![1, 2];
		let p1 = point![2, 3];

		assert_eq!(p0.unpack(), &[1, 2]);
		assert_eq!(p1.unpack(), &[2, 3]);

		let mut p2 = p0 + p1;
		
		assert_eq!(p2.unpack(), &[3, 5]);
		
		p2 -= point![4, 2];

		assert_eq!(p2.unpack(), &[-1, 3]);
	}
}


