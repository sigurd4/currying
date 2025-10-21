use core::marker::Tuple;

use crate::Curried;

/// A trait for things which may be curried.
/// 
/// C is the rightmost argument being applied in the curry.
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
pub trait RCurriable<C, X: Tuple> = RCurry<C, Output: FnOnce<X>>;

/// A trait providing the method for currying from the right.
/// 
/// Only types that implement [FnOnce](core::ops::FnOnce) and can take a rightmost argument of type `C` can be called once curried.
/// 
/// # Examples
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
pub const trait RCurry<C>: Sized
{
    type Output;

    fn rcurry_once(self, arg: C) -> Self::Output;
    fn rcurry_mut(&mut self, arg: C) -> <&mut Self as RCurry<C>>::Output
    {
        self.rcurry_once(arg)
    }
    fn rcurry(&self, arg: C) -> <&Self as RCurry<C>>::Output
    {
        self.rcurry_once(arg)
    }
}

impl<C, F> const RCurry<C> for F
{
    type Output = Curried<(), (C,), F>;

    fn rcurry_once(self, arg: C) -> Self::Output
    {
        Curried::rcurry(self, arg)
    }
}