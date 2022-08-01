use crate::common::position::generic::SignedDeltaPos;
use std::collections::{HashMap, HashSet};

pub struct OverlapMap(HashMap<&'static str, HashSet<SignedDeltaPos<isize>>>);

impl OverlapMap {
    fn add(&mut self, overlapped_filename: &'static str, delta_pos_arr: &[(isize, isize)]) {
        self.0.insert(
            overlapped_filename,
            delta_pos_arr
                .iter()
                .map(|&(row, column)| SignedDeltaPos::new(row, column))
                .collect(),
        );
    }

    pub fn new(overlap_filename: &'static str) -> Self {
        let mut overlap_map = OverlapMap(HashMap::new());

        match overlap_filename {
            "avlmtdr1.def" => {
                overlap_map.add(
                    "avlmtdr2.def",
                    &[(-2, -2), (-2, -3), (2, -2), (2, -3), (-1, 4), (1, -2)],
                );
                overlap_map.add(
                    "avlmtdr3.def",
                    &[(0, 3), (2, 0), (2, 1), (2, 2), (2, 3), (2, 4)],
                );
                overlap_map.add(
                    "avlmtdr4.def",
                    &[(2, 2), (2, 1), (2, 0), (2, -1), (2, -2), (-1, 0), (0, 3)],
                );
            }
            "avlmtdr2.def" => {
                overlap_map.add(
                    "avlmtdr1.def",
                    &[(-1, 2), (1, -4), (-2, 3), (-2, 2), (2, 3), (2, 2)],
                );
                overlap_map.add("avlmtdr3.def", &[(-1, 3), (-1, 4)]);
                overlap_map.add("avlmtdr4.def", &[(-1, 2), (2, -1)]);
            }
            "avlmtdr3.def" => {
                overlap_map.add(
                    "avlmtdr1.def",
                    &[(0, -3), (-2, 0), (-2, -1), (-2, -2), (-2, -3), (-2, -4)],
                );
                overlap_map.add("avlmtdr2.def", &[(1, -3), (1, -4)]);
                overlap_map.add("avlmtdr4.def", &[(-1, -1), (1, -1)]);
            }
            "avlmtdr4.def" => {
                overlap_map.add(
                    "avlmtdr1.def",
                    &[
                        (-2, -2),
                        (-2, -1),
                        (-2, 0),
                        (-2, 1),
                        (-2, 2),
                        (1, 0),
                        (0, -3),
                    ],
                );
                overlap_map.add("avlmtdr2.def", &[(1, -2), (-2, 1)]);
                overlap_map.add("avlmtdr3.def", &[(1, 1), (-1, 1)]);
            }
            _ => (),
        }

        overlap_map
    }

    pub fn may_overlap(
        &self,
        overlapped_filename: &'static str,
        delta: SignedDeltaPos<isize>,
    ) -> bool {
        let pos_set = self.0.get(overlapped_filename);
        if let Some(pos_set) = pos_set {
            pos_set.contains(&delta)
        } else {
            false
        }
    }
}
