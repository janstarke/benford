use crate::Alpha;
use crate::{BenfordClass, Digit};
use num::ToPrimitive;
use num::FromPrimitive;

pub struct BenfordTester {
    alpha: Alpha,
    samples: [u64; 10],
}

impl BenfordTester {
    pub fn with_alpha(mut self, alpha: Alpha) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn chi_squared(&self) -> f64 {
        let mut t = 0.0;
        let n = self.samples.iter().sum::<u64>() as f64;
        println!("n: {n}");


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

    pub fn is_benford(&self) -> bool {
        self.chi_squared() < Digit::chi_squared(self.alpha)
    }

    pub fn add_sample<C>(&mut self, sample: C)
    where
        C: BenfordClass,
    {
        self.add_digit(sample.into());
    }

    pub fn add_digit(&mut self, digit: Digit)
    {
        let idx: usize = digit.to_usize().unwrap();
        self.samples[idx] += 1;
    }
}

impl Default for BenfordTester {
    fn default() -> Self {
        Self {
            alpha: Alpha::Point995,
            samples: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}
