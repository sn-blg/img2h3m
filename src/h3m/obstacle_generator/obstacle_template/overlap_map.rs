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
                overlap_map.add("avlmtdr5.def", &[(2, 3)]);
                overlap_map.add("avlmtdr6.def", &[(-1, 0), (2, 0), (0, 3)]);
                overlap_map.add("avlmtdr7.def", &[(-1, -2), (2, 2), (0, -4), (0, 4)]);
                overlap_map.add(
                    "avlmtdr8.def",
                    &[
                        (-1, -2),
                        (-1, -1),
                        (-1, 0),
                        (2, -1),
                        (1, -3),
                        (2, -3),
                        (-1, 1),
                    ],
                );
            }
            "avlmtdr2.def" => {
                overlap_map.add(
                    "avlmtdr1.def",
                    &[(-1, 2), (1, -4), (-2, 3), (-2, 2), (2, 3), (2, 2)],
                );
                overlap_map.add("avlmtdr3.def", &[(-1, 3), (-1, 4)]);
                overlap_map.add("avlmtdr4.def", &[(-1, 2), (2, -1)]);
                overlap_map.add("avlmtdr5.def", &[(2, 2)]);
                overlap_map.add("avlmtdr6.def", &[(2, -1), (-1, 2)]);
                overlap_map.add("avlmtdr7.def", &[(2, 1), (-1, 2)]);
                overlap_map.add(
                    "avlmtdr8.def",
                    &[(-1, 1), (-1, 2), (2, -1), (2, -2), (2, -3)],
                );
            }
            "avlmtdr3.def" => {
                overlap_map.add(
                    "avlmtdr1.def",
                    &[(0, -3), (-2, 0), (-2, -1), (-2, -2), (-2, -3), (-2, -4)],
                );
                overlap_map.add("avlmtdr2.def", &[(1, -3), (1, -4)]);
                overlap_map.add("avlmtdr4.def", &[(-1, -1), (1, -1)]);
                overlap_map.add("avlmtdr6.def", &[(-1, -1), (1, -1), (-1, 0)]);
                overlap_map.add("avlmtdr7.def", &[(-1, -4), (0, -4), (1, -1)]);
                overlap_map.add("avlmtdr8.def", &[(-1, -1), (-1, -2), (1, -3)]);
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
                overlap_map.add("avlmtdr5.def", &[(1, 1)]);
                overlap_map.add("avlmtdr6.def", &[(1, -1), (-1, 1)]);
                overlap_map.add("avlmtdr7.def", &[(-1, 0)]);
                overlap_map.add("avlmtdr8.def", &[(-1, 0), (-1, -1), (1, -2)]);
            }
            "avlmtdr5.def" => {
                overlap_map.add("avlmtdr1.def", &[(-2, -3)]);
                overlap_map.add("avlmtdr2.def", &[(-2, -2)]);
                overlap_map.add("avlmtdr4.def", &[(-1, -1)]);
                overlap_map.add("avlmtdr6.def", &[(-1, -1), (1, 0), (-1, 0)]);
                overlap_map.add("avlmtdr7.def", &[(-1, -4)]);
                overlap_map.add(
                    "avlmtdr8.def",
                    &[(0, -4), (-1, -1), (-1, -2), (-1, 1), (1, -2), (1, -3)],
                );
            }
            "avlmtdr6.def" => {
                overlap_map.add("avlmtdr1.def", &[(1, 0), (-2, 0), (0, -3)]);
                overlap_map.add("avlmtdr2.def", &[(-2, 1), (1, -2)]);
                overlap_map.add("avlmtdr3.def", &[(1, 1), (-1, 1), (1, 0)]);
                overlap_map.add("avlmtdr4.def", &[(-1, 1), (1, -1)]);
                overlap_map.add("avlmtdr5.def", &[(1, 1), (-1, 0), (1, 0), (1, -1)]);
                overlap_map.add("avlmtdr7.def", &[(0, -3), (0, -4)]);
                overlap_map.add("avlmtdr8.def", &[(1, -1), (1, -2), (1, -3), (1, -4)]);
            }
            "avlmtdr7.def" => {
                overlap_map.add("avlmtdr1.def", &[(1, 2), (-2, -2), (0, 4), (0, -4)]);
                overlap_map.add("avlmtdr2.def", &[(-2, -1), (1, -2)]);
                overlap_map.add("avlmtdr3.def", &[(0, 4), (-1, 1)]);
                overlap_map.add("avlmtdr4.def", &[(1, 0), (0, -1)]);
                overlap_map.add("avlmtdr5.def", &[(-1, -3), (1, 4)]);
                overlap_map.add("avlmtdr6.def", &[(0, 3), (0, -1), (0, -2)]);
                overlap_map.add(
                    "avlmtdr8.def",
                    &[(-1, -1), (-1, -2), (0, -2), (0, -3), (1, 2), (-1, 1)],
                );
            }
            "avlmtdr8.def" => {
                overlap_map.add(
                    "avlmtdr1.def",
                    &[(1, 2), (1, 1), (1, 0), (-2, 1), (-1, 3), (-2, 3), (1, -1)],
                );
                overlap_map.add(
                    "avlmtdr2.def",
                    &[(1, -1), (1, -2), (-2, 1), (-2, 2), (-2, 3)],
                );
                overlap_map.add(
                    "avlmtdr3.def",
                    &[(1, 1), (1, 2), (-1, 3), (0, 2), (0, 3), (0, -2)],
                );
                overlap_map.add("avlmtdr4.def", &[(1, 0), (1, 1), (1, -1), (-1, 2)]);
                overlap_map.add(
                    "avlmtdr5.def",
                    &[
                        (0, 4),
                        (1, 1),
                        (1, 2),
                        (1, -1),
                        (0, -2),
                        (0, 3),
                        (-1, 2),
                        (-1, 3),
                    ],
                );
                overlap_map.add(
                    "avlmtdr6.def",
                    &[(-1, 1), (-1, 2), (-1, 3), (-1, 4), (1, -1), (1, -2)],
                );
                overlap_map.add(
                    "avlmtdr7.def",
                    &[(1, 1), (1, 2), (0, 2), (-1, -2), (1, -1), (0, -3), (0, -4)],
                );
            }

            "AVLPNTR2.def" => {
                overlap_map.add("AVLPNTR3.def", &[(-1, 1), (1, -1)]);
                overlap_map.add("AVLPNTR4.def", &[(1, 0), (-1, 0), (1, 1), (-1, -1)]);
                overlap_map.add(
                    "AVLPNTR5.def",
                    &[(1, 0), (-1, 0), (0, -2), (0, 2), (1, 1), (-1, -1)],
                );
                overlap_map.add(
                    "AVLpntr6.def",
                    &[(0, 1), (0, -2), (1, 0), (-2, 0), (-2, -1), (1, 1)],
                );
                overlap_map.add("AVLpntr7.def", &[(1, -1), (-2, 2), (-1, -1)]);
            }
            "AVLPNTR3.def" => {
                overlap_map.add("AVLPNTR2.def", &[(-1, 1), (1, -1)]);
                overlap_map.add(
                    "AVLPNTR4.def",
                    &[(1, 0), (-1, 0), (0, -2), (1, 1), (-1, -1)],
                );
                overlap_map.add(
                    "AVLPNTR5.def",
                    &[(1, 0), (-1, 0), (0, -2), (0, 1), (0, 2), (1, 1), (-1, -1)],
                );
                overlap_map.add("AVLpntr6.def", &[(1, 0), (1, 1), (-2, -1), (1, -1)]);
                overlap_map.add("AVLpntr7.def", &[(1, -1), (-2, 1), (-1, -1)]);
            }
            "AVLPNTR4.def" => {
                overlap_map.add("AVLPNTR2.def", &[(-1, 0), (1, 0), (-1, -1), (1, 1)]);
                overlap_map.add("AVLPNTR3.def", &[(-1, 0), (1, 0), (0, 2), (-1, -1), (1, 1)]);
                overlap_map.add("AVLPNTR5.def", &[(-1, -1), (1, 1), (1, 2), (-1, -2)]);
                overlap_map.add(
                    "AVLpntr6.def",
                    &[(1, 1), (-2, -2), (-2, 1), (-2, -1), (1, -1), (1, 2)],
                );
                overlap_map.add(
                    "AVLpntr7.def",
                    &[(1, 1), (-1, -1), (-2, -1), (-1, 2), (1, -1)],
                );
            }
            "AVLPNTR5.def" => {
                overlap_map.add(
                    "AVLPNTR2.def",
                    &[(-1, 0), (1, 0), (0, 2), (0, -2), (-1, -1), (1, 1)],
                );
                overlap_map.add(
                    "AVLPNTR3.def",
                    &[(-1, 0), (1, 0), (0, 2), (0, -1), (0, -2), (-1, -1), (1, 1)],
                );
                overlap_map.add("AVLPNTR4.def", &[(1, 1), (-1, -1), (-1, -2), (1, 2)]);
                overlap_map.add(
                    "AVLpntr6.def",
                    &[(1, 1), (-1, -1), (-2, -1), (-2, -2), (1, 2), (-1, 1)],
                );
                overlap_map.add(
                    "AVLpntr7.def",
                    &[
                        (1, 1),
                        (-1, -1),
                        (0, 2),
                        (1, -1),
                        (-1, 1),
                        (-2, -1),
                        (-2, 1),
                    ],
                );
            }
            "AVLpntr6.def" => {
                overlap_map.add(
                    "AVLPNTR2.def",
                    &[(0, -1), (-1, 0), (2, 0), (2, 1), (-1, -1)],
                );
                overlap_map.add("AVLPNTR3.def", &[(-1, 0), (-1, -1), (2, 1), (-1, 1)]);
                overlap_map.add(
                    "AVLPNTR4.def",
                    &[(-1, -1), (2, 2), (2, -1), (2, 1), (-1, 1), (-1, -2)],
                );
                overlap_map.add(
                    "AVLPNTR5.def",
                    &[(-1, -1), (1, 1), (2, 1), (2, 2), (-1, -2), (1, -1)],
                );
                overlap_map.add(
                    "AVLpntr7.def",
                    &[
                        (1, -1),
                        (-1, 1),
                        (1, 1),
                        (-1, -1),
                        (-2, 1),
                        (-2, -1),
                        (2, 1),
                        (2, -1),
                        (0, 2),
                    ],
                );
            }
            "AVLpntr7.def" => {
                overlap_map.add("AVLPNTR2.def", &[(-1, 1), (2, -2), (1, 1)]);
                overlap_map.add("AVLPNTR3.def", &[(-1, 1), (2, -1), (1, 1)]);
                overlap_map.add(
                    "AVLPNTR4.def",
                    &[(-1, -1), (1, 1), (2, 1), (1, -2), (-1, 1)],
                );
                overlap_map.add(
                    "AVLPNTR5.def",
                    &[(-1, -1), (1, 1), (0, -2), (-1, 1), (1, -1), (2, 1), (2, -1)],
                );
                overlap_map.add(
                    "AVLpntr6.def",
                    &[
                        (-1, 1),
                        (1, -1),
                        (-1, -1),
                        (1, 1),
                        (2, -1),
                        (2, 1),
                        (-2, -1),
                        (-2, 1),
                        (0, -2),
                    ],
                );
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
