use std::marker::{Tuple, PhantomData};

use tupleops::{TupleConcat, ConcatTuples};


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
pub struct Curried<LX, X, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X): TupleConcat<LX, X>,
    ConcatTuples<LX, X>: Tuple,
    (ConcatTuples<LX, X>, RX): TupleConcat<ConcatTuples<LX, X>, RX>,
    ConcatTuples<ConcatTuples<LX, X>, RX>: Tuple,
    F: FnOnce<ConcatTuples<ConcatTuples<LX, X>, RX>>
{
    pub(crate) args_left: LX,
    pub(crate) args_right: RX,
    pub(crate) func: F,
    pub(crate) phantom: PhantomData<X>
}

impl<LX, X, RX, F> FnOnce<X> for Curried<LX, X, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X): TupleConcat<LX, X>,
    ConcatTuples<LX, X>: Tuple,
    (ConcatTuples<LX, X>, RX): TupleConcat<ConcatTuples<LX, X>, RX>,
    ConcatTuples<ConcatTuples<LX, X>, RX>: Tuple,
    F: FnOnce<ConcatTuples<ConcatTuples<LX, X>, RX>>
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, args: X) -> Self::Output
    {
        self.func.call_once(tupleops::concat_tuples(tupleops::concat_tuples(self.args_left, args), self.args_right))
    }
}

impl<LX, X, RX, F> FnMut<X> for Curried<LX, X, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X): TupleConcat<LX, X>,
    ConcatTuples<LX, X>: Tuple,
    (ConcatTuples<LX, X>, RX): TupleConcat<ConcatTuples<LX, X>, RX>,
    ConcatTuples<ConcatTuples<LX, X>, RX>: Tuple,
    F: FnMut<ConcatTuples<ConcatTuples<LX, X>, RX>>
{
    extern "rust-call" fn call_mut(&mut self, args: X) -> Self::Output
    {
        self.func.call_mut(tupleops::concat_tuples(tupleops::concat_tuples(self.args_left, args), self.args_right))
    }
}

impl<LX, X, RX, F> Fn<X> for Curried<LX, X, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X): TupleConcat<LX, X>,
    ConcatTuples<LX, X>: Tuple,
    (ConcatTuples<LX, X>, RX): TupleConcat<ConcatTuples<LX, X>, RX>,
    ConcatTuples<ConcatTuples<LX, X>, RX>: Tuple,
    F: Fn<ConcatTuples<ConcatTuples<LX, X>, RX>>
{
    extern "rust-call" fn call(&self, args: X) -> Self::Output
    {
        self.func.call(tupleops::concat_tuples(tupleops::concat_tuples(self.args_left, args), self.args_right))
    }
}