use crate::Digit;

/// Maps a number to a [`Digit`].
/// 
/// The mapping is done by calling [`TryFrom`] on the number, followed by a call to [`Into<Digit>`] on the result.
/// This is the reason why you must implement a conversion for all primitive number types.
pub trait BenfordClass:
    TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<i8>
    + TryFrom<i16>
    + TryFrom<i32>
    + TryFrom<i64>
    //+ From<f32>
    //+ From<f64>
    + Into<Digit>
{
}