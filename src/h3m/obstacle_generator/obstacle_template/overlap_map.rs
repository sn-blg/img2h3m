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
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // DIRT
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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
                    &[(1, 0), (-1, 0), (0, -2), (0, 2), (1, 1), (-1, -1)],
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
                    &[(-1, 0), (1, 0), (0, 2), (0, -2), (-1, -1), (1, 1)],
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

            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // SAND
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            "AVLmtds1.def" => {
                overlap_map.add(
                    "AVLmtds2.def",
                    &[
                        (2, -2),
                        (1, -2),
                        (2, -1),
                        (1, -3),
                        (2, -3),
                        (1, -4),
                        (2, 1),
                        (2, 2),
                        (-1, 1),
                        (-2, 1),
                        (-1, 2),
                        (-2, 2),
                        (-1, 3),
                        (-1, 4),
                    ],
                );
                overlap_map.add("AVLmtds3.def", &[(-1, 1), (0, 3), (0, 4), (0, -2)]);
                overlap_map.add("AVLmtds4.def", &[(0, 3), (0, -1), (-1, 1)]);
                overlap_map.add("AVLmtds5.def", &[(-1, -1), (-1, -2), (1, -1)]);
                overlap_map.add(
                    "AVLmtds6.def",
                    &[(0, -2), (-1, 0), (-1, 1), (0, 3), (-1, -1)],
                );
            }
            "AVLmtds2.def" => {
                overlap_map.add(
                    "AVLmtds1.def",
                    &[
                        (-2, 2),
                        (-1, 2),
                        (-2, 1),
                        (-1, 3),
                        (-2, 3),
                        (-1, 4),
                        (-2, -1),
                        (-2, -2),
                        (1, -1),
                        (2, -1),
                        (1, -2),
                        (2, -2),
                        (1, -3),
                        (1, -4),
                    ],
                );
                overlap_map.add(
                    "AVLmtds3.def",
                    &[(1, 3), (1, -1), (-1, 2), (-1, 3), (0, -1)],
                );
                overlap_map.add(
                    "AVLmtds4.def",
                    &[(1, -1), (2, -1), (-1, 2), (-1, 3), (-1, 4)],
                );
                overlap_map.add("AVLmtds5.def", &[(1, -1), (2, -1), (-1, 2), (0, -1)]);
                overlap_map.add(
                    "AVLmtds6.def",
                    &[(0, 0), (0, -1), (-1, 1), (-1, 2), (-1, 3), (2, -1)],
                );
            }
            "AVLmtds3.def" => {
                overlap_map.add("AVLmtds1.def", &[(1, -1), (0, -3), (0, -4), (0, 2)]);
                overlap_map.add(
                    "AVLmtds2.def",
                    &[(-1, -3), (-1, 1), (1, -2), (1, -3), (0, 1)],
                );
                overlap_map.add(
                    "AVLmtds4.def",
                    &[(1, 0), (-1, 0), (-1, -1), (1, -1), (1, -2)],
                );
                overlap_map.add("AVLmtds5.def", &[(-1, -1), (0, -1), (1, -1)]);
                overlap_map.add(
                    "AVLmtds6.def",
                    &[(-1, 0), (1, -1), (0, -1), (1, -2), (-1, -1), (0, -2)],
                );
            }
            "AVLmtds4.def" => {
                overlap_map.add("AVLmtds1.def", &[(0, -3), (0, 1), (1, -1)]);
                overlap_map.add(
                    "AVLmtds2.def",
                    &[(-1, 1), (-2, 1), (1, -2), (1, -3), (1, -4)],
                );
                overlap_map.add("AVLmtds3.def", &[(-1, 0), (1, 0), (1, 1), (-1, 1), (-1, 2)]);
                overlap_map.add("AVLmtds5.def", &[(1, -1), (1, -2), (-1, 0), (-1, -1)]);
                overlap_map.add("AVLmtds6.def", &[(0, -1), (1, -1), (0, 1), (0, 2), (-1, 0)]);
            }
            "AVLmtds5.def" => {
                overlap_map.add("AVLmtds1.def", &[(1, 1), (1, 2), (-1, 1)]);
                overlap_map.add("AVLmtds2.def", &[(-1, 1), (-2, 1), (1, -2), (0, 1)]);
                overlap_map.add("AVLmtds3.def", &[(1, 1), (0, 1), (-1, 1)]);
                overlap_map.add("AVLmtds4.def", &[(-1, 1), (-1, 2), (1, 0), (1, 1)]);
                overlap_map.add(
                    "AVLmtds6.def",
                    &[(-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1)],
                );
            }
            "AVLmtds6.def" => {
                overlap_map.add(
                    "AVLmtds1.def",
                    &[(0, 1), (0, 2), (1, 0), (1, -1), (0, -3), (0, -4), (1, 1)],
                );
                overlap_map.add(
                    "AVLmtds2.def",
                    &[
                        (0, 0),
                        (0, 1),
                        (1, -1),
                        (1, -2),
                        (1, -3),
                        (1, -4),
                        (-1, 1),
                        (-2, 1),
                    ],
                );
                overlap_map.add(
                    "AVLmtds3.def",
                    &[(1, 0), (1, -1), (-1, 1), (0, 1), (-1, 2), (1, 1), (0, -2)],
                );
                overlap_map.add(
                    "AVLmtds4.def",
                    &[(0, 0), (0, 1), (-1, 1), (0, -1), (0, -2), (1, 0)],
                );
                overlap_map.add(
                    "AVLmtds5.def",
                    &[(1, 0), (-1, 0), (0, -2), (1, 1), (-1, -1), (-1, 1), (1, -1)],
                );
            }

            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // GRASS
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            "AVLmtgn0.def" => {
                overlap_map.add("AVLmtgn1.def", &[(-1, 2), (-1, 3)]);
                overlap_map.add("AVLmtgn2.def", &[(2, -1), (2, -2), (-1, 3)]);
                overlap_map.add(
                    "AVLmtgn3.def",
                    &[(2, -1), (2, 2), (2, 3), (1, 4), (-1, 3), (0, -2)],
                );
                overlap_map.add("AVLmtgn4.def", &[(-1, 3), (2, -2)]);
                overlap_map.add("AVLmtgn5.def", &[(0, -1), (-1, 1)]);
                overlap_map.add("AVLmtgr1.def", &[(2, -2), (2, 3), (-2, 3)]);
                overlap_map.add("AVLmtgr4.def", &[(2, -2)]);
                overlap_map.add("grsmnt02.def", &[(2, -1), (2, -2), (-1, 4), (-1, 3)]);
                overlap_map.add("grsmnt03.def", &[(0, -2)]);
                overlap_map.add("grsmnt04.def", &[(2, -1), (2, -2), (-1, 3), (-1, 4)]);
                overlap_map.add("grsmnt05.def", &[(-1, 2), (-1, 1)]);
            }
            "AVLmtgn1.def" => {
                overlap_map.add("AVLmtgn0.def", &[(1, -2), (1, -3)]);
                overlap_map.add("AVLmtgn2.def", &[(-1, 0), (1, -1), (1, -2), (2, 3), (0, 3)]);
                overlap_map.add("AVLmtgn3.def", &[(-1, -1), (2, 3), (2, 4)]);
                overlap_map.add("AVLmtgn4.def", &[(-1, 1), (1, -2)]);
                overlap_map.add("AVLmtgn5.def", &[(-1, -2), (-1, 1)]);
                overlap_map.add("grsmnt03.def", &[(2, 3), (2, 4)]);
                overlap_map.add("grsmnt04.def", &[(-1, 1), (0, 3)]);
                overlap_map.add("grsmnt05.def", &[(2, 3), (2, 4)]);
            }
            "AVLmtgn2.def" => {
                overlap_map.add("AVLmtgn0.def", &[(-2, 1), (-2, 2), (1, -3)]);
                overlap_map.add(
                    "AVLmtgn1.def",
                    &[(1, 0), (-1, 1), (-1, 2), (-2, -3), (0, -3)],
                );
                overlap_map.add("AVLmtgn3.def", &[(-1, 2)]);
                overlap_map.add("AVLmtgn4.def", &[(-1, 1)]);
                overlap_map.add("AVLmtgn5.def", &[(-1, 1), (1, 1)]);
                overlap_map.add("grsmnt02.def", &[(-1, 1)]);
                overlap_map.add("grsmnt05.def", &[(1, 1)]);
            }
            "AVLmtgn3.def" => {
                overlap_map.add(
                    "AVLmtgn0.def",
                    &[(-2, 1), (-2, -2), (-2, -3), (-1, -4), (1, -3), (0, 2)],
                );
                overlap_map.add("AVLmtgn1.def", &[(1, 1), (-2, -3), (-2, -4), (-1, -4)]);
                overlap_map.add("AVLmtgn2.def", &[(1, -2)]);
                overlap_map.add(
                    "AVLmtgn4.def",
                    &[(1, 1), (1, -2), (-1, -1), (-1, 0), (-1, 1)],
                );
                overlap_map.add("AVLmtgn5.def", &[(1, -1), (1, 1), (-1, -1)]);
            }
            "AVLmtgn4.def" => {
                overlap_map.add("AVLmtgn0.def", &[(1, -3), (-2, 2)]);
                overlap_map.add("AVLmtgn1.def", &[(0, -4), (1, -1), (-1, 2)]);
                overlap_map.add("AVLmtgn2.def", &[(-1, 2), (1, -2), (1, -1)]);
                overlap_map.add(
                    "AVLmtgn3.def",
                    &[(-1, -1), (-1, 2), (1, 1), (1, 0), (1, -1)],
                );
                overlap_map.add("AVLmtgn5.def", &[(1, 1), (-1, -1), (1, -1)]);
            }
            "AVLmtgn5.def" => {
                overlap_map.add("AVLmtgn0.def", &[(0, 1), (1, 0), (1, -1)]);
                overlap_map.add("AVLmtgn1.def", &[(1, 2), (1, -1)]);
                overlap_map.add("AVLmtgn2.def", &[(1, -1), (1, 1), (-1, -1)]);
                overlap_map.add("AVLmtgn3.def", &[(-1, 1), (-1, -1), (1, 1)]);
                overlap_map.add("AVLmtgn4.def", &[(-1, -1), (1, 1)]);
            }

            "AVLmtgr1.def" => {
                overlap_map.add("AVLmtgn0.def", &[(-2, 2), (-2, -3), (2, -3)]);
                overlap_map.add(
                    "AVLmtgr2.def",
                    &[(-2, -1), (2, -1), (2, -2), (2, -3), (-1, 3)],
                );
                overlap_map.add("AVLmtgr3.def", &[(2, 3), (-1, -1)]);
                overlap_map.add("AVLmtgr4.def", &[(1, -1), (1, -2), (2, -2)]);
                overlap_map.add("AVLmtgr6.def", &[(-1, 1), (-1, 2), (1, -2), (0, 3)]);
            }
            "AVLmtgr2.def" => {
                overlap_map.add(
                    "AVLmtgr1.def",
                    &[(2, 1), (-2, 1), (-2, 2), (-2, 3), (1, -3)],
                );
                overlap_map.add("AVLmtgr3.def", &[(2, 2), (0, -1)]);
                overlap_map.add("AVLmtgr4.def", &[(2, -1), (-1, 3)]);
                overlap_map.add("AVLmtgr5.def", &[(-1, 1)]);
            }
            "AVLmtgr3.def" => {
                overlap_map.add("AVLmtgr1.def", &[(-2, -3), (1, 1)]);
                overlap_map.add("AVLmtgr2.def", &[(-2, -2), (0, 1)]);
                overlap_map.add("AVLmtgr4.def", &[(1, -2)]);
                overlap_map.add("AVLmtgr6.def", &[(1, -1), (1, -2)]);
            }
            "AVLmtgr4.def" => {
                overlap_map.add("AVLmtgn0.def", &[(-2, 2)]);
                overlap_map.add("AVLmtgr1.def", &[(-1, 1), (-1, 2), (-2, 2)]);
                overlap_map.add("AVLmtgr2.def", &[(-2, 1), (1, -3)]);
                overlap_map.add("AVLmtgr3.def", &[(-1, 2)]);
                overlap_map.add("AVLmtgr5.def", &[(0, 1)]);
            }
            "AVLmtgr5.def" => {
                overlap_map.add("AVLmtgr2.def", &[(1, -1)]);
                overlap_map.add("AVLmtgr3.def", &[(1, 2)]);
                overlap_map.add("AVLmtgr4.def", &[(0, -1)]);
                overlap_map.add("AVLmtgr6.def", &[(1, -1), (-1, 1)]);
            }
            "AVLmtgr6.def" => {
                overlap_map.add(
                    "AVLmtgr1.def",
                    &[(1, -1), (1, -2), (-1, 2), (0, -3), (0, -4)],
                );
                overlap_map.add("AVLmtgr2.def", &[(1, -3), (2, 1)]);
                overlap_map.add("AVLmtgr3.def", &[(-1, 1), (-1, 2)]);
                overlap_map.add("AVLmtgr4.def", &[(1, -2)]);
                overlap_map.add("AVLmtgr5.def", &[(-1, 1), (1, -1)]);
            }

            "grsmnt00.def" => {
                overlap_map.add("grsmnt01.def", &[(-1, 2), (-1, -3), (2, 3)]);
                overlap_map.add("grsmnt02.def", &[(2, -1), (2, -2), (-1, 4), (-1, 3)]);
                overlap_map.add("grsmnt04.def", &[(2, -1), (2, -2), (-1, 3)]);
                overlap_map.add("grsmnt05.def", &[(-1, 1), (-1, 2), (1, 4)]);
            }
            "grsmnt01.def" => {
                overlap_map.add("grsmnt00.def", &[(1, -2), (1, 3), (-2, -3)]);
                overlap_map.add("grsmnt02.def", &[(0, 4), (2, 0)]);
                overlap_map.add("grsmnt03.def", &[(2, 4), (2, 3), (-1, -1), (-1, -2)]);
                overlap_map.add("grsmnt04.def", &[(-1, 1), (0, 3), (1, -1), (1, -2)]);
                overlap_map.add("grsmnt05.def", &[(-1, -1)]);
            }
            "grsmnt02.def" => {
                overlap_map.add("AVLmtgn0.def", &[(-2, 1), (-2, 2), (1, -4), (1, -3)]);
                overlap_map.add("AVLmtgn2.def", &[(-1, 1)]);
                overlap_map.add("grsmnt00.def", &[(-2, 1), (-2, 2), (1, -4), (1, -3)]);
                overlap_map.add("grsmnt01.def", &[(0, -4), (-2, 0)]);
                overlap_map.add("grsmnt03.def", &[(1, 1), (1, 2)]);
                overlap_map.add("grsmnt04.def", &[(-1, 1), (1, -1)]);
                overlap_map.add("grsmnt05.def", &[(1, 2), (-1, 0)]);
            }
            "grsmnt03.def" => {
                overlap_map.add("AVLmtgn0.def", &[(0, 2)]);
                overlap_map.add("AVLmtgn1.def", &[(-2, -3), (-2, -4)]);
                overlap_map.add("grsmnt00.def", &[(-2, -3)]);
                overlap_map.add("grsmnt01.def", &[(-2, -4), (-2, -3), (1, 1), (1, 2)]);
                overlap_map.add("grsmnt02.def", &[(-1, -1), (-1, -2)]);
                overlap_map.add("grsmnt04.def", &[(-1, 0), (1, -2)]);
                overlap_map.add("grsmnt05.def", &[(-1, -1), (-1, -2)]);
            }
            "grsmnt04.def" => {
                overlap_map.add("AVLmtgn0.def", &[(-2, 1), (-2, 2), (1, -3), (1, -4)]);
                overlap_map.add("AVLmtgn1.def", &[(1, -1), (0, -3)]);
                overlap_map.add("AVLmtgn2.def", &[(1, -2)]);
                overlap_map.add("grsmnt00.def", &[(-2, 1), (-2, 2), (1, -3), (1, -4)]);
                overlap_map.add("grsmnt01.def", &[(1, -1), (0, -3), (-1, 1), (-1, 2)]);
                overlap_map.add("grsmnt02.def", &[(1, -1), (1, -2), (-1, 1)]);
                overlap_map.add("grsmnt03.def", &[(1, 0), (-1, 2)]);
                overlap_map.add("grsmnt05.def", &[(-1, -1), (1, 1)]);
            }
            "grsmnt05.def" => {
                overlap_map.add("AVLmtgn0.def", &[(1, -2), (1, -1)]);
                overlap_map.add("AVLmtgn1.def", &[(-2, -3), (-2, -4)]);
                overlap_map.add("AVLmtgn2.def", &[(-1, -1)]);
                overlap_map.add("grsmnt00.def", &[(1, -1), (1, -2), (0, 2), (-1, -4)]);
                overlap_map.add("grsmnt01.def", &[(1, 1), (1, 2)]);
                overlap_map.add("grsmnt02.def", &[(-1, -2), (1, 0)]);
                overlap_map.add("grsmnt03.def", &[(1, 1), (1, 2)]);
                overlap_map.add("grsmnt04.def", &[(1, 1), (-1, -1)]);
            }

            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // SNOW
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            "AVLmtsn1.def" => {
                overlap_map.add("AVLmtsn2.def", &[(-1, -2), (-2, -2), (2, -3)]);
                overlap_map.add("AVLmtsn3.def", &[(2, 3), (-1, -1)]);
                overlap_map.add("AVLmtsn4.def", &[(0, 3), (-1, 0), (1, -1), (1, -2)]);
                overlap_map.add("AVLmtsn5.def", &[(-1, -1)]);
                overlap_map.add("AVLmtsn6.def", &[(1, -2), (-1, 1)]);
            }
            "AVLmtsn2.def" => {
                overlap_map.add("AVLmtsn1.def", &[(1, 2), (2, 2), (-2, 3)]);
                overlap_map.add("AVLmtsn3.def", &[(2, 1), (2, 2), (2, 3), (-1, 1), (0, -1)]);
                overlap_map.add("AVLmtsn4.def", &[(-1, 3), (2, -1), (2, -2)]);
                overlap_map.add(
                    "AVLmtsn5.def",
                    &[(1, 4), (0, -1), (2, -1), (-1, 3), (-1, 1)],
                );
                overlap_map.add("AVLmtsn6.def", &[(-1, 3), (2, -2)]);
            }
            "AVLmtsn3.def" => {
                overlap_map.add("AVLmtsn1.def", &[(-2, -3), (1, 1), (1, 2)]);
                overlap_map.add(
                    "AVLmtsn2.def",
                    &[(-2, -1), (-2, -2), (-2, -3), (1, -1), (0, 1)],
                );
                overlap_map.add("AVLmtsn4.def", &[(-1, -1), (1, 1), (-1, 1)]);
                overlap_map.add("AVLmtsn5.def", &[(-1, -1), (-1, -2), (1, 1)]);
                overlap_map.add("AVLmtsn6.def", &[(1, -2), (0, 2), (-1, 1)]);
            }
            "AVLmtsn4.def" => {
                overlap_map.add(
                    "AVLmtsn1.def",
                    &[(0, -3), (0, -4), (1, 0), (-1, 1), (-1, 2)],
                );
                overlap_map.add("AVLmtsn2.def", &[(1, -3), (1, -4), (-2, 1), (-2, 2)]);
                overlap_map.add("AVLmtsn3.def", &[(1, 1), (-1, -1), (1, -1)]);
                overlap_map.add(
                    "AVLmtsn5.def",
                    &[(-1, 1), (1, -1), (1, 1), (-1, -1), (1, 2), (0, -2)],
                );
                overlap_map.add("AVLmtsn6.def", &[(1, -1), (1, -2), (-1, 2), (-1, 1)]);
            }
            "AVLmtsn5.def" => {
                overlap_map.add("AVLmtsn1.def", &[(1, 1)]);
                overlap_map.add(
                    "AVLmtsn2.def",
                    &[(-1, -4), (0, 1), (-2, 1), (1, -3), (1, -1)],
                );
                overlap_map.add("AVLmtsn3.def", &[(1, 1), (1, 2), (-1, -1)]);
                overlap_map.add(
                    "AVLmtsn4.def",
                    &[(1, -1), (-1, 1), (-1, -1), (1, 1), (-1, -2), (0, 2)],
                );
                overlap_map.add("AVLmtsn6.def", &[(1, -1)]);
            }
            "AVLmtsn6.def" => {
                overlap_map.add("AVLmtsn1.def", &[(0, -4), (-1, 2), (1, -1)]);
                overlap_map.add("AVLmtsn2.def", &[(1, -3), (-2, 2)]);
                overlap_map.add("AVLmtsn3.def", &[(-1, 2), (0, -2), (1, -1)]);
                overlap_map.add("AVLmtsn4.def", &[(-1, 1), (-1, 2), (1, -2), (1, -1)]);
                overlap_map.add("AVLmtsn5.def", &[(-1, 1), (1, -1)]);
            }

            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // SWAMP
            ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            "AVLmtsw1.def" => {
                overlap_map.add(
                    "AVLmtsw2.def",
                    &[
                        (-2, -3),
                        (2, 2),
                        (2, -3),
                        (-2, 2),
                        (-2, 1),
                        (-1, 1),
                        (-1, 2),
                        (-1, -2),
                        (1, -1),
                        (1, -2),
                        (1, -3),
                        (1, -4),
                        (1, 3),
                    ],
                );
                overlap_map.add(
                    "AVLmtsw3.def",
                    &[(-1, -1), (2, 3), (2, 4), (1, -1), (-1, 1)],
                );
                overlap_map.add(
                    "AVLmtsw4.def",
                    &[(2, -1), (-1, 1), (0, 3), (2, 3), (-1, -1)],
                );
                overlap_map.add(
                    "AVLmtsw5.def",
                    &[(1, 4), (2, 3), (2, 4), (1, -1), (0, 2), (-1, -1)],
                );
                overlap_map.add("AVLmtsw6.def", &[(-1, 1), (-1, 2), (0, -1)]);
            }
            "AVLmtsw2.def" => {
                overlap_map.add(
                    "AVLmtsw1.def",
                    &[
                        (2, 3),
                        (-2, -2),
                        (-2, 3),
                        (2, -2),
                        (2, -1),
                        (1, -1),
                        (1, -2),
                        (1, 2),
                        (-1, 1),
                        (-1, 2),
                        (-1, 3),
                        (-1, 4),
                        (-1, -3),
                    ],
                );
                overlap_map.add(
                    "AVLmtsw3.def",
                    &[(2, -1), (2, 2), (2, 3), (2, 4), (-1, 2), (-1, 3), (-1, 4)],
                );
                overlap_map.add(
                    "AVLmtsw4.def",
                    &[(2, -1), (-1, 1), (2, 1), (-1, 3), (2, -2)],
                );
                overlap_map.add("AVLmtsw5.def", &[(0, -1), (-1, 0), (1, -2)]);
                overlap_map.add("AVLmtsw6.def", &[(-1, 3), (-1, 4), (2, -2)]);
            }
            "AVLmtsw3.def" => {
                overlap_map.add(
                    "AVLmtsw1.def",
                    &[(1, 1), (-2, -3), (-2, -4), (-1, 1), (1, -1)],
                );
                overlap_map.add(
                    "AVLmtsw2.def",
                    &[
                        (-2, 1),
                        (-2, -2),
                        (-2, -3),
                        (-2, -4),
                        (1, -2),
                        (1, -3),
                        (1, -4),
                    ],
                );
                overlap_map.add("AVLmtsw4.def", &[(1, -1), (1, -2), (0, 2), (-1, -1)]);
                overlap_map.add("AVLmtsw6.def", &[(1, -1), (1, -2), (-1, 0), (-1, 1)]);
            }
            "AVLmtsw4.def" => {
                overlap_map.add(
                    "AVLmtsw1.def",
                    &[(-2, 1), (1, -1), (0, -3), (0, -4), (-2, -3), (1, 1)],
                );
                overlap_map.add(
                    "AVLmtsw2.def",
                    &[(-2, 1), (1, -1), (-2, -1), (1, -3), (1, -4), (-2, 2)],
                );
                overlap_map.add("AVLmtsw3.def", &[(-1, 1), (-1, 2), (0, -2), (1, 1)]);
                overlap_map.add("AVLmtsw5.def", &[(-1, 0)]);
                overlap_map.add("AVLmtsw6.def", &[(-1, 1), (1, -1)]);
            }
            "AVLmtsw5.def" => {
                overlap_map.add(
                    "AVLmtsw1.def",
                    &[(-1, -4), (-2, -3), (-2, -4), (-1, 1), (0, -2), (1, 1)],
                );
                overlap_map.add("AVLmtsw2.def", &[(0, 1), (1, 0), (0, -3), (-1, 2)]);
                overlap_map.add("AVLmtsw3.def", &[(1, 2)]);
                overlap_map.add("AVLmtsw4.def", &[(1, 0)]);
                overlap_map.add("AVLmtsw6.def", &[(-1, 1), (1, -1), (-1, -1)]);
            }
            "AVLmtsw6.def" => {
                overlap_map.add("AVLmtsw1.def", &[(0, -4), (1, -1), (1, -2), (0, 1)]);
                overlap_map.add("AVLmtsw2.def", &[(-1, 1), (1, -3), (1, -4), (-2, 2)]);
                overlap_map.add("AVLmtsw3.def", &[(-1, 1), (-1, 2), (1, 0), (1, -1)]);
                overlap_map.add("AVLmtsw4.def", &[(1, -1), (1, -2), (-1, 1)]);
                overlap_map.add("AVLmtsw5.def", &[(1, -1), (-1, 1), (1, 1), (-1, -1)]);
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
