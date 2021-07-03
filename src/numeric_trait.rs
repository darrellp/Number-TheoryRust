use num::FromPrimitive;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Numeric:
    Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + Copy
    + PartialEq
    + FromPrimitive
{
}

impl<T> Numeric for T where
    T: Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Rem<Output = T>
        + Copy
        + PartialEq
        + FromPrimitive
{
}
