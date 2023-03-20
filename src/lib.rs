#![feature(tuple_trait)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

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

use std::marker::{Tuple, PhantomData};

use tupleops::{TupleConcat, TuplePrepend, Prepend, ConcatTuples};

/// A trait for things which may be curried
/// 
/// X is the argument being applied in the curry.
/// 
/// RX is the rest of the arguments left over after currying.
/// 
/// This trait is automatically implemented for anything implementing [FnOnce](FnOnce) which takes one or more argument.
/// 
/// ```rust
/// use currying::*;
/// 
/// let f = |x, y, z| x + y + z;
/// let (x, y, z) = (1, 2, 3);
/// 
/// let fx = f.curry(x);
/// 
/// assert_eq!(fx(y, z), f(x, y, z));
/// 
/// let fxy = fx.curry(y);
/// 
/// assert_eq!(fxy(z), f(x, y, z));
/// 
/// let fxyz = fxy.curry(z);
/// 
/// assert_eq!(fxyz(), f(x, y, z));
/// ```
pub trait Curry<X, RX>
{
    type Output;

    fn curry(self, arg: X) -> Self::Output;
}

impl<X, RX, F> Curry<X, RX> for F
where
    RX: Tuple,
    (X, RX): TuplePrepend<X, RX>,
    Prepend<X, RX>: Tuple,
    F: FnOnce<Prepend<X, RX>>,
    Curried<(X,), RX, F>: FnOnce<RX>
{
    type Output = Curried<(X,), RX, F>;

    fn curry(self, arg: X) -> Self::Output
    {
        Curried {
            args: (arg,),
            func: self,
            phantom: PhantomData
        }
    }
}

/// A struct which represents a curried function.
/// 
/// This struct implements [FnOnce](FnOnce), [FnMut](FnMut) and [Fn](Fn) if the curried function also implements these traits.
/// 
/// Curried arguments are then omitted when calling the curried function, as they have already been passed.
/// 
/// ```rust
/// use currying::*;
/// 
/// let f = |x, y, z| x + y + z;
/// let (x, y, z) = (1, 2, 3);
/// 
/// let fx = f.curry(x);
/// 
/// assert_eq!(fx(y, z), f(x, y, z));
/// 
/// let fxy = fx.curry(y);
/// 
/// assert_eq!(fxy(z), f(x, y, z));
/// 
/// let fxyz = fxy.curry(z);
/// 
/// assert_eq!(fxyz(), f(x, y, z));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Curried<LX, RX, F>
where
    LX: Tuple,
    RX: Tuple,
    (LX, RX): TupleConcat<LX, RX>,
    ConcatTuples<LX, RX>: Tuple,
    F: FnOnce<ConcatTuples<LX, RX>>
{
    args: LX,
    func: F,
    phantom: PhantomData<RX>
}

impl<LX, RX, F> FnOnce<RX> for Curried<LX, RX, F>
where
    LX: Tuple,
    RX: Tuple,
    (LX, RX): TupleConcat<LX, RX>,
    ConcatTuples<LX, RX>: Tuple,
    F: FnOnce<ConcatTuples<LX, RX>>
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, args: RX) -> Self::Output
    {
        self.func.call_once(tupleops::concat_tuples(self.args, args))
    }
}

impl<LX, RX, F> FnMut<RX> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    RX: Tuple,
    (LX, RX): TupleConcat<LX, RX>,
    ConcatTuples<LX, RX>: Tuple,
    F: FnMut<ConcatTuples<LX, RX>>
{
    extern "rust-call" fn call_mut(&mut self, args: RX) -> Self::Output
    {
        self.func.call_mut(tupleops::concat_tuples(self.args, args))
    }
}

impl<LX, RX, F> Fn<RX> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    RX: Tuple,
    (LX, RX): TupleConcat<LX, RX>,
    ConcatTuples<LX, RX>: Tuple,
    F: Fn<ConcatTuples<LX, RX>>
{
    extern "rust-call" fn call(&self, args: RX) -> Self::Output
    {
        self.func.call(tupleops::concat_tuples(self.args, args))
    }
}