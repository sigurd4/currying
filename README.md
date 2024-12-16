[![Build Status (nightly)](https://github.com/sigurd4/currying/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/currying/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/currying/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/currying/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/currying/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/currying/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/currying/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/currying/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/currying/workflows/Test/badge.svg)](https://github.com/sigurd4/currying/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/currying/workflows/Lint/badge.svg)](https://github.com/sigurd4/currying/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/currying.svg)](https://crates.io/crates/currying)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/currying)](https://docs.rs/currying)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/currying)](https://app.codecov.io/github/sigurd4/currying)

# Currying

A crate for currying anything implementing `FnOnce`.

Arguments can be passed one at a time, yielding a new something implementing `FnOnce` (and possibly `FnMut` and `Fn`) which can be called with one less argument. It also implements `AsyncFnOnce`, `AsyncFnMut` and `AsyncFn` if the feature `async` is enabled, since this is an experimental feature. Curried arguments are then omitted when calling the curried function, as they have already been passed.

## Example

```rust
use currying::*;

let f = |x, y, z| x + y + z;
let (x, y, z) = (1, 2, 3);

let fx = f.curry(x);

assert_eq!(fx(y, z), f(x, y, z));

let fxz = fx.rcurry(z);

assert_eq!(fxz(y), f(x, y, z));

let fxyz = fxy.curry(y);

assert_eq!(fxyz(), f(x, y, z));
```

## Experimental features

While this crate does use nightly features regardless, i've found that especially the compile-time stuff tend to break in new versions of the rust language. This is why i've isolated it into a special opt-in feature. If it no longer compiles, please report the error here on github, however the base crate should still work at the very least.

### Asyncronous function traits

Asyncronous function traits are an experminental feature. Enable it with the `async` or the `experimental` feature flag.

It should work, but i've not tested it yet.

### Compile-time currying

Currying also works at compile-time.

```rust
#![feature(const_trait_impl)]

use currying::*;

const fn f(x: u8, y: u8, z: u8) -> u8 {
    x + y + z
}

const X: u8 = 1;
const Y: u8 = 2;
const Z: u8 = 3;

type FType = fn(u8, u8, u8) -> u8;
type FXType = Curried<(u8,), (), FType>;
type FXZType = Curried<(), (u8,), FXType>;
type FXYZType = Curried<(u8,), (), FXZType>;

const F: FType = f;
const FX: FXType = F.curry(X);
const FXZ: FXZType = FX.rcurry(Z);
const FXYZ: FXYZType = FXZ.curry(Y);

assert_eq!(FX(Y, Z), f(X, Y, Z));
assert_eq!(FXZ(Y), f(X, Y, Z));
assert_eq!(FXYZ(), f(X, Y, Z));
```

Compile-time currying is an experminental feature. Enable it with the `const` or the `experimental` feature flag.

### Compile-time function traits

This did work fine initially, but in later rust-nightly releases, it broke. It's currently not possible as of writing. I want to add this again when the language supports it.

## Currying from the right

Currying can be done from the right too, with the method `rcurry()`.

This is a stable feature, and is enabled by default. You can opt out of it by disabling the `rcurry` feature flag.

## Pedantic

By default, anything can technically be curried. While it would be nice to be able to prevent currying of something that isn't a function, this makes type inferrence much worse.

If you want this a type-constraint so that only function-types can be curried, at the cost of ideal type-inferrence, use the feature flag `pedantic`.
