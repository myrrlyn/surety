use core::{
	cmp::Ordering,
	ops::{
		Add,
		AddAssign,
		Mul,
		MulAssign,
		Sub,
		SubAssign,
	},
};

use funty::IsInteger;

/** Marks a type for saturating-overflow arithmetic.

When values of this type exceed their type’s range in either direction due to
arithmetic, they are clamped to the edge value of their range in the direction
of overflow. That is, if subtraction would cause a value to go below
`min_value()`, it clamps to `min_value()`, and if addition or multiplication
would cause a value to go above `max_value()`, it clamps to `max_value()`
instead.

This can lead to unexpected results, as unlike the `Wrapping` behavior,
arithmetic stops at the value boundary until an operation reverses direction.
Resumed arithmetic always begins from the boundary value, so all information
about intermediate results is lost.
**/
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Saturating<T: IsInteger> {
	/// The contained integer.
	pub value: T,
}

impl<T: IsInteger> Saturating<T> {
	/// Saturating integer exponentiation. Computes `self.value.pow(exp)`,
	/// saturating at the numeric bounds instead of overflowing.
	pub fn saturating_pow(self, exp: u32) -> Self {
		self.value.saturating_pow(exp).into()
	}
}

impl<T: IsInteger> PartialEq<T> for Saturating<T> {
	fn eq(&self, other: &T) -> bool {
		self.value.eq(other)
	}
}

impl<T: IsInteger> PartialOrd<T> for Saturating<T> {
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		self.value.partial_cmp(other)
	}
}

impl<T: IsInteger> AsRef<T> for Saturating<T> {
	fn as_ref(&self) -> &T {
		&self.value
	}
}

impl<T: IsInteger> AsMut<T> for Saturating<T> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T: IsInteger> From<T> for Saturating<T> {
	fn from(value: T) -> Self {
		Self { value }
	}
}

impl<T: IsInteger> Add<Self> for Saturating<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		self.value.saturating_add(rhs.value).into()
	}
}

impl<T: IsInteger> Add<&Self> for Saturating<T> {
	type Output = Self;

	fn add(self, rhs: &Self) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> Add<T> for Saturating<T> {
	type Output = Self;

	fn add(self, rhs: T) -> Self {
		self.value.saturating_add(rhs).into()
	}
}

impl<T: IsInteger> Add<&T> for Saturating<T> {
	type Output = Self;

	fn add(self, rhs: &T) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> AddAssign<Self> for Saturating<T> {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&Self> for Saturating<T> {
	fn add_assign(&mut self, rhs: &Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<T> for Saturating<T> {
	fn add_assign(&mut self, rhs: T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&T> for Saturating<T> {
	fn add_assign(&mut self, rhs: &T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> Sub<Self> for Saturating<T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		self.value.saturating_sub(rhs.value).into()
	}
}

impl<T: IsInteger> Sub<&Self> for Saturating<T> {
	type Output = Self;

	fn sub(self, rhs: &Self) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> Sub<T> for Saturating<T> {
	type Output = Self;

	fn sub(self, rhs: T) -> Self {
		self.value.saturating_sub(rhs).into()
	}
}

impl<T: IsInteger> Sub<&T> for Saturating<T> {
	type Output = Self;

	fn sub(self, rhs: &T) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> SubAssign<Self> for Saturating<T> {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&Self> for Saturating<T> {
	fn sub_assign(&mut self, rhs: &Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<T> for Saturating<T> {
	fn sub_assign(&mut self, rhs: T) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&T> for Saturating<T> {
	fn sub_assign(&mut self, rhs: &T) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> Mul<Self> for Saturating<T> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		self.value.saturating_mul(rhs.value).into()
	}
}

impl<T: IsInteger> Mul<&Self> for Saturating<T> {
	type Output = Self;

	fn mul(self, rhs: &Self) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> Mul<T> for Saturating<T> {
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		self.value.saturating_mul(rhs).into()
	}
}

impl<T: IsInteger> Mul<&T> for Saturating<T> {
	type Output = Self;

	fn mul(self, rhs: &T) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> MulAssign<Self> for Saturating<T> {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&Self> for Saturating<T> {
	fn mul_assign(&mut self, rhs: &Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<T> for Saturating<T> {
	fn mul_assign(&mut self, rhs: T) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&T> for Saturating<T> {
	fn mul_assign(&mut self, rhs: &T) {
		*self = *self * rhs
	}
}
