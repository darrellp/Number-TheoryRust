use core::fmt;
// CheckedMul, FromPrimitive, Signed, ToPrimitive
use num::{Num, One, Signed, Unsigned, Zero};
use std::ops::{Add, AddAssign, BitAnd, Div, Mul, MulAssign, Neg, Rem, Shl, Shr, Sub};

pub trait UTraits:
    Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + One
    + Zero
    + Unsigned
    + Copy
    + Clone
    + PartialEq
    + Ord
    + PartialOrd
    + fmt::Display
    + BitAnd<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
}

impl<T> UTraits for T where
    T: Add<Output = T>
        + Div<Output = Self>
        + Mul<Output = Self>
        + Sub<Output = Self>
        + Rem<Output = Self>
        + One
        + Zero
        + Unsigned
        + Copy
        + Clone
        + PartialEq
        + Ord
        + PartialOrd
        + fmt::Display
        + BitAnd<Output = Self>
        + Shl<usize, Output = Self>
        + Shr<usize, Output = Self>
{
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// UToI definition
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UToI<T: UTraits> {
    // is_neg should be false for zero
    is_neg: bool,
    absval: T,
}

impl<T> UToI<T>
where
    T: UTraits,
{
    fn new(n: T) -> Self {
        UToI {
            is_neg: false,
            absval: n,
        }
    }

    #[inline]
    fn add_help(&self, other: &UToI<T>, other_neg: bool) -> UToI<T> {
        let (mgn, mut neg) = match (self.is_neg, other_neg) {
            (true, true) | (false, false) => (self.absval + other.absval, self.is_neg),
            (true, false) | (false, true) => {
                if self.absval >= other.absval {
                    (self.absval - other.absval, self.is_neg)
                } else {
                    (other.absval - self.absval, other_neg)
                }
            }
        };
        if mgn == T::zero() {
            neg = false;
        }
        UToI {
            is_neg: neg,
            absval: mgn,
        }
    }
}

impl<T> fmt::Display for UToI<T>
where
    T: UTraits,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.is_neg { "-" } else { "" }, self.absval)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Arithmetic operations
////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> Rem for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn rem(self, other: UToI<T>) -> UToI<T> {
        let rem = self.absval % other.absval;
        let neg = if rem == T::zero() { false } else { self.is_neg };
        UToI {
            is_neg: neg,
            absval: rem,
        }
    }
}

impl<T> Add for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn add(self, other: UToI<T>) -> UToI<T> {
        self.add_help(&other, other.is_neg)
    }
}

impl<T> AddAssign for UToI<T>
where
    T: UTraits,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> Sub for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn sub(self, other: UToI<T>) -> UToI<T> {
        self.add_help(&other, !other.is_neg)
    }
}

impl<T> Mul for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn mul(self, other: UToI<T>) -> UToI<T> {
        let mgn = self.absval * other.absval;
        let neg = self.is_neg != other.is_neg && mgn != T::zero();
        UToI {
            is_neg: neg,
            absval: mgn,
        }
    }
}

impl<T> MulAssign for UToI<T>
where
    T: UTraits,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> Div for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn div(self, other: UToI<T>) -> UToI<T> {
        let mgn = self.absval / other.absval;
        let neg = self.is_neg != other.is_neg && mgn != T::zero();
        UToI {
            is_neg: neg,
            absval: mgn,
        }
    }
}

impl<T> Neg for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn neg(self) -> Self {
        UToI {
            is_neg: if self.absval != T::zero() {
                !self.is_neg
            } else {
                false
            },
            absval: self.absval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Logical operations
////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> BitAnd for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        UToI {
            // Try to emulate 2's complement...
            is_neg: self.is_neg && rhs.is_neg,
            absval: self.absval & rhs.absval,
        }
    }
}

impl<T> Shl<usize> for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn shl(self, rhs: usize) -> Self::Output {
        UToI {
            // There is no exact 2's complement solution here so just use self.is_neg for sign
            is_neg: self.is_neg,
            absval: self.absval << rhs,
        }
    }
}

impl<T> Shr<usize> for UToI<T>
where
    T: UTraits,
{
    type Output = UToI<T>;

    #[inline]
    fn shr(self, rhs: usize) -> Self::Output {
        UToI {
            // Always a logical right shift
            is_neg: false,
            absval: self.absval >> rhs,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Comparison operations
////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> PartialOrd for UToI<T>
where
    T: UTraits,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for UToI<T>
where
    T: UTraits,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.is_neg, other.is_neg) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => self.absval.cmp(&other.absval),
            (true, true) => other.absval.cmp(&self.absval),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> One for UToI<T>
where
    T: UTraits,
{
    #[inline]
    fn one() -> UToI<T> {
        UToI {
            is_neg: false,
            absval: T::one(),
        }
    }
}

impl<T> Zero for UToI<T>
where
    T: UTraits,
{
    #[inline]
    fn zero() -> Self {
        UToI {
            is_neg: false,
            absval: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.absval == T::zero()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromStrRadixErrInt {
    kind: IntErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntErrorKind {
    Empty,
    Invalid,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// num operations
////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> Num for UToI<T>
where
    T: UTraits,
{
    type FromStrRadixErr = FromStrRadixErrInt;

    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if str.len() == 0 {
            let err = FromStrRadixErrInt {
                kind: IntErrorKind::Empty,
            };
            return Err(err);
        }
        let mut str_copy = str;
        let is_neg_result = if str.as_bytes()[0] as char == '-' {
            str_copy = &str_copy[1..];
            true
        } else {
            false
        };
        match T::from_str_radix(str_copy, radix) {
            Err(_) => {
                let err = FromStrRadixErrInt {
                    kind: IntErrorKind::Invalid,
                };
                Err(err)
            }
            Ok(val) => {
                let val_ret = UToI {
                    // For the case of "-0"
                    is_neg: if val == T::zero() {
                        false
                    } else {
                        is_neg_result
                    },
                    absval: val,
                };
                Ok(val_ret)
            }
        }
    }
}

impl<T> Signed for UToI<T>
where
    T: UTraits,
{
    fn abs(&self) -> Self {
        UToI {
            is_neg: false,
            absval: self.absval,
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let newval = match (self.is_neg, other.is_neg) {
            (true, true) | (false, false) => {
                if self.absval >= other.absval {
                    self.absval - other.absval
                } else {
                    other.absval - self.absval
                }
            }
            (true, false) | (false, true) => self.absval + other.absval,
        };
        UToI::new(newval)
    }

    fn signum(&self) -> Self {
        UToI {
            absval: T::one(),
            is_neg: self.is_neg,
        }
    }

    fn is_positive(&self) -> bool {
        !self.is_neg && self.absval != T::zero()
    }

    fn is_negative(&self) -> bool {
        self.is_neg
    }
}

#[test]
fn test_u_to_i() {
    let m = -UToI::new(10u32);
    let mut n = UToI::new(10u32);
    assert_ne!(m, n);
    // verify Copy
    n = m;
    let mut o = n + m;
    println!("neg: {}, pos: {}", o, UToI::new(27u32));
    o += UToI::new(27u32);
    assert_eq!(o, UToI::new(7u32));
    o *= UToI::new(2u32);
    assert_eq!(o, UToI::new(14u32));
    o = o & UToI::new(7u32);
    assert_eq!(o, UToI::new(6u32));
    assert_eq!(o >> 1, UToI::new(3u32));
    assert_eq!(o << 1, UToI::new(12u32));
}
