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
	option::{
		Iter,
		IterMut,
	},
};

use funty::{
	IsInteger,
	IsSigned,
};

/** Marks an integer for checked-overflow arithmetic.

This type encloses a Rust integer, and causes all arithmetic operations done on
it to detect overflow and refuse to act if it occurs.

Note: this works by deferring to the `.checked_op` methods on the Rust integers.
The implementation may choose to detect overflow condition either before or
after executing the arithmetic instruction, so the processor may observe an
overflow event.

Once a `Checked<_>` integer enters the overflow state, it will no longer execute
arithmetic instructions until it is reset to a valid value.

This type provides an `Option`-like API in addition to its integer properties.
**/
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Checked<T: IsInteger> {
	/// The contained integer.
	///
	/// This is `Some` while the value has not yet overflowed an arithmetic
	/// operation. Once an overflow occurs, this is set to `None` until
	/// explicitly reset to a fresh value.
	pub value: Option<T>,
}

impl<T: IsInteger> Checked<T> {
	/// Checked Euclidean division. Computes `self.value?.div_euclid(rhs)`,
	/// returning `None` if `rhs == 0` or the division results in overflow.
	pub fn div_euclid(self, rhs: Self) -> Self {
		self.and_then(|val| {
			rhs.value.and_then(|rhs| val.checked_div_euclid(rhs))
		})
	}

	/// Checked Euclidean remainder. Computes `self.value?.rem_euclid(rhs)`,
	/// returning `None` if `rhs == 0` or the division results in overflow.
	pub fn rem_euclid(self, rhs: Self) -> Self {
		self.and_then(|val| {
			rhs.value.and_then(|rhs| val.checked_rem_euclid(rhs))
		})
	}

	/// Checked absolute value. Computes `self.value?.abs()`, returning `None`
	/// if `self.value == T::MIN`.
	pub fn abs(self) -> Self
	where T: IsSigned {
		self.and_then(T::checked_abs)
	}

	/// Checked exponentiation. Computes `self.value?.pow(exp)`, returning
	/// `None` if overflow occurred.
	pub fn pow(self, exp: u32) -> Self {
		self.and_then(|val| val.checked_pow(exp))
	}

	/// Tests if the integer is still valid, and has not yet overflowed.
	///
	/// # Original
	///
	/// [`Option::is_some`](https://doc.rust-lang.org/core/option/enum.Option.html#method.is_some)
	pub fn is_some(&self) -> bool {
		self.value.is_some()
	}

	/// Tests if the integer has overflowed.
	///
	/// # Original
	///
	/// [`Option::is_none`](https://doc.rust-lang.org/core/option/enum.Option.html#method.is_none)
	pub fn is_none(&self) -> bool {
		self.value.is_none()
	}

	/// Borrows the integer value, if present.
	///
	/// # Original
	///
	/// [`Option::as_ref`](https://doc.rust-lang.org/core/option/enum.Option.html#method.as_ref)
	pub fn as_ref(&self) -> Option<&T> {
		self.value.as_ref()
	}

	/// Mutably borrows the integer value, if present.
	///
	/// # Original
	///
	/// [`Option::as_mut`](https://doc.rust-lang.org/core/option/enum.Option.html#method.as_mut)
	pub fn as_mut(&mut self) -> Option<&mut T> {
		self.value.as_mut()
	}

	/// Unwraps the bare integer value, panicking with `msg` if absent.
	///
	/// # Original
	///
	/// [`Option::expect`](https://doc.rust-lang.org/core/option/enum.Option.html#method.expect)
	pub fn expect(self, msg: &str) -> T {
		self.value.expect(msg)
	}

	/// Unwraps the bare integer value, panicking if absent.
	///
	/// # Original
	///
	/// [`Option::unwrap`](https://doc.rust-lang.org/core/option/enum.Option.html#method.is_some)
	pub fn unwrap(self) -> T {
		self.value.unwrap()
	}

	/// Unwraps the bare integer value, substituting a default value if absent.
	///
	/// # Original
	///
	/// [`Option::unwrap_or`](https://doc.rust-lang.org/core/option/enum.Option.html#method.unwrap_or)
	pub fn unwrap_or(self, default: T) -> T {
		self.value.unwrap_or(default)
	}

