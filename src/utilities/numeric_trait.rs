use num::{CheckedMul, FromPrimitive, Signed, ToPrimitive};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

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
{
}
