use core::marker::Tuple;

use crate::Curried;

/// A trait for things which may be curried.
/// 
/// C is the rightmost argument being applied in the curry.
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
/// Only types that implement [FnOnce](FnOnce) and can take a rightmost argument of type `C` can be called once curried.
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
#[cfg_attr(feature = "const", const_trait)]
pub trait RCurry<C>: Sized
{
    type Output;

    #[cfg(not(feature = "pedantic"))]
    fn rcurry_once(self, arg: C) -> Self::Output;
    #[cfg(not(feature = "pedantic"))]
    fn rcurry_mut(&mut self, arg: C) -> <&mut Self as RCurry<C>>::Output
    {
        self.rcurry_once(arg)
    }
    #[cfg(not(feature = "pedantic"))]
    fn rcurry(&self, arg: C) -> <&Self as RCurry<C>>::Output
    {
        self.rcurry_once(arg)
    }

    #[cfg(feature = "pedantic")]
    fn rcurry_once<X>(self, arg: C) -> Self::Output
    where
        X: Tuple,
        Self::Output: FnOnce<X>;
    #[cfg(feature = "pedantic")]
    fn rcurry_mut<'a, X>(&'a mut self, arg: C) -> <&'a mut Self as RCurry<C>>::Output
    where
        X: Tuple,
        <&'a mut Self as RCurry<C>>::Output: FnOnce<X>
    {
        self.rcurry_once(arg)
    }
    #[cfg(feature = "pedantic")]
    fn rcurry<'a, X>(&'a self, arg: C) -> <&'a Self as RCurry<C>>::Output
    where
        X: Tuple,
        <&'a Self as RCurry<C>>::Output: FnOnce<X>
    {
        self.rcurry_once(arg)
    }
}

#[cfg(feature = "const")]
impl<C, F> const RCurry<C> for F
{
    type Output = Curried<(), (C,), F>;

    #[cfg(not(feature = "pedantic"))]
    fn rcurry_once(self, arg: C) -> Self::Output
    {
        Curried::rcurry(self, arg)
    }

    #[cfg(feature = "pedantic")]
    fn rcurry_once<X>(self, arg: C) -> Self::Output
    where
        X: Tuple,
        Self::Output: FnOnce<X>
    {
        Curried::rcurry(self, arg)
    }
}

#[cfg(not(feature = "const"))]
impl<C, F> RCurry<C> for F
{
    type Output = Curried<(), (C,), F>;

    #[cfg(not(feature = "pedantic"))]
    fn rcurry_once(self, arg: C) -> Self::Output
    {
        Curried::rcurry(self, arg)
    }

    #[cfg(feature = "pedantic")]
    fn rcurry_once<X>(self, arg: C) -> Self::Output
    where
        X: Tuple,
        Self::Output: FnOnce<X>
    {
        Curried::rcurry(self, arg)
    }
}