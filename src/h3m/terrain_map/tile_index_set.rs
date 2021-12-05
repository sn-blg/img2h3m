use hashbag::HashBag;
use rand::Rng;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct TileIndexSet(HashBag<u8>);

impl TileIndexSet {
    pub fn new(range: RangeInclusive<u8>, frequency: usize) -> TileIndexSet {
        let mut index_set = HashBag::new();
        for index in range {
            index_set.insert_many(index, frequency);
        }
        TileIndexSet(index_set)
    }

    pub fn with_tiles(mut self, range: RangeInclusive<u8>, frequency: usize) -> TileIndexSet {
        for index in range {
            self.0.insert_many(index, frequency);
        }
        self
    }

    pub fn random_index(&self) -> u8 {
        *self
            .0
            .iter()
            .nth(rand::thread_rng().gen_range(0..self.0.len()))
            .unwrap()
    }

    pub fn remove(&mut self, index: u8) {
        self.0.take_all(&index).unwrap();
    }
}
