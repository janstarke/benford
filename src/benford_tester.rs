use crate::Alpha;
use crate::{BenfordClass, Digit};
use num::FromPrimitive;
use num::ToPrimitive;

#[derive(Default)]
pub struct BenfordTester {
    samples: [u64; 10],
}

impl BenfordTester {

    /// Does a Chi²-Test of the current data set and returns the result,
    /// which afterwards should be compared to (1-\alpha).
    pub fn chi_squared(&self) -> f64 {
        let mut t = 0.0;
        let n = self.samples.iter().sum::<u64>() as f64;
        //println!("n: {n}");

        for digit_idx in 1..self.samples.len() {
            let digit: Digit = Digit::from_usize(digit_idx).unwrap();

            let h_j = self.samples[digit_idx] as f64;
            let np_j = n * digit.probability();
            //println!("h_j = {h_j}, np_{digit_idx} = {np_j}");
            t += (h_j - np_j).powf(2.0) / np_j;
            //println!("res: {}", (h_j - np_j).powf(2.0) / np_j);
        }
        t
    }
    /// returns `true` if the distribution of dataset matches the expected
    /// Benford's law. To test this, a Chi²-Test is being used.
    pub fn is_benford_with_alpha(&self, alpha: Alpha) -> bool {
        self.chi_squared() < Digit::chi_squared(alpha)
    }

    /// returns `true` if the distribution of dataset matches the expected
    /// Benford's law. To test this, a Chi²-Test is being used.
    pub fn is_benford(&self) -> bool {
        self.chi_squared() < Digit::chi_squared(Alpha::Point995)
    }

    /// add a number to the dataset
    pub fn add_sample<C>(&mut self, sample: C)
    where
        C: BenfordClass,
    {
        self.add_digit(sample.into());
    }

    /// add a digit to the dataset
    pub fn add_digit(&mut self, digit: Digit) {
        let idx: usize = digit.to_usize().unwrap();
        self.samples[idx] += 1;
    }

    /// Resets the dataset, so that this tester can be reused
    pub fn reset(&mut self) {
        self.samples = Default::default();
    }
}

impl AsRef<[u64; 10]> for BenfordTester {

    /// provides access to the array which stores the cumulative frequency of all digits.
    /// 
    /// For every digit there is a corresponding entry in this array. For example,
    /// for numbers with `1` as leading digit, the entry `[1]` contains the absolute number
    /// of ocurrances in the data set.
    /// 
    /// Note that this array contains 10 entries, while numbers normally have no leading zero,
    /// or at least Benford's law ignores numbers with leading zero (such as *0.5*). So, the
    /// entry with the index `0` always contains `0`. 
    fn as_ref(&self) -> &[u64; 10] {
        &self.samples
    }
}
