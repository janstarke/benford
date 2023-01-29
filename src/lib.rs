//! This crate provides a way to test of some given set of numbers conforms to
//! [Benford's law](https://en.wikipedia.org/wiki/Benford%27s_law)
//!
//! Currently, only the first digit is being used. You can use your own method
//! of mapping a number to a digit, by implementing [`BenfordClass`]
//!
//! # Example 1: Fibonacci numbers are Benford
//! 
//! ```rust
//! use benford::{BenfordTester, FirstDigitBase10};
//! use num::CheckedAdd;
//!
//! struct Fibonacci<D>(D, D)
//! where
//!     D: CheckedAdd + Copy + From<u8> ;
//! 
//! impl<D> Iterator for Fibonacci<D>
//! where
//!     D: CheckedAdd + Copy + From<u8> ,
//! {
//!     type Item = D;
//! 
//!     fn next(&mut self) -> Option<Self::Item> {
//!         let res = self.0;
//!         self.0 = self.1;
//!         self.1 = match self.1.checked_add(&res) {
//!             Some(sum) => sum,
//!             None => return None,
//!         };
//!         Some(res)
//!     }
//! }
//! 
//! impl<D> Default for Fibonacci<D> where D: CheckedAdd + Copy + From<u8> {
//!     fn default() -> Self {
//!         Self(0.into(), 1.into())
//!     }
//! }
//! 
//! let mut tester = BenfordTester::default();
//! let mut fibonacci = Fibonacci::<u64>::default();
//! for val in fibonacci {
//!     if val != 0 {
//!         tester.add_sample::<FirstDigitBase10>(val.into());
//!     }
//! }
//! assert!(tester.is_benford());
//! ```
//!
//! # Example 2: Natural numbers are not Benford
//! 
//! ```rust
//! use benford::{BenfordTester, FirstDigitBase10};
//!
//! let mut tester = BenfordTester::default();
//! for val in 1..u16::MAX {
//!     tester.add_sample::<FirstDigitBase10>(val.into());
//! }
//! assert!(! tester.is_benford())
//! ```
mod alpha;
mod benford_class;
mod benford_tester;
mod digit;
mod first_digit_base10;

pub use alpha::*;
pub use benford_class::*;
pub use benford_tester::*;
pub use digit::*;
pub use first_digit_base10::*;
