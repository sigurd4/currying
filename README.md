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

### Compile-time currying

Currying also works at compile-time.

```rust
use currying::*;

const fn f(x: u8, y: u8, z: u8) -> u8 {
    x + y + z
}

const X: u8 = 1;
const Y: u8 = 2;
const Z: u8 = 3;

const {
    let fx = f.curry(X);

    assert!(fx(Y, Z) == f(X, Y, Z));

    let fxz = fx.rcurry(Z);

    assert!(fxz(Y) == f(X, Y, Z));

    let fxyz = fxz.curry(Y);

    assert!(fxyz() == f(X, Y, Z));
}
```

### Asyncronous function traits

Asyncronous function traits are an experminental feature. Enable it with the `async` or the `experimental` feature flag.

It should work, but i've not tested it yet.

## Currying from the right

Currying can be done from the right too, with the method `rcurry()`.

This is a stable feature, and is enabled by default. You can opt out of it by disabling the `rcurry` feature flag.