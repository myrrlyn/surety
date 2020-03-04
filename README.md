<div class="title-block" style="text-align: center;" align="center">

# Surety <!-- omit in toc -->

## A Relief Drain for Arithmetic Overflows <!-- omit in toc -->

[![Crate][crate_img]][crate]
[![Documentation][docs_img]][docs]
[![License][license_img]][license_file]

[![Crate Downloads][downloads_img]][crate]
[![Crate Size][loc_img]][loc]

</div>

This crate provides wrappers over the Rust integer fundamentals for smooth,
careful, arithmetic.

Integers in computing are only an approximation of the mathematical concept of
the “integer” set (ℤ), and this approximation breaks down when the result of an
arithmetic operation produces a number outside the range that a given register
can hold.

Rust defines that arithmetic overflow in debug builds is detected, and begins a
panic, so that users are more likely to observe and rectify these events. In
release builds, the operation silently wraps, without detection. This choice is
made to provide a compromise between numeric correctness (mathematics makes
consistent sense, and the sum of two numbers is always larger than the two
numbers summed) and programmatic correctness (it runs quickly).

## Usage

This crate has a very straightforward API.

Import its types:

```rust
use surety::*;
```

It only exports five names: `Ensure`, `Checked`, `Overflowing`, `Wrapping`, and
`Saturating`. You do not need any of these names to be visible, so you can also
write

```rust
use surety::Ensure as _;
```

and get the same effect, without any symbol collisions.

The `Ensure` trait adds conversion methods to the Rust integers to produce the
wrapper types.

```rust
let check = 5.checked();
let wrap = 6.wrapping();
let sat = 7.saturating();
let ovf = 8.overflowing();
```

Call these constructors once, at the start of your arithmetic, and nothing else
changes. The types all implement the arithmetic operators `+`, `-`, `*`,
`/`, and `%`. All but `Saturating` implement `-`, `<<`, and `>>`.

Every prefixed method in `abs`, `pow`, `div_euclid`, `rem_euclid` is available
as an unprefixed method on their corresponding types.

## Examples

```rust
use surety::Ensure as _;
```

## What Is Overflow

Arithmetic overflow (unrelated to buffer overflow) occurs when the numeric
result of a mathematical operation cannot be losslessly encoded in the
fixed-width integer type that the operation is defined to return.

We can prove that for any fixed-width, 2’s-complement, integer type, addition of
two `N`-bit numbers produces an `N+1`-bit sum, and multiplication produces a
`2N`-bit product. Typically, programming languages require that for fixed-width,
2’s-complement integer types of width `N`, arithmetic is defined as `N + N -> N`
and `N * N -> N`. `N` is smaller than `N + 1` or `2N`, so some additions and
multiplications will produce a value that requires too many bits to encode in
the output type. This is overflow.

Rust defines three ways to handle overflow while retaining fixed-width integer
types.

## Checked Overflow

When an arithmetic instruction produces an overflowing result, the CPU detects
this and sets a status flag. Programs can view the status flag to determine
whether the result value is sensible or not.

Rust implements this as `fn checked_math<T>(a: T, b: T) -> Option<T>;`. Programs
must first look at the status flag before they can use the result value. If the
status flag marks `None`, an overflow occurred, and the result cannot be used
as a number.

This behavior is available as the `.checked_op(self, rhs) -> Option<Self>`
method on all integers by default. The `Checked<T>` type in this crate
wraps `Option<T>`, and implements the arithmetic operators by calling
`.checked_op` only while the value is still valid.

This is the safest option, as an integer value is only present while it matches
the expected behavior of a pure number.

## Wrapping Overflow

When a CPU computes an `N`-bit arithmetic operation, it stores the `N` lowest
bits of the result in the output register. This truncation is equivalent to
“wrapping” the number line into a number circle, so that it runs
`…, max - 1, max, min, min + 1, …` as a consecutive sequence.

The `Wrapping<T>` type in this crate implements the arithmetic operators by only
preserving the `N` least significant bits, discarding any excess bits that
overflow the type.

This is the fastest option, as it never inspects the value and only uses the
ordinary pathways present in the CPU’s arithmetic engine.

### Detected Overflow

The `Overflowing<T>` type is a compromise between `Checked<T>` and
`Wrapping<T>`. It uses wrapping arithmetic, sets a flag on overflow, and retains
the result. Its value is always considered valid, and can be viewed and used for
arithmetic. The overflow flag, at `.has_overflowed`, is optional to inspect.

## Saturating Overflow

This is a clamping function on the value range of the integer type. When a
result goes beyond the value range, it is brought back to the range edge.
Attempting to produce a value less than `min` returns `min`, and attempting to
produce a value greater than `max` returns `max`.

This is available through the `Saturating<T>` type.

<!-- Badges -->

[crate]: https://crates.io/crates/surety "Crate Link"
[crate_img]: https://img.shields.io/crates/v/surety.svg?logo=rust "Crate Page"
[docs]: https://docs.rs/surety "Documentation"
[docs_img]: https://docs.rs/surety/badge.svg "Documentation Link"
[downloads_img]: https://img.shields.io/crates/dv/surety.svg?logo=rust "Crate Downloads"
[license_file]: https://github.com/myrrlyn/surety/blob/master/LICENSE.txt "License File"
[license_img]: https://img.shields.io/crates/l/surety.svg "License Display"
[loc]: https://github.com/myrrlyn/surety "Repository"
[loc_img]: https://tokei.rs/b1/github/myrrlyn/surety?category=code "Repository Size"
