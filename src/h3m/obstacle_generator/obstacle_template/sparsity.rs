use std::ops::RangeInclusive;

#[derive(Clone, Copy)]
pub struct Sparsity {
    min: usize,
    max: usize,
}

impl Sparsity {
    pub fn new(sparsity: RangeInclusive<usize>) -> Sparsity {
        Sparsity {
            min: *sparsity.start(),
            max: *sparsity.end(),
        }
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> usize {
        self.max
    }
}
