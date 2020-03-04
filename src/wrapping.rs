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

/** Marks an integer for wrapping-overflow arithmetic.

This type encloses a Rust integer, and causes all arithmetic operations done on
it to discard output bits that cannot fit in the type.

This type is the fastest, as it has no branches and merely truncates results to
fit, but is by the same token the least precise. It is useful for ring
arithmetic, but not for any arithmetic where you need to observe boundary
conditions.
**/
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Wrapping<T: IsInteger> {
	/// The contained integer.
	pub value: T,
}

impl<T: IsInteger> Wrapping<T> {
	/// Wrapping Eulidean division. Computes `self.value.div_euclid(rhs.value)`,
	/// wrapping around at the boundary of the type.
	///
	/// # Signed Types
	///
	/// Wrapping will only occur in `MIN / -1` on a signed type (where `MIN` is
	/// the negative minimal value for the type). This is equivalent to `-MIN`,
	/// a positive value that is too large to represent in the type. In this
	/// case, this method returns `MIN` itself.
	///
	/// # Unsigned Types
	///
	/// Wrapped division on unsigned types is just normal division. There’s no
	/// way wrapping could ever happen. This function exists, so that all
	/// operations are accounted for in the wrapping operations. Since, for the
	/// positive integers, all common definitions of division are equal, this is
	/// exactly equal to `self / rhs`.
	///
	/// # Panics
	///
	/// This function will panic if `rhs` is 0.
	pub fn div_euclid(self, rhs: Self) -> Self {
		self.value.wrapping_div_euclid(rhs.value).into()
	}

	/// Wrapping Euclidean remainder. Computes
	/// `self.value.rem_euclid(rhs.value)`, wrapping around at the boundary of
	/// the type.
	///
	/// # Signed Integers
	///
	/// Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is
	/// the negative minimal value for the type). In this case, this method
	/// returns 0.
	///
	/// # Unsigned Integers
	///
	/// Wrapped modulo calculation on unsigned types is just the regular
	/// remainder calculation. There’s no way wrapping could ever happen. This
	/// function exists, so that all operations are accounted for in the
	/// wrapping operations. Since, for the positive integers, all common
	/// definitions of division are equal, this is exactly equal to
	/// `self % rhs`.
	///
	/// # Panics
	///
	/// This function will panic if `rhs` is 0.
	pub fn rem_euclid(self, rhs: Self) -> Self {
		self.value.wrapping_rem_euclid(rhs.value).into()
	}

	/// Wrapping (modular) absolute value. Computes `self.value.abs()`, wrapping
	/// around at the boundary of the type.
	///
	/// The only case where such wrapping can occur is when one takes the
	/// absolute value of the negative minimal value for the type this is a
	/// positive value that is too large to represent in the type. In such a
	/// case, this function returns `MIN` itself.
	pub fn abs(self) -> Self
	where T: IsSigned {
		self.value.wrapping_abs().into()
	}

	/// Wrapping (modular) exponentiation. Computes `self.value.pow(exp)`,
	/// wrapping around at the boundary of the type.
	pub fn pow(self, exp: u32) -> Self {
		self.value.wrapping_pow(exp).into()
	}
}

impl<T: IsInteger> PartialEq<T> for Wrapping<T> {
	fn eq(&self, other: &T) -> bool {
		self.value.eq(other)
	}
}

impl<T: IsInteger> PartialOrd<T> for Wrapping<T> {
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		self.value.partial_cmp(other)
	}
}

impl<T: IsInteger> AsRef<T> for Wrapping<T> {
	fn as_ref(&self) -> &T {
		&self.value
	}
}

impl<T: IsInteger> AsMut<T> for Wrapping<T> {
	fn as_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T: IsInteger> From<T> for Wrapping<T> {
	fn from(value: T) -> Self {
		Self { value }
	}
}

impl<T: IsInteger> Add<Self> for Wrapping<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		self.value.wrapping_add(rhs.value).into()
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
		self.value.wrapping_add(rhs).into()
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
		self.value.wrapping_sub(rhs.value).into()
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
		self.value.wrapping_sub(rhs).into()
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

impl<T: IsSigned> Neg for Wrapping<T> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		self.value.wrapping_neg().into()
	}
}

