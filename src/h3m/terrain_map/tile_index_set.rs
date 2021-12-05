use hashbag::HashBag;
use rand::Rng;
use std::ops::Range;

#[derive(Clone)]
pub struct TileIndexSet(HashBag<u8>);

impl TileIndexSet {
    pub fn new() -> TileIndexSet {
        TileIndexSet(HashBag::new())
    }

    pub fn add_tiles(&mut self, range: Range<u8>, frequency: usize) {
        for index in range {
            self.0.insert_many(index, frequency);
        }
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
