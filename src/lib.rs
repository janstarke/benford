//! This crate provides a way to test of some given set of numbers conforms to
//! [Benford's law](https://en.wikipedia.org/wiki/Benford%27s_law)
//!
//! Currently, only the first digit is being used. You can use your own method
//! of mapping a number to a digit, by implementing [`BenfordClass`]
//!
//! # Example 1: Fibonacci numbers are Benford ...
//! 
//! ```rust
//! use benford::{BenfordTester, FirstDigitBase10, Fibonacci};
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
//! # Example 2: ... but only with limited confidence
//! 
//! if your test set is too small
//! 
//! ```rust
//! use benford::{Alpha, BenfordTester, FirstDigitBase10, Fibonacci};
//!
//! let mut tester = BenfordTester::default();
//! let mut fibonacci = Fibonacci::<u16>::default();
//! for val in fibonacci {
//!     if val != 0 {
//!         tester.add_sample::<FirstDigitBase10>(val.into());
//!     }
//! }
//! assert!(! tester.is_benford());
//! assert!(tester.is_benford_with_alpha(Alpha::Point9));
//! ```
//! 
//! # Example 3: Natural numbers are not Benford
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
mod fibonacci;

pub use alpha::*;
pub use benford_class::*;
pub use benford_tester::*;
pub use digit::*;
pub use first_digit_base10::*;
pub use fibonacci::*;