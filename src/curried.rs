use std::marker::{Tuple, PhantomData, Destruct};

use tupleops::{TupleConcatMany, ConcatMany};


/// A struct which represents a curried function.
/// 
/// This struct implements [FnOnce](FnOnce), [FnMut](FnMut) and [Fn](Fn) if the curried function also implements these traits.
/// 
/// Curried arguments are then omitted when calling the curried function, as they have already been passed.
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
#[derive(Clone, Copy, Debug)]
pub struct Curried<LX, X, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: FnOnce<ConcatMany<(LX, X, RX)>>
{
    pub(crate) args_left: LX,
    pub(crate) args_right: RX,
    pub(crate) func: F,
    pub(crate) phantom: PhantomData<X>
}

impl<LX, X, RX, F> /*const*/ FnOnce<X> for Curried<LX, X, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: /*~const*/ FnOnce<ConcatMany<(LX, X, RX)>>,
    Self: /*~const*/ Destruct
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, args: X) -> Self::Output
    {
        self.func.call_once(private::tuples_concat_const(self.args_left, args, self.args_right))
    }
}

impl<LX, X, RX, F> /*const*/ FnMut<X> for Curried<LX, X, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: /*~const*/ FnMut<ConcatMany<(LX, X, RX)>>
{
    extern "rust-call" fn call_mut(&mut self, args: X) -> Self::Output
    {
        self.func.call_mut(private::tuples_concat_const(self.args_left, args, self.args_right))
    }
}

impl<LX, X, RX, F> /*const*/ Fn<X> for Curried<LX, X, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: /*~const*/ Fn<ConcatMany<(LX, X, RX)>>
{
    extern "rust-call" fn call(&self, args: X) -> Self::Output
    {
        self.func.call(private::tuples_concat_const(self.args_left, args, self.args_right))
    }
}

mod private
{
    use std::{marker::Tuple, mem::ManuallyDrop};

    use tupleops::{TupleConcatMany, ConcatMany};

    union TupleConcatManyTransmutation<Tpls>
    where
        Tpls: TupleConcatMany<Tpls>
    {
        tuples: ManuallyDrop<Tpls>,
        concat: ManuallyDrop<ConcatMany<Tpls>>
    }

    pub const fn tuples_concat_const<LX, X, RX>(left: LX, mid: X, right: RX) -> ConcatMany<(LX, X, RX)>
    where
        LX: Tuple,
        X: Tuple,
        RX: Tuple,
        (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
        ConcatMany<(LX, X, RX)>: Tuple
    {
        unsafe {
            ManuallyDrop::into_inner(TupleConcatManyTransmutation
            {
                tuples: ManuallyDrop::new((left, mid, right))
            }.concat)
        }
    }
}