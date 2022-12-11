use super::Sparsity;
use std::ops::RangeInclusive;
use std::slice::Iter;

#[derive(Clone, Copy)]
pub struct MultiSparsityEntry {
    sparsity: Sparsity,
    neighbor_name: &'static str,
}

impl MultiSparsityEntry {
    fn new(neighbor_name: &'static str, sparsity: RangeInclusive<usize>) -> MultiSparsityEntry {
        MultiSparsityEntry {
            sparsity: Sparsity::new(sparsity),
            neighbor_name,
        }
    }

    pub fn sparsity(&self) -> Sparsity {
        self.sparsity
    }

    pub fn neighbor_name(&self) -> &'static str {
        self.neighbor_name
    }
}

#[derive(Clone)]
pub struct MultiSparsity(Vec<MultiSparsityEntry>);

impl MultiSparsity {
    pub fn new() -> MultiSparsity {
        MultiSparsity(Vec::new())
    }

    pub fn add(&mut self, neighbor_name: &'static str, sparsity: RangeInclusive<usize>) {
        self.0
            .push(MultiSparsityEntry::new(neighbor_name, sparsity));
    }

    pub fn iter(&self) -> Iter<MultiSparsityEntry> {
        self.0.iter()
    }
}
