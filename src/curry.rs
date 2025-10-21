use core::marker::Tuple;

use crate::Curried;

/// A trait for things which may be curried.
/// 
/// C is the leftmost argument being applied in the curry.
/// 
/// X is the rest of the arguments left over after currying.
/// 
/// This trait is automatically implemented for anything implementing [FnOnce](core::ops::FnOnce) which takes one or more argument.
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
pub trait Curriable<C, X: Tuple> = Curry<C, Output: FnOnce<X>>;

/// A trait providing the method for currying from the left.
/// 
/// Only types that implement [FnOnce](core::ops::FnOnce) and can take a leftmost argument of type `C` can be called once curried.
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
pub const trait Curry<C>: Sized
{
    type Output;

    fn curry_once(self, arg: C) -> Self::Output;
    fn curry_mut(&mut self, arg: C) -> <&mut Self as Curry<C>>::Output
    {
        self.curry_once(arg)
    }
    fn curry(&self, arg: C) -> <&Self as Curry<C>>::Output
    {
        self.curry_once(arg)
    }
}

impl<C, F> const Curry<C> for F
{
    type Output = Curried<(C,), (), F>;

    fn curry_once(self, arg: C) -> Self::Output
    {
        Curried::curry(self, arg)
    }
}