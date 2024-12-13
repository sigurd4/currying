use core::marker::Tuple;

#[cfg(feature = "async")]
use core::ops::{AsyncFn, AsyncFnMut, AsyncFnOnce};

use tupleops::{TupleConcatMany, ConcatMany};

/// A struct which represents a curried function.
/// 
/// This struct implements [FnOnce](FnOnce), [FnMut](FnMut) and [Fn](Fn) if the curried function also implements these traits.
/// 
/// It also implements [AsyncFnOnce](AsyncFnOnce), [AsyncFnMut](AsyncFnMut) and [AsyncFn](AsyncFn) if the feature `async` is enabled,
/// since this is an experimental feature.
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
/// let fxz = fx.rcurry(z);
/// 
/// assert_eq!(fxz(y), f(x, y, z));
/// 
/// let fxyz = fxz.curry(y);
/// 
/// assert_eq!(fxyz(), f(x, y, z));
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Curried<LX, RX, F>
where
    LX: Tuple,
    RX: Tuple
{
    args_left: LX,
    args_right: RX,
    func: F
}

impl<LX, RX, F> Curried<LX, RX, F>
where
    LX: Tuple,
    RX: Tuple
{
    pub(crate) const fn new(args_left: LX, args_right: RX, func: F) -> Self
    {
        Self {
            args_left,
            args_right,
            func
        }
    }
}

impl<C, F> Curried<(C,), (), F>
{
    pub(crate) const fn curry(func: F, arg: C) -> Self
    {
        Self::new((arg,), (), func)
    }
}

#[cfg(feature = "rcurry")]
impl<C, F> Curried<(), (C,), F>
{
    pub(crate) const fn rcurry(func: F, arg: C) -> Self
    {
        Self::new((), (arg,), func)
    }
}

impl<LX, X, RX, F> /*const*/ FnOnce<X> for Curried<LX, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: /*~const*/ FnOnce<ConcatMany<(LX, X, RX)>>,
    //Self: /*~const*/ Destruct
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, args: X) -> Self::Output
    {
        //self.func.call_once(private::tuples_concat_const(self.args_left, args, self.args_right))
        self.func.call_once(tupleops::concat_many((self.args_left, args, self.args_right)))
    }
}

impl<LX, X, RX, F> /*const*/ FnMut<X> for Curried<LX, RX, F>
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
        //self.func.call_mut(private::tuples_concat_const(self.args_left, args, self.args_right))
        self.func.call_mut(tupleops::concat_many((self.args_left, args, self.args_right)))
    }
}

impl<LX, X, RX, F> /*const*/ Fn<X> for Curried<LX, RX, F>
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
        //self.func.call(private::tuples_concat_const(self.args_left, args, self.args_right))
        self.func.call(tupleops::concat_many((self.args_left, args, self.args_right)))
    }
}

#[cfg(feature = "async")]
impl<LX, X, RX, F> /*const*/ AsyncFnOnce<X> for Curried<LX, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: /*~const*/ AsyncFnOnce<ConcatMany<(LX, X, RX)>>,
    //Self: /*~const*/ Destruct
{
    type Output = F::Output;

    type CallOnceFuture = F::CallOnceFuture;

    extern "rust-call" fn async_call_once(self, args: X) -> Self::CallOnceFuture
    {
        //self.func.async_call_once(private::tuples_concat_const(self.args_left, args, self.args_right))
        self.func.async_call_once(tupleops::concat_many((self.args_left, args, self.args_right)))
    }
}

#[cfg(feature = "async")]
impl<LX, X, RX, F> /*const*/ AsyncFnMut<X> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: /*~const*/ AsyncFnMut<ConcatMany<(LX, X, RX)>>
{
    type CallRefFuture<'a> = F::CallRefFuture<'a>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(&mut self, args: X) -> Self::CallRefFuture<'_>
    {
        //self.func.async_call_mut(private::tuples_concat_const(self.args_left, args, self.args_right))
        self.func.async_call_mut(tupleops::concat_many((self.args_left, args, self.args_right)))
    }
}

#[cfg(feature = "async")]
impl<LX, X, RX, F> /*const*/ AsyncFn<X> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): TupleConcatMany<(LX, X, RX)>,
    ConcatMany<(LX, X, RX)>: Tuple,
    F: /*~const*/ AsyncFn<ConcatMany<(LX, X, RX)>>
{
    extern "rust-call" fn async_call(&self, args: X) -> Self::CallRefFuture<'_>
    {
        //self.func.async_call(private::tuples_concat_const(self.args_left, args, self.args_right))
        self.func.async_call(tupleops::concat_many((self.args_left, args, self.args_right)))
    }
}

/*mod private
{
    use core::{marker::Tuple, mem::ManuallyDrop};

    use tupleops::{TupleConcatMany, ConcatMany};

    union TupleConcatManyTransmutation<Tpls>
    where
        Tpls: TupleConcatMany<Tpls>
    {
        tuples: ManuallyDrop<Tpls>,
        concat: ManuallyDrop<ConcatMany<Tpls>>
    }

    #[deprecated]
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
}*/