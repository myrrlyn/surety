use core::{
	cmp::Ordering,
	convert::TryInto as _,
	ops::{
		Add,
		AddAssign,
		Div,
		DivAssign,
		Mul,
		MulAssign,
		Neg,
		Rem,
		RemAssign,
		Shl,
		ShlAssign,
		Shr,
		ShrAssign,
		Sub,
		SubAssign,
	},
};

use funty::{
	IsInteger,
	IsSigned,
};

/** Marks an integer for overflow-detecting arithmetic.

This type encloses a Rust integer, and a marker `bool`. This type performs
wrapping arithmetic, but overflows are detected and recorded until the value
is reset. Users can freely continue to do arithmetic after overflow, and may
choose to examine or ignore the overflow flag as desired.
**/
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Overflowing<T: IsInteger> {
	/// The contained integer.
	pub value: T,
	/// Marks whether an overflow has occurred. Once an overflow is detected,
	/// this flag remains set until explicitly cleared.
	pub has_overflowed: bool,
}

impl<T: IsInteger> Overflowing<T> {
	/// Calculates the quotient of Euclidean division
	/// `self.value.div_euclid(rhs)`.
	///
	/// If the division would produce an overflow, the `has_overflowed` flag is
	/// set and the `value` is not modified.
	///
	/// # Panics
	///
	/// This function will panic if `rhs` is 0.
	pub fn div_euclid(self, rhs: Self) -> Self {
		let (value, ovf) = self.value.overflowing_div_euclid(rhs.value);
		Self {
			value,
			has_overflowed: self.has_overflowed | rhs.has_overflowed | ovf,
		}
	}

	/// Overflowing Euclidean remainder. Calculates
	/// `self.value.rem_euclid(rhs)`.
	///
	/// If the modulus produces an overflow, then the `has_overflowed` flag is
	/// set and `value` is set to zero.
	///
	/// # Panics
	///
	/// This function will panic if rhs is 0.
	pub fn rem_euclid(self, rhs: Self) -> Self {
		let (value, ovf) = self.value.overflowing_rem_euclid(rhs.value);
		Self {
			value,
			has_overflowed: self.has_overflowed | rhs.has_overflowed | ovf,
		}
	}

	/// Computes the absolute value of `self.value`.
	///
	/// If the absolute value causes an overflow (`T::MIN` has no corresponding
	/// positive value), then `value` is unchanged and the `has_overflow` flag
	/// is set.
	pub fn abs(self) -> Self
	where T: IsSigned {
		let (value, ovf) = self.value.overflowing_abs();
		Self {
			value,
			has_overflowed: self.has_overflowed | ovf,
		}
	}

	/// Raises self to the power of `exp`, using exponentiation by squaring.
	///
	/// The `value` is the wrapped result of exponentiation, and `has_overflow`
	/// is set appropriately.
	pub fn pow(self, exp: u32) -> Self {
		let (value, ovf) = self.value.overflowing_pow(exp);
		Self {
			value,
			has_overflowed: self.has_overflowed | ovf,
		}
	}

	/// Applies an overflowing function to `self.value`.
	fn apply(self, func: impl FnOnce(T) -> (T, bool)) -> Self {
		let (value, ovf) = func(self.value);
		Self {
			value,
			has_overflowed: self.has_overflowed | ovf,
		}
	}

	/// Applies an overflowing function to `self.value` and `rhs.value`.
	fn bin_apply<U: IsInteger>(
		self,
		rhs: Overflowing<U>,
		func: impl FnOnce(T, U) -> (T, bool),
	) -> Self
	{
		let (value, ovf) = func(self.value, rhs.value);
		Self {
			value,
			has_overflowed: self.has_overflowed | rhs.has_overflowed | ovf,
		}
	}
}

impl<T: IsInteger> PartialEq<T> for Overflowing<T> {
	fn eq(&self, other: &T) -> bool {
		self.value.eq(other)
	}
}

impl<T: IsInteger> PartialOrd<T> for Overflowing<T> {
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		self.value.partial_cmp(other)
	}
}

impl<T: IsInteger> AsRef<T> for Overflowing<T> {
	fn as_ref(&self) -> &T {
		&self.value
	}
}

impl<T: IsInteger> AsMut<T> for Overflowing<T> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T: IsInteger> From<T> for Overflowing<T> {
	fn from(value: T) -> Self {
		Self {
			value,
			has_overflowed: false,
		}
	}
}

