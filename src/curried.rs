use core::marker::Tuple;

use crate::concat_args::ConcatArgs;

use core::ops::{AsyncFn, AsyncFnMut, AsyncFnOnce};

/// A struct which represents a curried function.
/// 
/// This struct implements [FnOnce](core::ops::FnOnce), [FnMut](core::ops::FnMut) and [Fn](core::ops::Fn) if the curried function also implements these traits.
/// 
/// It also implements [AsyncFnOnce](core::ops::AsyncFnOnce), [AsyncFnMut](core::ops::AsyncFnMut) and [AsyncFn](core::ops::AsyncFn) if the feature `async` is enabled,
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

impl<LX, X, RX, F, U> const FnOnce<X> for Curried<LX, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X, RX): ~const ConcatArgs<Type = U>,
    F: ~const FnOnce<U>
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, args: X) -> Self::Output
    {
        self.func.call_once((self.args_left, args, self.args_right).concat_args())
    }
}

impl<LX, X, RX, F, U> const FnMut<X> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): ~const ConcatArgs<Type = U>,
    F: ~const FnMut<U>
{
    extern "rust-call" fn call_mut(&mut self, args: X) -> Self::Output
    {
        self.func.call_mut((self.args_left, args, self.args_right).concat_args())
    }
}

impl<LX, X, RX, F, U> const Fn<X> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): ~const ConcatArgs<Type = U>,
    F: ~const Fn<U>
{
    extern "rust-call" fn call(&self, args: X) -> Self::Output
    {
        self.func.call((self.args_left, args, self.args_right).concat_args())
    }
}

impl<LX, X, RX, F, U> AsyncFnOnce<X> for Curried<LX, RX, F>
where
    LX: Tuple,
    X: Tuple,
    RX: Tuple,
    (LX, X, RX): ConcatArgs<Type = U>,
    F: AsyncFnOnce<U>
{
    type Output = F::Output;

    type CallOnceFuture = F::CallOnceFuture;

    extern "rust-call" fn async_call_once(self, args: X) -> Self::CallOnceFuture
    {
        self.func.async_call_once((self.args_left, args, self.args_right).concat_args())
    }
}

impl<LX, X, RX, F, U> AsyncFnMut<X> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): ConcatArgs<Type = U>,
    F: AsyncFnMut<U>
{
    type CallRefFuture<'a> = F::CallRefFuture<'a>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(&mut self, args: X) -> Self::CallRefFuture<'_>
    {
        self.func.async_call_mut((self.args_left, args, self.args_right).concat_args())
    }
}

impl<LX, X, RX, F, U> AsyncFn<X> for Curried<LX, RX, F>
where
    LX: Tuple + Copy,
    X: Tuple,
    RX: Tuple + Copy,
    (LX, X, RX): ConcatArgs<Type = U>,
    F: AsyncFn<U>
{
    extern "rust-call" fn async_call(&self, args: X) -> Self::CallRefFuture<'_>
    {
        self.func.async_call((self.args_left, args, self.args_right).concat_args())
    }
}