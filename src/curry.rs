use std::marker::{Tuple, PhantomData};

use tupleops::{TupleConcatMany, ConcatMany};

use crate::Curried;

/// A trait for things which may be curried.
/// 
/// C is the leftmost argument being applied in the curry.
/// 
/// X is the rest of the arguments left over after currying.
/// 
/// This trait is automatically implemented for anything implementing [FnOnce](FnOnce) which takes one or more argument.
/// 
/// # Examples
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
#[const_trait]
pub trait Curry<C, X>
{
    type Output;

    fn curry(self, arg: C) -> Self::Output;
}

impl<C, X, F> const Curry<C, X> for F
where
    X: Tuple,
    ((C,), X, ()): TupleConcatMany<((C,), X, ())>,
    ConcatMany<((C,), X, ())>: Tuple,
    F: FnOnce<ConcatMany<((C,), X, ())>>
{
    type Output = Curried<(C,), X, (), F>;

    fn curry(self, arg: C) -> Self::Output
    {
        Curried {
            args_left: (arg,),
            args_right: (),
            func: self,
            phantom: PhantomData
        }
    }
}