impl<T: IsInteger> From<(T, bool)> for Overflowing<T> {
	fn from((value, has_overflowed): (T, bool)) -> Self {
		Self {
			value,
			has_overflowed,
		}
	}
}

impl<T: IsInteger> Add<Self> for Overflowing<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		self.bin_apply(rhs, |lhs, rhs| lhs.overflowing_add(rhs))
	}
}

impl<T: IsInteger> Add<&Self> for Overflowing<T> {
	type Output = Self;

	fn add(self, rhs: &Self) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> Add<T> for Overflowing<T> {
	type Output = Self;

	fn add(self, rhs: T) -> Self {
		self.apply(|val| val.overflowing_add(rhs))
	}
}

impl<T: IsInteger> Add<&T> for Overflowing<T> {
	type Output = Self;

	fn add(self, rhs: &T) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> AddAssign<Self> for Overflowing<T> {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&Self> for Overflowing<T> {
	fn add_assign(&mut self, rhs: &Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<T> for Overflowing<T> {
	fn add_assign(&mut self, rhs: T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&T> for Overflowing<T> {
	fn add_assign(&mut self, rhs: &T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> Sub<Self> for Overflowing<T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		self.bin_apply(rhs, |lhs, rhs| lhs.overflowing_sub(rhs))
	}
}

impl<T: IsInteger> Sub<&Self> for Overflowing<T> {
	type Output = Self;

	fn sub(self, rhs: &Self) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> Sub<T> for Overflowing<T> {
	type Output = Self;

	fn sub(self, rhs: T) -> Self {
		self.apply(|val| val.overflowing_sub(rhs))
	}
}

impl<T: IsInteger> Sub<&T> for Overflowing<T> {
	type Output = Self;

	fn sub(self, rhs: &T) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> SubAssign<Self> for Overflowing<T> {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&Self> for Overflowing<T> {
	fn sub_assign(&mut self, rhs: &Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<T> for Overflowing<T> {
	fn sub_assign(&mut self, rhs: T) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&T> for Overflowing<T> {
	fn sub_assign(&mut self, rhs: &T) {
		*self = *self - rhs
	}
}

impl<T: IsSigned> Neg for Overflowing<T> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		self.apply(T::overflowing_neg)
	}
}

impl<T: IsInteger> Mul<Self> for Overflowing<T> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		self.bin_apply(rhs, |lhs, rhs| lhs.overflowing_mul(rhs))
	}
}

impl<T: IsInteger> Mul<&Self> for Overflowing<T> {
	type Output = Self;

	fn mul(self, rhs: &Self) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> Mul<T> for Overflowing<T> {
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		self.apply(|val| val.overflowing_mul(rhs))
	}
}

impl<T: IsInteger> Mul<&T> for Overflowing<T> {
	type Output = Self;

	fn mul(self, rhs: &T) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> MulAssign<Self> for Overflowing<T> {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&Self> for Overflowing<T> {
	fn mul_assign(&mut self, rhs: &Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<T> for Overflowing<T> {
	fn mul_assign(&mut self, rhs: T) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&T> for Overflowing<T> {
	fn mul_assign(&mut self, rhs: &T) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> Div<Self> for Overflowing<T> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		self.bin_apply(rhs, |lhs, rhs| lhs.overflowing_div(rhs))
	}
}

impl<T: IsInteger> Div<&Self> for Overflowing<T> {
	type Output = Self;

	fn div(self, rhs: &Self) -> Self {
		self / *rhs
	}
}

impl<T: IsInteger> Div<T> for Overflowing<T> {
	type Output = Self;

	fn div(self, rhs: T) -> Self {
		self.apply(|val| val.overflowing_div(rhs))
	}
}

impl<T: IsInteger> Div<&T> for Overflowing<T> {
	type Output = Self;

	fn div(self, rhs: &T) -> Self {
		self / *rhs
	}
}

impl<T: IsInteger> DivAssign<Self> for Overflowing<T> {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<&Self> for Overflowing<T> {
	fn div_assign(&mut self, rhs: &Self) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<T> for Overflowing<T> {
	fn div_assign(&mut self, rhs: T) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<&T> for Overflowing<T> {
	fn div_assign(&mut self, rhs: &T) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> Rem<Self> for Overflowing<T> {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self {
		self.bin_apply(rhs, |lhs, rhs| lhs.overflowing_rem(rhs))
	}
}

impl<T: IsInteger> Rem<&Self> for Overflowing<T> {
	type Output = Self;

	fn rem(self, rhs: &Self) -> Self {
		self % *rhs
	}
}

impl<T: IsInteger> Rem<T> for Overflowing<T> {
	type Output = Self;

	fn rem(self, rhs: T) -> Self {
		self.apply(|val| val.overflowing_rem(rhs))
	}
}

impl<T: IsInteger> Rem<&T> for Overflowing<T> {
	type Output = Self;

	fn rem(self, rhs: &T) -> Self {
		self % *rhs
	}
}

impl<T: IsInteger> RemAssign<Self> for Overflowing<T> {
	fn rem_assign(&mut self, rhs: Self) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<&Self> for Overflowing<T> {
	fn rem_assign(&mut self, rhs: &Self) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<T> for Overflowing<T> {
	fn rem_assign(&mut self, rhs: T) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<&T> for Overflowing<T> {
	fn rem_assign(&mut self, rhs: &T) {
		*self = *self % rhs
	}
}

macro_rules! shift {
	($($t:ty),* $(,)?) => { $(
		impl<T: IsInteger> Shl<Overflowing<$t>> for Overflowing<T> {
			type Output = Self;

			fn shl(self, rhs: Overflowing<$t>) -> Self::Output {
				self.bin_apply(rhs, |lval, rval| {
					lval.overflowing_shl(
						rval.try_into()
							.expect("Could not convert the shift amount to `u32`"),
					)
				})
			}
		}

		impl<T: IsInteger> Shl<&Overflowing<$t>> for Overflowing<T> {
			type Output = Self;

			fn shl(self, rhs: &Overflowing<$t>) -> Self::Output {
				self << *rhs
			}
		}

		impl<T: IsInteger> Shl<$t> for Overflowing<T> {
			type Output = Self;

			fn shl(self, rhs: $t) -> Self::Output {
				self.apply(|val| {
					val.overflowing_shl(
						rhs.try_into()
							.expect("Could not convert the shift amount to `u32`"),
					)
				})
			}
		}

		impl<T: IsInteger> Shl<&$t> for Overflowing<T> {
			type Output = Self;

			fn shl(self, rhs: &$t) -> Self::Output {
				self << *rhs
			}
		}

		impl<T: IsInteger> ShlAssign<Overflowing<$t>> for Overflowing<T> {
			fn shl_assign(&mut self, rhs: Overflowing<$t>) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<&Overflowing<$t>> for Overflowing<T> {
			fn shl_assign(&mut self, rhs: &Overflowing<$t>) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<$t> for Overflowing<T> {
			fn shl_assign(&mut self, rhs: $t) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<&$t> for Overflowing<T> {
			fn shl_assign(&mut self, rhs: &$t) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> Shr<Overflowing<$t>> for Overflowing<T> {
			type Output = Self;

			fn shr(self, rhs: Overflowing<$t>) -> Self::Output {
				self.bin_apply(rhs, |lval, rval| {
					lval.overflowing_shr(
						rval.try_into()
							.expect("Could not convert the shift amount to `u32`"),
					)
				})
			}
		}

		impl<T: IsInteger> Shr<&Overflowing<$t>> for Overflowing<T> {
			type Output = Self;

			fn shr(self, rhs: &Overflowing<$t>) -> Self::Output {
				self >> *rhs
			}
		}

		impl<T: IsInteger> Shr<$t> for Overflowing<T> {
			type Output = Self;

			fn shr(self, rhs: $t) -> Self::Output {
				self.apply(|val| {
					val.overflowing_shl(
						rhs.try_into()
							.expect("Could not convert the shift amount to `u32`"),
					)
				})
			}
		}

		impl<T: IsInteger> Shr<&$t> for Overflowing<T> {
			type Output = Self;

			fn shr(self, rhs: &$t) -> Self::Output {
				self >> *rhs
			}
		}

		impl<T: IsInteger> ShrAssign<Overflowing<$t>> for Overflowing<T> {
			fn shr_assign(&mut self, rhs: Overflowing<$t>) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<&Overflowing<$t>> for Overflowing<T> {
			fn shr_assign(&mut self, rhs: &Overflowing<$t>) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<$t> for Overflowing<T> {
			fn shr_assign(&mut self, rhs: $t) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<&$t> for Overflowing<T> {
			fn shr_assign(&mut self, rhs: &$t) {
				*self = *self >> rhs
			}
		}
	)* };
}

shift!(
	i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
