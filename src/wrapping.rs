use core::{
	cmp::Ordering,
	ops::{
		Add,
		AddAssign,
		Div,
		DivAssign,
		Mul,
		MulAssign,
		Rem,
		RemAssign,
		Sub,
		SubAssign,
	},
};

use funty::IsInteger;

/** Marks an integer for wrapping-overflow arithmetic.

This type encloses a Rust integer, and causes all arithmetic operations done on
it to discard output bits that cannot fit in the type.

This type is the fastest, as it has no branches and merely truncates results to
fit, but is by the same token the least precise. It is useful for ring
arithmetic, but not for any arithmetic where you need to observe boundary
conditions.
**/
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Wrapping<T: IsInteger> {
	inner: T,
}

impl<T: IsInteger> Wrapping<T> {
	/// Removes the `Wrapping` marker, returning the inner value.
	pub fn value(self) -> T {
		self.inner
	}
}

impl<T: IsInteger> PartialEq<T> for Wrapping<T> {
	fn eq(&self, other: &T) -> bool {
		self.inner.eq(other)
	}
}

impl<T: IsInteger> PartialOrd<T> for Wrapping<T> {
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		self.inner.partial_cmp(other)
	}
}

impl<T: IsInteger> AsRef<T> for Wrapping<T> {
	fn as_ref(&self) -> &T {
		&self.inner
	}
}

impl<T: IsInteger> AsMut<T> for Wrapping<T> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.inner
	}
}

impl<T: IsInteger> From<T> for Wrapping<T> {
	fn from(inner: T) -> Self {
		Self { inner }
	}
}

impl<T: IsInteger> Add<Self> for Wrapping<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		self.inner.wrapping_add(rhs.inner).into()
	}
}

impl<T: IsInteger> Add<&Self> for Wrapping<T> {
	type Output = Self;

	fn add(self, rhs: &Self) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> Add<T> for Wrapping<T> {
	type Output = Self;

	fn add(self, rhs: T) -> Self {
		self.inner.wrapping_add(rhs).into()
	}
}

impl<T: IsInteger> Add<&T> for Wrapping<T> {
	type Output = Self;

	fn add(self, rhs: &T) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> AddAssign<Self> for Wrapping<T> {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&Self> for Wrapping<T> {
	fn add_assign(&mut self, rhs: &Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<T> for Wrapping<T> {
	fn add_assign(&mut self, rhs: T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&T> for Wrapping<T> {
	fn add_assign(&mut self, rhs: &T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> Sub<Self> for Wrapping<T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		self.inner.wrapping_sub(rhs.inner).into()
	}
}

impl<T: IsInteger> Sub<&Self> for Wrapping<T> {
	type Output = Self;

	fn sub(self, rhs: &Self) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> Sub<T> for Wrapping<T> {
	type Output = Self;

	fn sub(self, rhs: T) -> Self {
		self.inner.wrapping_sub(rhs).into()
	}
}

impl<T: IsInteger> Sub<&T> for Wrapping<T> {
	type Output = Self;

	fn sub(self, rhs: &T) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> SubAssign<Self> for Wrapping<T> {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&Self> for Wrapping<T> {
	fn sub_assign(&mut self, rhs: &Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<T> for Wrapping<T> {
	fn sub_assign(&mut self, rhs: T) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&T> for Wrapping<T> {
	fn sub_assign(&mut self, rhs: &T) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> Mul<Self> for Wrapping<T> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		self.inner.wrapping_mul(rhs.inner).into()
	}
}

impl<T: IsInteger> Mul<&Self> for Wrapping<T> {
	type Output = Self;

	fn mul(self, rhs: &Self) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> Mul<T> for Wrapping<T> {
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		self.inner.wrapping_mul(rhs).into()
	}
}

impl<T: IsInteger> Mul<&T> for Wrapping<T> {
	type Output = Self;

	fn mul(self, rhs: &T) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> MulAssign<Self> for Wrapping<T> {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&Self> for Wrapping<T> {
	fn mul_assign(&mut self, rhs: &Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<T> for Wrapping<T> {
	fn mul_assign(&mut self, rhs: T) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&T> for Wrapping<T> {
	fn mul_assign(&mut self, rhs: &T) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> Div<Self> for Wrapping<T> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		self.inner.wrapping_div(rhs.inner).into()
	}
}

impl<T: IsInteger> Div<&Self> for Wrapping<T> {
	type Output = Self;

	fn div(self, rhs: &Self) -> Self {
		self / *rhs
	}
}

impl<T: IsInteger> Div<T> for Wrapping<T> {
	type Output = Self;

	fn div(self, rhs: T) -> Self {
		self.inner.wrapping_div(rhs).into()
	}
}

impl<T: IsInteger> Div<&T> for Wrapping<T> {
	type Output = Self;

	fn div(self, rhs: &T) -> Self {
		self / *rhs
	}
}

impl<T: IsInteger> DivAssign<Self> for Wrapping<T> {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<&Self> for Wrapping<T> {
	fn div_assign(&mut self, rhs: &Self) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<T> for Wrapping<T> {
	fn div_assign(&mut self, rhs: T) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<&T> for Wrapping<T> {
	fn div_assign(&mut self, rhs: &T) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> Rem<Self> for Wrapping<T> {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self {
		self.inner.wrapping_rem(rhs.inner).into()
	}
}

impl<T: IsInteger> Rem<&Self> for Wrapping<T> {
	type Output = Self;

	fn rem(self, rhs: &Self) -> Self {
		self % *rhs
	}
}

impl<T: IsInteger> Rem<T> for Wrapping<T> {
	type Output = Self;

	fn rem(self, rhs: T) -> Self {
		self.inner.wrapping_rem(rhs).into()
	}
}

impl<T: IsInteger> Rem<&T> for Wrapping<T> {
	type Output = Self;

	fn rem(self, rhs: &T) -> Self {
		self % *rhs
	}
}

impl<T: IsInteger> RemAssign<Self> for Wrapping<T> {
	fn rem_assign(&mut self, rhs: Self) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<&Self> for Wrapping<T> {
	fn rem_assign(&mut self, rhs: &Self) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<T> for Wrapping<T> {
	fn rem_assign(&mut self, rhs: T) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<&T> for Wrapping<T> {
	fn rem_assign(&mut self, rhs: &T) {
		*self = *self % rhs
	}
}
