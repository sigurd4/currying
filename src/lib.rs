#![feature(tuple_trait)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]
#![feature(const_destruct)]

//! A crate for currying functions in rust
//! 
//! Currying is a functional programming term which essentially means to pass the first argument to a function, yielding a new function needing only the next following arguments.
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
//! let fxy = fx.curry(y);
//! 
//! assert_eq!(fxy(z), f(x, y, z));
//! 
//! let fxyz = fxy.curry(z);
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
//! const fn f(x: u8, y: u8, z: u8) -> u8
//! {
//!     x + y + z
//! }
//! 
//! const X: u8 = 1;
//! const Y: u8 = 2;
//! const Z: u8 = 3;
//! 
//! const ASSERTIONS: [bool; 3] = {
//!     let fx = f.curry(X);
//!     let fxy = fx.curry(Y);
//!     let fxyz = fxy.curry(Z);
//!     [
//!         fx(Y, Z) == f(X, Y, Z),
//!         fxy(Z) == f(X, Y, Z),
//!         fxyz() == f(X, Y, Z)
//!     ]
//! };
//! 
//! assert_eq!(ASSERTIONS, [true; 3]);
//! ```

moddef::moddef!(
    flat(pub) mod {
        curried,
        curry,
        rcurry
    }
);