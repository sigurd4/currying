#![no_std]
#![feature(tuple_trait)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(const_trait_impl)]
//#![cfg_attr(feature = "const", feature(const_trait_impl))]
//#![cfg_attr(feature = "const", feature(const_destruct))]
#![cfg_attr(feature = "async", feature(async_closure))]
#![cfg_attr(feature = "async", feature(async_fn_traits))]

//! A crate for currying functions in rust
//!
//! Arguments can be passed one at a time, yielding a new something implementing `FnOnce`
//! (and possibly `FnMut` and `Fn`) which can be called with one less argument.
//!
//! It also implements [AsyncFnOnce](AsyncFnOnce), [AsyncFnMut](AsyncFnMut) and [AsyncFn](AsyncFn) if the feature `async` is enabled,
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
//! #![feature(const_trait_impl)]
//!
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
//! type FType = fn(u8, u8, u8) -> u8;
//! type FXType = Curried<(u8,), (), &'static FType>;
//! type FXZType = Curried<(), (u8,), &'static FXType>;
//! type FXYZType = Curried<(u8,), (), &'static FXZType>;
//!
//! const F: FType = f;
//! const FX: FXType = F.curry(X);
//! const FXZ: FXZType = FX.rcurry(Z);
//! const FXYZ: FXYZType = FXZ.curry(Y);
//!
//! assert_eq!(FX(Y, Z), f(X, Y, Z));
//! assert_eq!(FXZ(Y), f(X, Y, Z));
//! assert_eq!(FXYZ(), f(X, Y, Z));
//! ```
moddef::moddef!(
    flat(pub) mod {
        curried,
        curry,
        rcurry for cfg(feature = "rcurry")
    }
);

#[cfg(not(feature = "pedantic"))]
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

    #[cfg(feature = "const")]
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

        type FType = fn(u8, u8, u8) -> u8;
        type FXType = Curried<(u8,), (), &'static FType>;
        type FXZType = Curried<(), (u8,), &'static FXType>;
        type FXYZType = Curried<(u8,), (), &'static FXZType>;

        const F: FType = f;
        const FX: FXType = F.curry(X);
        const FXZ: FXZType = FX.rcurry(Z);
        const FXYZ: FXYZType = FXZ.curry(Y);

        assert_eq!(FX(Y, Z), f(X, Y, Z));
        assert_eq!(FXZ(Y), f(X, Y, Z));
        assert_eq!(FXYZ(), f(X, Y, Z));
    }
}

#[cfg(feature = "pedantic")]
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

        let fx = f.curry::<(i32, i32)>(x);

        assert_eq!(fx(y, z), f(x, y, z));

        let fxz = fx.rcurry::<(i32,)>(z);

        assert_eq!(fxz(y), f(x, y, z));

        let fxyz = fxz.curry::<()>(y);

        assert_eq!(fxyz(), f(x, y, z));
    }

    #[cfg(feature = "const")]
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

        type FType = fn(u8, u8, u8) -> u8;
        type FXType = Curried<(u8,), (), &'static FType>;
        type FXZType = Curried<(), (u8,), &'static FXType>;
        type FXYZType = Curried<(u8,), (), &'static FXZType>;

        const F: FType = f;
        const FX: FXType = F.curry::<(u8, u8)>(X);
        const FXZ: FXZType = FX.rcurry::<(u8,)>(Z);
        const FXYZ: FXYZType = FXZ.curry::<()>(Y);

        assert_eq!(FX(Y, Z), f(X, Y, Z));
        assert_eq!(FXZ(Y), f(X, Y, Z));
        assert_eq!(FXYZ(), f(X, Y, Z));
    }
}
