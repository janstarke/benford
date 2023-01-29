use num_derive::{FromPrimitive, ToPrimitive};

use crate::Alpha;

/// represents all digits that are [Benford's law](https://en.wikipedia.org/wiki/Benford%27s_law) copes with.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, FromPrimitive, ToPrimitive, Debug)]
pub enum Digit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
}

impl Digit {
    /// specifies the probability of an occurance of the current digit,
    /// according to [Benford's law](https://en.wikipedia.org/wiki/Benford%27s_law)
    pub fn probability(&self) -> f64 {
        match self {
            Digit::One => f64::log10(2.0) - f64::log10(1.0),
            Digit::Two => f64::log10(3.0) - f64::log10(2.0),
            Digit::Three => f64::log10(4.0) - f64::log10(3.0),
            Digit::Four => f64::log10(5.0) - f64::log10(4.0),
            Digit::Five => f64::log10(6.0) - f64::log10(5.0),
            Digit::Six => f64::log10(7.0) - f64::log10(6.0),
            Digit::Seven => f64::log10(8.0) - f64::log10(7.0),
            Digit::Eight => f64::log10(9.0) - f64::log10(8.0),
            Digit::Nine => f64::log10(10.0) - f64::log10(9.0),
        }
    }

    /// returns the value from the [Chi Squared table](https://en.wikibooks.org/wiki/Engineering_Tables/Chi-Squared_Distribution)
    /// for some given [`Alpha`], assuming that *k-1=8*
    pub fn chi_squared(p: Alpha) -> f64 {
        match p {
            Alpha::Point995 => 1.344,
            Alpha::Point99 => 1.646,
            Alpha::Point975 => 2.180,
            Alpha::Point95 => 2.733,
            Alpha::Point9 => 3.490,
            Alpha::Point75 => 5.071,
            Alpha::Point5 => 7.344,
            Alpha::Point25 => 10.219,
            Alpha::Point1 => 13.362,
            Alpha::Point05 => 15.507,
            Alpha::Point025 => 17.535,
            Alpha::Point01 => 20.090,
            Alpha::Point005 => 21.955,
            Alpha::Point002 => 24.352,
            Alpha::Point001 => 26.124,
        }
    }
}