	/// Unwraps the bare integer value, or computes a default value if absent.
	///
	/// # Original
	///
	/// [`Option::unwrap_or_else`](https://doc.rust-lang.org/core/option/enum.Option.html#method.unwrap_or_else)
	pub fn unwrap_or_else(self, func: impl FnOnce() -> T) -> T {
		self.value.unwrap_or_else(func)
	}

	/// Transforms the integer value to a new integer, if present.
	///
	/// # Original
	///
	/// [`Option::map`](https://doc.rust-lang.org/core/option/enum.Option.html#method.map)
	pub fn map<U: IsInteger>(self, func: impl FnOnce(T) -> U) -> Checked<U> {
		self.value.map(func).into()
	}

	/// Applies a function to the contained integer, substituting a default
	/// value if absent.
	///
	/// The returned `Checked` is always in a valid state.
	///
	/// # Original
	///
	/// [`Option::map_or`](https://doc.rust-lang.org/core/option/enum.Option.html#method.map_or)
	pub fn map_or<U: IsInteger>(
		self,
		default: U,
		func: impl FnOnce(T) -> U,
	) -> Checked<U>
	{
		self.value.map_or(default, func).into()
	}

	/// Applies a function to the contained integer, computing a default value
	/// if absent.
	///
	/// The returned `Checked` is always in a valid state.
	///
	/// # Original
	///
	/// [`Option::map_or_else`](https://doc.rust-lang.org/core/option/enum.Option.html#method.map_or_else)
	pub fn map_or_else<U: IsInteger>(
		self,
		default: impl FnOnce() -> U,
		func: impl FnOnce(T) -> U,
	) -> Checked<U>
	{
		self.value.map_or_else(default, func).into()
	}

	/// Transforms the `Checked<T>` into a `Result<T, E>`, producing `Ok(num)`
	/// if the integer is present and `Err(err)` if it is not.
	///
	/// # Original
	///
	/// [`Option::ok_or`](https://doc.rust-lang.org/core/option/enum.Option.html#method.ok_or)
	pub fn ok_or<E>(self, err: E) -> Result<T, E> {
		self.value.ok_or(err)
	}

	/// Transforms the `Checked<T>` into a `Result<T, E>` producing `Ok(num)` if
	/// the integer is present and computing `Err(func())` if it is not.
	///
	/// # Original
	///
	/// [`Option::map_or_else`](https://doc.rust-lang.org/core/option/enum.Option.html#method.map_or_else)
	pub fn ok_or_else<E>(self, func: impl FnOnce() -> E) -> Result<T, E> {
		self.value.ok_or_else(func)
	}

	/// Returns an iterator over the possibly-contained integer.
	///
	/// # Original
	///
	/// [`Option::iter`](https://doc.rust-lang.org/core/option/enum.Option.html#method.iter)
	pub fn iter(&self) -> Iter<T> {
		self.value.iter()
	}

