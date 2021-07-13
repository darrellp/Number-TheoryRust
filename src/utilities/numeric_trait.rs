use num::{CheckedMul, FromPrimitive, Signed, ToPrimitive};
use std::ops::{Add, AddAssign, BitAnd, Div, Mul, MulAssign, Neg, Rem, Shl, Shr, Sub};

pub trait Numeric:
    Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + Neg<Output = Self>
    + Ord
    + Signed
    + Copy
    + Clone
    + PartialEq
    + FromPrimitive
    + ToPrimitive
    + CheckedMul
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + BitAnd<Output = Self>
    + MulAssign
    + AddAssign
{
}

impl<T> Numeric for T where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Rem<Output = T>
        + Neg<Output = T>
        + Ord
        + Signed
        + Copy
        + Clone
        + PartialEq
        + FromPrimitive
        + ToPrimitive
        + CheckedMul
        + Shl<usize, Output = T>
        + Shr<usize, Output = T>
        + BitAnd<Output = Self>
        + MulAssign
        + AddAssign
{
}
