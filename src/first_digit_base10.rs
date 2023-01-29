use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{DivAssign, Neg},
};

use crate::{BenfordClass, Digit};
use num::{FromPrimitive, Num};

/// Extracts the first digit of a number.
/// 
/// # Example
/// 
/// ```rust
/// use benford::{Digit, FirstDigitBase10};
/// 
/// assert_eq!(Digit::Three, FirstDigitBase10::from(38974).into());
/// assert_eq!(Digit::Seven, FirstDigitBase10::from(-738974).into());
/// ```
pub struct FirstDigitBase10 (Digit);

impl BenfordClass for FirstDigitBase10 {}


trait FirstDigitOf {
    fn first_digit_of(self) -> Digit;
}

struct Signed;
struct Unsigned;

trait SignType {}
impl SignType for Signed {}
impl SignType for Unsigned {}
trait Integer: DivAssign<Self> + Ord + FromPrimitive + From<u8> + TryInto<u8> + Display + Copy {
    type Sign: SignType;
}

impl Integer for i16 {
    type Sign = Signed;
}
impl Integer for i32 {
    type Sign = Signed;
}
impl Integer for i64 {
    type Sign = Signed;
}
impl Integer for u8 {
    type Sign = Unsigned;
}
impl Integer for u16 {
    type Sign = Unsigned;
}
impl Integer for u32 {
    type Sign = Unsigned;
}
impl Integer for u64 {
    type Sign = Unsigned;
}

trait WrapFirstDigitOf<T: Integer, S = <T as Integer>::Sign> {
    fn to_first_digit(self) -> Digit;
}

impl<I> FirstDigitOf for I
where
    I: Integer,
    Self: WrapFirstDigitOf<I>,
{
    fn first_digit_of(self) -> Digit {
        I::to_first_digit(self)
    }
}

impl<I> WrapFirstDigitOf<I, Unsigned> for I
where
    I: Integer<Sign = Unsigned>,
    <I as std::convert::TryInto<u8>>::Error: std::fmt::Debug,
{
    fn to_first_digit(mut self) -> Digit {
        let ten = Self::from(10);

        while self.cmp(&ten) != Ordering::Less {
            self.div_assign(ten);
        }

        Digit::from_u8(self.try_into().unwrap()).unwrap()
    }
}

impl<I> WrapFirstDigitOf<I, Signed> for I
where
    I: Integer<Sign = Signed> + num::Signed,
    <I as std::convert::TryInto<u8>>::Error: std::fmt::Debug,
{
    fn to_first_digit(mut self) -> Digit {
        let ten = Self::from(10);
        self = self.abs();

        while self.cmp(&ten) != Ordering::Less {
            self.div_assign(ten);
        }

        Digit::from_u8(self.try_into().unwrap()).unwrap()
    }
}

impl FirstDigitOf for i8
{
    fn first_digit_of(mut self) -> Digit {
        let ten = 10;
        if self.is_negative() {
            self = self.neg();
        }

        while self.cmp(&ten) != Ordering::Less {
            self.div_assign(ten);
        }

        Digit::from_u8(self.try_into().unwrap()).unwrap()
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

impl From<&FirstDigitBase10> for Digit {
    fn from(val: &FirstDigitBase10) -> Digit {
        val.0
    }
}