	/// Returns a mutable iterator over the possibly-contained integer.
	///
	/// # Original
	///
	/// [`Option::iter_mut`](https://doc.rust-lang.org/core/option/enum.Option.html#method.iter_mut)
	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.value.iter_mut()
	}

	/// Replaces `self` with `other` only if the integer is present.
	///
	/// # Original
	///
	/// [`Option::and`](https://doc.rust-lang.org/core/option/enum.Option.html#method.and)
	pub fn and<U: IsInteger>(self, other: impl Into<Checked<U>>) -> Checked<U> {
		self.value.and(other.into().value).into()
	}

	/// Passes the integer into a new fallible computation, if present.
	///
	/// The `Option` produced by the argument function is interpreted as a
	/// `Checked` integer.
	///
	/// # Original
	///
	/// [`Option::and_then`](https://doc.rust-lang.org/core/option/enum.Option.html#method.and_then)
	pub fn and_then<U: IsInteger>(
		self,
		func: impl FnOnce(T) -> Option<U>,
	) -> Checked<U>
	{
		self.value.and_then(func).into()
	}

	/// Tests if the integer satisfies a test. If the integer is missing, or
	/// fails the test, this returns a `None`; otherwise, it returns the integer
	/// unchanged.
	///
	/// # Original
	///
	/// [`Option::filter`](https://doc.rust-lang.org/core/option/enum.Option.html#method.filter)
	pub fn filter(self, func: impl FnOnce(&T) -> bool) -> Self {
		self.value.filter(func).into()
	}

	/// If the integer is missing, replaces it with a new checked integer.
	///
	/// # Original
	///
	/// [`Option::or`](https://doc.rust-lang.org/core/option/enum.Option.html#method.or)
	pub fn or(self, other: Self) -> Self {
		self.value.or(other.value).into()
	}

	/// If the integer is missing, replaces it with a newly-computed checked
	/// integer.
	///
	/// # Original
	///
	/// [`Option::or_else`](https://doc.rust-lang.org/core/option/enum.Option.html#method.or_else)
	pub fn or_else(self, func: impl FnOnce() -> Option<T>) -> Self {
		self.value.or_else(func).into()
	}

	/// If the integer is missing, sets it to be a new integer.
	pub fn or_insert(self, other: T) -> Self {
		self.value.or(Some(other)).into()
	}

	/// If the integer is missing, sets it to be a newly-computed integer.
	pub fn or_insert_with(self, func: impl FnOnce() -> T) -> Self {
		self.value.or_else(|| Some(func())).into()
	}

	/// Returns a valid checked integer if only one of `self` and `other` is
	/// valid.
	///
	/// # Original
	///
	/// [`Option::xor`](https://doc.rust-lang.org/core/option/enum.Option.html#method.xor)
	pub fn xor(self, other: Self) -> Self {
		self.value.xor(other.value).into()
	}

	/// Gets a write reference to the integer, first setting it to a new value
	/// if absent.
	///
	/// # Original
	///
	/// [`Option::get_or_insert`](https://doc.rust-lang.org/core/option/enum.Option.html#method.get_or_insert)
	pub fn get_or_insert(&mut self, val: T) -> &mut T {
		self.value.get_or_insert(val)
	}

	/// Gets a write reference to the integer, first setting it to a
	/// newly-computed value if absent.
	///
	/// # Original
	///
	/// [`Option::get_or_insert_with`](https://doc.rust-lang.org/core/option/enum.Option.html#method.get_or_insert_with)
	pub fn get_or_insert_with(&mut self, func: impl FnOnce() -> T) -> &mut T {
		self.value.get_or_insert_with(func)
	}

	/// Takes the checked value, replacing it with an empty `Checked`.
	///
	/// # Original
	///
	/// [`Option::take`](https://doc.rust-lang.org/core/option/enum.Option.html#method.take)
	pub fn take(&mut self) -> Self {
		self.take_value().into()
	}

	/// Takes the integer, replacing it with an empty `Checked`.
	pub fn take_value(&mut self) -> Option<T> {
		self.value.take()
	}

	/// Replaces the integer with a new value, returining the original
	/// maybe-missing value.
	///
	/// # Original
	///
	/// [`Option::replace`](https://doc.rust-lang.org/core/option/enum.Option.html#method.replace)
	pub fn replace(&mut self, other: T) -> Self {
		self.replace_value(other).into()
	}

	/// Replaces the integer with a new value, returning the original
	/// maybe-missing value.
	pub fn replace_value(&mut self, other: T) -> Option<T> {
		self.value.replace(other)
	}
}

impl<T: IsInteger> PartialEq<Option<T>> for Checked<T> {
	fn eq(&self, other: &Option<T>) -> bool {
		self.value.eq(other)
	}
}

impl<T: IsInteger> PartialOrd<Option<T>> for Checked<T> {
	fn partial_cmp(&self, other: &Option<T>) -> Option<Ordering> {
		self.value.partial_cmp(other)
	}
}

impl<T: IsInteger> From<T> for Checked<T> {
	fn from(num: T) -> Self {
		Self { value: Some(num) }
	}
}

impl<T: IsInteger> From<Option<T>> for Checked<T> {
	fn from(value: Option<T>) -> Self {
		Self { value }
	}
}

impl<T: IsInteger> Add<Self> for Checked<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		self.and_then(|a| rhs.value.and_then(|b| a.checked_add(b)))
	}
}

impl<T: IsInteger> Add<&Self> for Checked<T> {
	type Output = Self;