impl<T: IsInteger> Mul<Self> for Wrapping<T> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		self.value.wrapping_mul(rhs.value).into()
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
		self.value.wrapping_mul(rhs).into()
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
		self.value.wrapping_div(rhs.value).into()
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
		self.value.wrapping_div(rhs).into()
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
		self.value.wrapping_rem(rhs.value).into()
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
		self.value.wrapping_rem(rhs).into()
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

macro_rules! shift {
	($($t:ty),* $(,)?) => { $(
		impl<T: IsInteger> Shl<Wrapping<$t>> for Wrapping<T> {
			type Output = Self;

			fn shl(self, rhs: Wrapping<$t>) -> Self::Output {
				self.value.wrapping_shl(
					rhs.value
						.try_into()
						.expect("Could not convert the shift amount to `u32`")
				).into()
			}
		}

		impl<T: IsInteger> Shl<&Wrapping<$t>> for Wrapping<T> {
			type Output = Self;

			fn shl(self, rhs: &Wrapping<$t>) -> Self::Output {
				self << *rhs
			}
		}

		impl<T: IsInteger> Shl<$t> for Wrapping<T> {
			type Output = Self;

			fn shl(self, rhs: $t) -> Self::Output {
				self.value.wrapping_shl(
					rhs.try_into()
						.expect("Could not convert the shift amount to `u32`")
				).into()
			}
		}

		impl<T: IsInteger> Shl<&$t> for Wrapping<T> {
			type Output = Self;

			fn shl(self, rhs: &$t) -> Self::Output {
				self << *rhs
			}
		}

		impl<T: IsInteger> ShlAssign<Wrapping<$t>> for Wrapping<T> {
			fn shl_assign(&mut self, rhs: Wrapping<$t>) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<&Wrapping<$t>> for Wrapping<T> {
			fn shl_assign(&mut self, rhs: &Wrapping<$t>) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<$t> for Wrapping<T> {
			fn shl_assign(&mut self, rhs: $t) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<&$t> for Wrapping<T> {
			fn shl_assign(&mut self, rhs: &$t) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> Shr<Wrapping<$t>> for Wrapping<T> {
			type Output = Self;

			fn shr(self, rhs: Wrapping<$t>) -> Self::Output {
				self.value.wrapping_shr(
					rhs.value
						.try_into()
						.expect("Could not convert the shift amount to `u32`")
				).into()
			}
		}

		impl<T: IsInteger> Shr<&Wrapping<$t>> for Wrapping<T> {
			type Output = Self;

			fn shr(self, rhs: &Wrapping<$t>) -> Self::Output {
				self >> *rhs
			}
		}

		impl<T: IsInteger> Shr<$t> for Wrapping<T> {
			type Output = Self;

			fn shr(self, rhs: $t) -> Self::Output {
				self.value.wrapping_shr(
					rhs.try_into()
						.expect("Could not convert the shift amount to `u32`")
				).into()
			}
		}

		impl<T: IsInteger> Shr<&$t> for Wrapping<T> {
			type Output = Self;

			fn shr(self, rhs: &$t) -> Self::Output {
				self >> *rhs
			}
		}

		impl<T: IsInteger> ShrAssign<Wrapping<$t>> for Wrapping<T> {
			fn shr_assign(&mut self, rhs: Wrapping<$t>) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<&Wrapping<$t>> for Wrapping<T> {
			fn shr_assign(&mut self, rhs: &Wrapping<$t>) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<$t> for Wrapping<T> {
			fn shr_assign(&mut self, rhs: $t) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<&$t> for Wrapping<T> {
			fn shr_assign(&mut self, rhs: &$t) {
				*self = *self >> rhs
			}
		}
	)* };
}

shift!(
	i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
