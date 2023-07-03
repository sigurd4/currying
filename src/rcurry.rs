use std::marker::{Tuple, PhantomData};

use tupleops::{TuplePrepend, Prepend, TupleConcat, ConcatTuples};

use crate::Curried;

/// A trait for things which may be curried.
/// 
/// C is the rightmost argument being applied in the curry.
/// 
/// X is the rest of the arguments left over after currying.
/// 
/// This trait is automatically implemented for anything implementing [FnOnce](FnOnce) which takes one or more argument.
/// 
/// ```rust
/// use currying::*;
/// 
/// let f = |x, y, z| x + y + z;
/// let (x, y, z) = (1, 2, 3);
/// 
/// let fz = f.rcurry(z);
/// 
/// assert_eq!(fz(x, y), f(x, y, z));
/// 
/// let fyz = fz.rcurry(y);
/// 
/// assert_eq!(fyz(x), f(x, y, z));
/// 
/// let fxyz = fyz.rcurry(x);
/// 
/// assert_eq!(fxyz(), f(x, y, z));
/// ```
#[const_trait]
pub trait RCurry<C, X>
{
    type Output;

    fn rcurry(self, arg: C) -> Self::Output;
}

impl<C, X, F> const RCurry<C, X> for F
where
    (C,): Tuple,
    X: Tuple,
    ((), X): TupleConcat<(), X>,
    ConcatTuples<(), X>: Tuple,
    (ConcatTuples<(), X>, (C,)): TupleConcat<ConcatTuples<(), X>, (C,)>,
    ConcatTuples<ConcatTuples<(), X>, (C,)>: Tuple,
    F: FnOnce<ConcatTuples<ConcatTuples<(), X>, (C,)>>
{
    type Output = Curried<(), X, (C,), F>;

    fn rcurry(self, arg: C) -> Self::Output
    {
        Curried {
            args_left: (),
            args_right: (arg,),
            func: self,
            phantom: PhantomData
        }
    }
}