	fn add(self, rhs: &Self) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> Add<T> for Checked<T> {
	type Output = Self;

	fn add(self, rhs: T) -> Self {
		self.and_then(|a| a.checked_add(rhs))
	}
}

impl<T: IsInteger> Add<&T> for Checked<T> {
	type Output = Self;

	fn add(self, rhs: &T) -> Self {
		self + *rhs
	}
}

impl<T: IsInteger> AddAssign<Self> for Checked<T> {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&Self> for Checked<T> {
	fn add_assign(&mut self, rhs: &Self) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<T> for Checked<T> {
	fn add_assign(&mut self, rhs: T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> AddAssign<&T> for Checked<T> {
	fn add_assign(&mut self, rhs: &T) {
		*self = *self + rhs
	}
}

impl<T: IsInteger> Sub<Self> for Checked<T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		self.and_then(|a| rhs.value.and_then(|b| a.checked_sub(b)))
	}
}

impl<T: IsInteger> Sub<&Self> for Checked<T> {
	type Output = Self;

	fn sub(self, rhs: &Self) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> Sub<T> for Checked<T> {
	type Output = Self;

	fn sub(self, rhs: T) -> Self {
		self.and_then(|a| a.checked_sub(rhs))
	}
}

impl<T: IsInteger> Sub<&T> for Checked<T> {
	type Output = Self;

	fn sub(self, rhs: &T) -> Self {
		self - *rhs
	}
}

impl<T: IsInteger> SubAssign<Self> for Checked<T> {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&Self> for Checked<T> {
	fn sub_assign(&mut self, rhs: &Self) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<T> for Checked<T> {
	fn sub_assign(&mut self, rhs: T) {
		*self = *self - rhs
	}
}

impl<T: IsInteger> SubAssign<&T> for Checked<T> {
	fn sub_assign(&mut self, rhs: &T) {
		*self = *self - rhs
	}
}

impl<T: IsSigned> Neg for Checked<T> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		self.and_then(T::checked_neg)
	}
}

impl<T: IsInteger> Mul<Self> for Checked<T> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		self.and_then(|a| rhs.value.and_then(|b| a.checked_mul(b)))
	}
}

impl<T: IsInteger> Mul<&Self> for Checked<T> {
	type Output = Self;

	fn mul(self, rhs: &Self) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> Mul<T> for Checked<T> {
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		self.and_then(|a| a.checked_mul(rhs))
	}
}

impl<T: IsInteger> Mul<&T> for Checked<T> {
	type Output = Self;

	fn mul(self, rhs: &T) -> Self {
		self * *rhs
	}
}

impl<T: IsInteger> MulAssign<Self> for Checked<T> {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&Self> for Checked<T> {
	fn mul_assign(&mut self, rhs: &Self) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<T> for Checked<T> {
	fn mul_assign(&mut self, rhs: T) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> MulAssign<&T> for Checked<T> {
	fn mul_assign(&mut self, rhs: &T) {
		*self = *self * rhs
	}
}

impl<T: IsInteger> Div<Self> for Checked<T> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		self.and_then(|a| rhs.value.and_then(|b| a.checked_div(b)))
	}
}

impl<T: IsInteger> Div<&Self> for Checked<T> {
	type Output = Self;

	fn div(self, rhs: &Self) -> Self {
		self / *rhs
	}
}

impl<T: IsInteger> Div<T> for Checked<T> {
	type Output = Self;

	fn div(self, rhs: T) -> Self {
		self.and_then(|a| a.checked_div(rhs))
	}
}

impl<T: IsInteger> Div<&T> for Checked<T> {
	type Output = Self;

