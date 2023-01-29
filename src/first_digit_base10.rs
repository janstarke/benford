use std::{
    cmp::Ordering,
    fmt::Display,
    ops::DivAssign,
};

use crate::{BenfordClass, Digit};
use num::{FromPrimitive, Num, NumCast};

/// Extracts the first digit of a number.
/// 
/// # Example
/// 
/// ```rust
/// use benford::{Digit, FirstDigitBase10};
/// 
/// assert_eq!(Digit::Three, FirstDigitBase10::from(38974).into());
/// assert_eq!(Digit::Seven, FirstDigitBase10::from(-738974).into());
/// assert_eq!(Digit::Eight, FirstDigitBase10::from(-81.2974).into());
/// ```
pub struct FirstDigitBase10 (Digit);

impl BenfordClass for FirstDigitBase10 {}


trait FirstDigitOf {
    fn first_digit_of(self) -> Digit;
}

struct SignedInteger;
struct UnsignedInteger;
struct Float;

trait NumberType {}
impl NumberType for SignedInteger {}
impl NumberType for UnsignedInteger {}
impl NumberType for Float {}
trait Number: DivAssign<Self> + PartialOrd + FromPrimitive + NumCast + Display + Copy {
    type Type: NumberType;
}

impl Number for i8 {
    type Type = SignedInteger;
}
impl Number for i16 {
    type Type = SignedInteger;
}
impl Number for i32 {
    type Type = SignedInteger;
}
impl Number for i64 {
    type Type = SignedInteger;
}
impl Number for u8 {
    type Type = UnsignedInteger;
}
impl Number for u16 {
    type Type = UnsignedInteger;
}
impl Number for u32 {
    type Type = UnsignedInteger;
}
impl Number for u64 {
    type Type = UnsignedInteger;
}
impl Number for f32 {
    type Type = Float;
}
impl Number for f64 {
    type Type = Float;
}

trait WrapFirstDigitOf<N: Number, T = <N as Number>::Type> {
    fn to_first_digit(self) -> Digit;
}

impl<N> FirstDigitOf for N
where
    N: Number,
    Self: WrapFirstDigitOf<N>,
{
    fn first_digit_of(self) -> Digit {
        N::to_first_digit(self)
    }
}

impl<N> WrapFirstDigitOf<N, UnsignedInteger> for N
where
    N: Number<Type = UnsignedInteger>
{
    fn to_first_digit(mut self) -> Digit {
        let ten = Self::from(10).unwrap();

        while self.partial_cmp(&ten).unwrap() != Ordering::Less {
            self.div_assign(ten);
        }

        Digit::from_u8(<u8 as NumCast>::from(self).unwrap()).unwrap()
    }
}

impl<N> WrapFirstDigitOf<N, SignedInteger> for N
where
    N: Number<Type = SignedInteger> + num::Signed
{
    fn to_first_digit(mut self) -> Digit {
        let ten = Self::from(10).unwrap();
        self = self.abs();

        while self.partial_cmp(&ten).unwrap() != Ordering::Less {
            self.div_assign(ten);
        }

        Digit::from_u8(<u8 as NumCast>::from(self).unwrap()).unwrap()
    }
}

impl<N> WrapFirstDigitOf<N, Float> for N
where
    N: Number<Type = Float> + num::Signed
{
    fn to_first_digit(mut self) -> Digit {
        let ten = Self::from(10).unwrap();
        self = self.abs();

        while self.partial_cmp(&ten).unwrap() != Ordering::Less {
            self.div_assign(ten);
        }

        Digit::from_u8(<u8 as NumCast>::from(self).unwrap()).unwrap()
    }
}


impl<D> From<D> for FirstDigitBase10
where
    D: FirstDigitOf + Num,
{
    fn from(value: D) -> Self {
        if value.is_zero() {
            panic!("using zero is not allowed");
        } else {
            Self (
                value.first_digit_of()
            )
        }
    }
}

impl From<FirstDigitBase10> for Digit {
    fn from(val: FirstDigitBase10) -> Digit {
        val.0
    }
}
