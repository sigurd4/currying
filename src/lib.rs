#![no_std]
#![feature(tuple_trait)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(const_trait_impl)]
#![feature(const_destruct)]
#![feature(const_precise_live_drops)]
#![feature(async_fn_traits)]
#![feature(specialization)]

//! A crate for currying functions in rust
//!
//! Arguments can be passed one at a time, yielding a new something implementing [FnOnce](core::ops::FnOnce)
//! (and possibly [FnMut](core::ops::FnMut) and [Fn](core::ops::Fn)) which can be called with one less argument.
//!
//! It also implements [AsyncFnOnce](core::ops::AsyncFnOnce), [AsyncFnMut](core::ops::AsyncFnMut) and [AsyncFn](core::ops::AsyncFn) if the feature `async` is enabled,
//! since this is an experimental feature.
//!
//! Curried arguments are then omitted when calling the curried function, as they have already been passed.
//!
//! # Examples
//!
//! ```rust
//! use currying::*;
//!
//! let f = |x, y, z| x + y + z;
//! let (x, y, z) = (1, 2, 3);
//!
//! let fx = f.curry(x);
//!
//! assert_eq!(fx(y, z), f(x, y, z));
//!
//! let fxz = fx.rcurry(z);
//!
//! assert_eq!(fxz(y), f(x, y, z));
//!
//! let fxyz = fxz.curry(y);
//!
//! assert_eq!(fxyz(), f(x, y, z));
//! ```
//!
//! Currying also works at compile-time.
//!
//! ```rust
//! # #![feature(const_trait_impl)]
//! use currying::*;
//!
//! const fn f(x: u8, y: u8, z: u8) -> u8 {
//!     x + y + z
//! }
//!
//! const X: u8 = 1;
//! const Y: u8 = 2;
//! const Z: u8 = 3;
//!
//! const {
//!     let fx = f.curry(X);
//!
//!     assert!(fx(Y, Z) == f(X, Y, Z));
//!
//!     let fxz = fx.rcurry(Z);
//!
//!     assert!(fxz(Y) == f(X, Y, Z));
//!
//!     let fxyz = fxz.curry(Y);
//!
//!     assert!(fxyz() == f(X, Y, Z));
//! }
//! ```
moddef::moddef!(
    flat(pub) mod {
        curried,
        curry,
        rcurry for cfg(feature = "rcurry"),
        concat_args
    }
);

#[cfg(test)]
mod test
{
    #[cfg(feature = "rcurry")]
    #[test]
    fn test()
    {
        use crate::*;

        let f = |x, y, z| x + y + z;
        let (x, y, z) = (1, 2, 3);

        let fx = f.curry(x);

        assert_eq!(fx(y, z), f(x, y, z));

        let fxz = fx.rcurry(z);

        assert_eq!(fxz(y), f(x, y, z));

        let fxyz = fxz.curry(y);

        assert_eq!(fxyz(), f(x, y, z));
    }

    #[test]
    fn test_mut()
    {
        use crate::Curry;

        let i0 = 0;
        let mut i = i0;
        let n = 1;

        let mut f = |j| {
            i += j;
            i
        };

        let mut fj = f.curry_mut(n);

        for k in 1..10
        {
            assert_eq!(fj(), i0 + k * n)
        }
    }

    #[test]
    fn test_once()
    {
        use crate::Curry;

        let i0 = 0;
        let i = i0;
        let n = 1;

        let f = |j| i + j;

        let i = f.curry_once(n)();

        assert_eq!(i, i0 + n)
    }

    #[cfg(feature = "rcurry")]
    #[test]
    fn test_const()
    {
        use crate::*;

        const fn f(x: u8, y: u8, z: u8) -> u8
        {
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
    }
}
