/*! `surety` provides convenient wrappers over the Rust integers with fixed
overflow behavior.

The Rust fundamental integers have `checked`, `wrapping`, and `saturating`
overflow arithmetic defined as inherent methods, but this requires replacing
operators with method calls, and is unusable in generic contexts that use the
arithmetic operator traits.

This crate provides `Checked`, `Wrapping`, and `Saturating` wrappers which
implement the arithmetic operators by deferring to their wrapped integer’s
inherent methods.

In addition to these wrappers, this crate provides an extension trait, `Ensure`,
on the fundamental integers which adds the `.checked()`, `.wrapping()`, and
`.saturating()` conversion methods to wrap an integer in the named type.

# Examples

This example shows how to attach wrapping-overflow behavior to a number. When
addition exceeds `T::max_value()`, the extra carry-out bit is discarded, and the
sum is wrapped back to `T::min_value()`.

```rust
use surety::*;

//  wrap some integer
let num = 120i8.wrapping();

//  overflow
let wrapped = num + 10;
assert_eq!(wrapped, -126);

//  reverse overflow (not underflow; integers cannot do this)
let reverse = wrapped - 20;
assert_eq!(reverse, 110);
```

In addition, you can saturate at the minimum and maximum values, without
wrapping around to the other edge:

```rust
# use surety::*;
# let reverse = 110i8.wrapping();
//  get the value, and mark it as saturating at the boundary
let sat = reverse.value().saturating();

let max = sat + 20;
assert_eq!(max, i8::max_value());

let low = i8::min_value() + 10;
let min = low.saturating() - 20;
assert_eq!(min, i8::min_value());
```

And lastly, you can perform checked arithmetic that halts on overflow
and performs no further work:

```rust
# use surety::*;
let num = 120i8.checked();

//  arithmetic that does not overflow operates as normal
let valid = num - 10;
assert_eq!(valid, Some(110));

//  arithmetic that does overflow is erased
let invalid = num + 20;
assert!(invalid.is_none());

//  once overflow occurs, the number stays erased until reset.
//  further arithmetic does nothing.
assert!((invalid - 2).is_none());

let reset = invalid.or_insert(0);
assert_eq!(reset, Some(0));
```
!*/

#![no_std]

mod checked;
mod saturating;
mod wrapping;

pub use self::{
	checked::Checked,
	saturating::Saturating,
	wrapping::Wrapping,
};

use funty::IsInteger;

/** Extension method to attach `surety` constructors to the integers.

This trait is only implementable on, and implemented by, the Rust fundamental
integers. It provides typecast wrappers which select a specific arithmetic
behavior on overflow.
**/
pub trait Ensure: IsInteger {
	/// Selects checked-overflow arithmetic.
	fn checked(self) -> Checked<Self>;

	/// Selects wrapping-overflow arithmetic.
	fn wrapping(self) -> Wrapping<Self>;

	/// Selects saturating-overflow arithmetic.
	fn saturating(self) -> Saturating<Self>;
}

impl<T: IsInteger> Ensure for T {
	fn checked(self) -> Checked<Self> {
		self.into()
	}

	fn wrapping(self) -> Wrapping<Self> {
		self.into()
	}

	fn saturating(self) -> Saturating<Self> {
		self.into()
	}
}
