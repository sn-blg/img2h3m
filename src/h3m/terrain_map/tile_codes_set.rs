use hashbag::HashBag;
use rand::Rng;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct TileCodesSet(HashBag<u8>);

impl TileCodesSet {
    pub fn new(range: RangeInclusive<u8>) -> TileCodesSet {
        let mut codes_set = HashBag::new();
        for code in range {
            codes_set.insert(code);
        }
        TileCodesSet(codes_set)
    }

    pub fn random_code(&self) -> u8 {
        *self
            .0
            .iter()
            .nth(rand::thread_rng().gen_range(0..self.0.len()))
            .unwrap()
    }

    pub fn remove(&mut self, code: u8) {
        self.0.take_all(&code);
    }
}