	fn div(self, rhs: &T) -> Self {
		self / *rhs
	}
}

impl<T: IsInteger> DivAssign<Self> for Checked<T> {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<&Self> for Checked<T> {
	fn div_assign(&mut self, rhs: &Self) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<T> for Checked<T> {
	fn div_assign(&mut self, rhs: T) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> DivAssign<&T> for Checked<T> {
	fn div_assign(&mut self, rhs: &T) {
		*self = *self / rhs
	}
}

impl<T: IsInteger> Rem<Self> for Checked<T> {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self {
		self.and_then(|a| rhs.value.and_then(|b| a.checked_rem(b)))
	}
}

impl<T: IsInteger> Rem<&Self> for Checked<T> {
	type Output = Self;

	fn rem(self, rhs: &Self) -> Self {
		self % *rhs
	}
}

impl<T: IsInteger> Rem<T> for Checked<T> {
	type Output = Self;

	fn rem(self, rhs: T) -> Self {
		self.and_then(|a| a.checked_rem(rhs))
	}
}

impl<T: IsInteger> Rem<&T> for Checked<T> {
	type Output = Self;

	fn rem(self, rhs: &T) -> Self {
		self % *rhs
	}
}

impl<T: IsInteger> RemAssign<Self> for Checked<T> {
	fn rem_assign(&mut self, rhs: Self) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<&Self> for Checked<T> {
	fn rem_assign(&mut self, rhs: &Self) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<T> for Checked<T> {
	fn rem_assign(&mut self, rhs: T) {
		*self = *self % rhs
	}
}

impl<T: IsInteger> RemAssign<&T> for Checked<T> {
	fn rem_assign(&mut self, rhs: &T) {
		*self = *self % rhs
	}
}

macro_rules! shift {
	($($t:ty),* $(,)?) => { $(
		impl<T: IsInteger> Shl<Checked<$t>> for Checked<T> {
			type Output = Self;

			fn shl(self, rhs: Checked<$t>) -> Self::Output {
				self.and_then(|val| val.checked_shl(rhs.value?.try_into().ok()?))
			}
		}

		impl<T: IsInteger> Shl<&Checked<$t>> for Checked<T> {
			type Output = Self;

			fn shl(self, rhs: &Checked<$t>) -> Self::Output {
				self << *rhs
			}
		}

		impl<T: IsInteger> Shl<$t> for Checked<T> {
			type Output = Self;

			fn shl(self, rhs: $t) -> Self::Output {
				self.and_then(|val| val.checked_shl(rhs.try_into().ok()?))
			}
		}

		impl<T: IsInteger> Shl<&$t> for Checked<T> {
			type Output = Self;

			fn shl(self, rhs: &$t) -> Self::Output {
				self << *rhs
			}
		}

		impl<T: IsInteger> ShlAssign<Checked<$t>> for Checked<T> {
			fn shl_assign(&mut self, rhs: Checked<$t>) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<&Checked<$t>> for Checked<T> {
			fn shl_assign(&mut self, rhs: &Checked<$t>) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<$t> for Checked<T> {
			fn shl_assign(&mut self, rhs: $t) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> ShlAssign<&$t> for Checked<T> {
			fn shl_assign(&mut self, rhs: &$t) {
				*self = *self << rhs
			}
		}

		impl<T: IsInteger> Shr<Checked<$t>> for Checked<T> {
			type Output = Self;

			fn shr(self, rhs: Checked<$t>) -> Self::Output {
				self.and_then(|val| val.checked_shr(rhs.value?.try_into().ok()?))
			}
		}

		impl<T: IsInteger> Shr<&Checked<$t>> for Checked<T> {
			type Output = Self;

			fn shr(self, rhs: &Checked<$t>) -> Self::Output {
				self >> *rhs
			}
		}

		impl<T: IsInteger> Shr<$t> for Checked<T> {
			type Output = Self;

			fn shr(self, rhs: $t) -> Self::Output {
				self.and_then(|val| val.checked_shr(rhs.try_into().ok()?))
			}
		}

		impl<T: IsInteger> Shr<&$t> for Checked<T> {
			type Output = Self;

			fn shr(self, rhs: &$t) -> Self::Output {
				self >> *rhs
			}
		}

		impl<T: IsInteger> ShrAssign<Checked<$t>> for Checked<T> {
			fn shr_assign(&mut self, rhs: Checked<$t>) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<&Checked<$t>> for Checked<T> {
			fn shr_assign(&mut self, rhs: &Checked<$t>) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<$t> for Checked<T> {
			fn shr_assign(&mut self, rhs: $t) {
				*self = *self >> rhs
			}
		}

		impl<T: IsInteger> ShrAssign<&$t> for Checked<T> {
			fn shr_assign(&mut self, rhs: &$t) {
				*self = *self >> rhs
			}
		}
	)* };
}

shift!(
	i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
