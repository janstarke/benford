use num::CheckedAdd;

pub struct Fibonacci<D>(D, D)
where
    D: CheckedAdd + Copy + From<u8>;
    
impl<D> Iterator for Fibonacci<D>
where
    D: CheckedAdd + Copy + From<u8>,
{
    type Item = D;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.0;
        self.0 = self.1;
        self.1 = match self.1.checked_add(&res) {
            Some(sum) => sum,
            None => return None,
        };
        Some(res)
    }
}

impl<D> Default for Fibonacci<D>
where
    D: CheckedAdd + Copy + From<u8>,
{
    fn default() -> Self {
        Self(0.into(), 1.into())
    }
}
