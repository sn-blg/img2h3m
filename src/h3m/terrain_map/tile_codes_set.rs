use hashbag::HashBag;
use rand::Rng;
use std::ops::RangeInclusive;

pub struct TileCodesSet {
    subsets: Vec<HashBag<u8>>,
    subset_indexes: HashBag<usize>,
}

impl TileCodesSet {
    pub fn new(codes: RangeInclusive<u8>) -> TileCodesSet {
        TileCodesSet::with_frequency(codes, 1)
    }

    pub fn with_frequency(codes: RangeInclusive<u8>, frequency: usize) -> TileCodesSet {
        TileCodesSet {
            subsets: Vec::new(),
            subset_indexes: HashBag::new(),
        }
        .add_codes(codes, frequency)
    }

    pub fn add_codes(mut self, codes: RangeInclusive<u8>, frequency: usize) -> TileCodesSet {
        let new_subset_index = self.subsets.len();

        self.subsets.push(HashBag::from_iter(codes.into_iter()));
        self.subset_indexes.insert_many(new_subset_index, frequency);

        self
    }

    pub fn random_not_excluded_code(&self, excluded_codes: &[u8]) -> Option<u8> {
        let subset_index = *self
            .subset_indexes
            .iter()
            .nth(rand::thread_rng().gen_range(0..self.subset_indexes.len()))
            .unwrap();

        let mut subset = self.subsets[subset_index].clone();

        for code in excluded_codes {
            subset.take_all(&code);
        }

        subset
            .iter()
            .nth(rand::thread_rng().gen_range(0..subset.len()))
            .cloned()
    }

    pub fn random_code(&self) -> u8 {
        let subset_index = *self
            .subset_indexes
            .iter()
            .nth(rand::thread_rng().gen_range(0..self.subset_indexes.len()))
            .unwrap();

        let subset = &self.subsets[subset_index];

        *subset
            .iter()
            .nth(rand::thread_rng().gen_range(0..subset.len()))
            .unwrap()
    }
}
