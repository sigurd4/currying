#![feature(tuple_trait)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_trait_impl)]

//! A crate for currying functions in rust
//! 
//! Currying is a functional programming term which essentially means to pass the first argument to a function, yielding a new function needing only the next following arguments.
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

moddef::pub_flat_mods!(
    curried
    curry
    rcurry
);