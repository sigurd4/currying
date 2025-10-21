use core::marker::Tuple;

use tupleops::TupleConcatMany;

pub const trait ConcatArgs: TupleConcatMany<Self, Type: Tuple> + Sized
{
    fn concat_args(self) -> Self::Type;
}

impl<T> ConcatArgs for T
where
    T: TupleConcatMany<Self, Type: Tuple>
{
    default fn concat_args(self) -> Self::Type
    {
        tupleops::concat_many(self)
    }
}

macro_rules! impl_concat_args {
    ($($t0:ident $($t:ident)*)?) => {
        impl<L, $($t0, $($t,)*)? R> const ConcatArgs for ((L,), ($($t0, $($t,)*)?), (R,))
        {
            fn concat_args(self) -> Self::Type
            {
                let ((l,), ($($t0, $($t,)*)?), (r,)) = self;
                (l, $($t0, $($t,)*)? r,)
            }
        }
        impl<L, $($t0, $($t,)*)?> const ConcatArgs for ((L,), ($($t0, $($t,)*)?), ())
        {
            fn concat_args(self) -> Self::Type
            {
                let ((l,), ($($t0, $($t,)*)?), ()) = self;
                (l, $($t0, $($t,)*)?)
            }
        }
        impl<$($t0, $($t,)*)? R> const ConcatArgs for ((), ($($t0, $($t,)*)?), (R,))
        {
            fn concat_args(self) -> Self::Type
            {
                let ((), ($($t0, $($t,)*)?), (r,)) = self;
                ($($t0, $($t,)*)? r,)
            }
        }
        $(impl_concat_args!($($t)*);)?
    };
}

#[cfg(any(not(feature = "8"), feature = "dont_hurt_yourself_by_using_all_features"))]
impl_concat_args!();
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(all(feature = "8", not(feature = "16")))]
impl_concat_args!(
    _3 _4 _5 _6 _7 _8
);
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(all(feature = "16", not(feature = "32")))]
impl_concat_args!(
    _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _16
);
#[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
#[cfg(feature = "32")]
impl_concat_args!(
    _3 _4 _5 _6 _7 _8 _9 _10 _11 _12 _13 _14 _16 _17 _18 _19 _20 _21 _22 _23 _24 _25 _26 _27 _28 _29 _30 _31 _32
);