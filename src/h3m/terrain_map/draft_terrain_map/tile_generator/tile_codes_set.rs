use crate::common::index_multiset::IndexMultiset;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct TileCodesSet {
    subsets: Vec<IndexMultiset<u8>>,
    subset_indexes: IndexMultiset<usize>,
}

impl TileCodesSet {
    pub fn new(codes: RangeInclusive<u8>) -> TileCodesSet {
        TileCodesSet::with_frequency(codes, 1)
    }

    pub fn from_code(code: u8) -> TileCodesSet {
        TileCodesSet::new(code..=code)
    }

    pub fn with_frequency(codes: RangeInclusive<u8>, frequency: usize) -> TileCodesSet {
        TileCodesSet {
            subsets: Vec::new(),
            subset_indexes: IndexMultiset::new(),
        }
        .add_codes(codes, frequency)
    }

    pub fn add_codes(mut self, codes: RangeInclusive<u8>, frequency: usize) -> TileCodesSet {
        let new_subset_index = self.subsets.len();

        self.subsets.push({
            let mut subset = IndexMultiset::new();
            for code in codes {
                subset.add_index(code, 1);
            }
            subset
        });

        if frequency > 0 {
            self.subset_indexes.add_index(new_subset_index, frequency);
        }
        self
    }

    pub fn random_not_excluded_code(&self, excluded_codes: &[u8]) -> Option<u8> {
        let subset_index = self.subset_indexes.random_index().unwrap();
        let mut subset = self.subsets[subset_index].clone();

        for &code in excluded_codes {
            subset.remove_index(code);
        }
        subset.random_index()
    }

    pub fn random_code(&self) -> u8 {
        let subset_index = self.subset_indexes.random_index().unwrap();
        let subset = &self.subsets[subset_index];
        subset.random_index().unwrap()
    }
}
