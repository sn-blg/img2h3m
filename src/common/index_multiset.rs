use hashbag::HashBag;
use num::Unsigned;
use rand::{rngs::ThreadRng, Rng};
use std::hash::Hash;

#[derive(Clone)]
pub struct IndexMultiset<T: Clone + Copy + Unsigned + Eq + Hash> {
    inner: HashBag<T>,
}

impl<T: Clone + Copy + Unsigned + Eq + Hash> IndexMultiset<T> {
    pub fn new() -> Self {
        IndexMultiset {
            inner: HashBag::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn add_index(&mut self, index: T, frequency: usize) {
        if frequency > 0 {
            self.inner.insert_many(index, frequency);
        }
    }

    pub fn random_index(&self, rng: &mut ThreadRng) -> Option<T> {
        if self.inner.is_empty() {
            return None;
        }
        self.inner
            .iter()
            .nth(rng.gen_range(0..self.inner.len()))
            .cloned()
    }

    pub fn remove_index(&mut self, index: T) -> Option<(T, usize)> {
        self.inner.take_all(&index)
    }
}
