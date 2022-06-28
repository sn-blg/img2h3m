use super::common::NEIGHBORHOOD_SIZE;
use super::draft_tile::{Orientation, TerrainVisibleType, TileComposition, TileType};
use super::terrain_relation::{NeighborhoodPattern, TerrainCategory, TerrainRelation};
use super::TileCodesSet;
use crate::h3m::Terrain;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum TileSymmetry {
    Full,
    MainDiagonal,
    SideDiagonal,
    None,
}

#[rustfmt::skip]
fn vertical_mirroring_pattern(pattern: &NeighborhoodPattern) -> NeighborhoodPattern {
    [
        pattern[5], pattern[6], pattern[7],
        pattern[3],             pattern[4],
        pattern[0], pattern[1], pattern[2],
    ]
}

#[rustfmt::skip]
fn horizontal_mirroring_pattern(pattern: &NeighborhoodPattern) -> NeighborhoodPattern {
    [
        pattern[2], pattern[1], pattern[0],
        pattern[4],             pattern[3],
        pattern[7], pattern[6], pattern[5],
    ]
}

#[rustfmt::skip]
fn rotate_pattern_with_main_diagonal_symmetry(
    pattern: &NeighborhoodPattern,
) -> NeighborhoodPattern {
    [
        pattern[0], pattern[3], pattern[5],
        pattern[1],             pattern[6],
        pattern[2], pattern[4], pattern[7],
    ]
}

#[rustfmt::skip]
fn rotate_pattern_with_side_diagonal_symmetry(
    pattern: &NeighborhoodPattern,
) -> NeighborhoodPattern {
    [
        pattern[7], pattern[4], pattern[2],
        pattern[6],             pattern[1],
        pattern[5], pattern[3], pattern[0],
    ]
}

fn additional_patterns(
    pattern: &NeighborhoodPattern,
    tile_symmetry: TileSymmetry,
) -> Vec<NeighborhoodPattern> {
    let mut additional_patterns = Vec::new();
    match tile_symmetry {
        TileSymmetry::Full => {
            additional_patterns.push(vertical_mirroring_pattern(pattern));
            additional_patterns.push(horizontal_mirroring_pattern(pattern));
            additional_patterns.push(rotate_pattern_with_main_diagonal_symmetry(pattern));
            additional_patterns.push(rotate_pattern_with_side_diagonal_symmetry(pattern))
        }
        TileSymmetry::MainDiagonal => {
            additional_patterns.push(rotate_pattern_with_main_diagonal_symmetry(pattern))
        }
        TileSymmetry::SideDiagonal => {
            additional_patterns.push(rotate_pattern_with_side_diagonal_symmetry(pattern))
        }
        TileSymmetry::None => (),
    }
    additional_patterns
}

fn expand_patterns(
    patterns: &[NeighborhoodPattern],
    tile_symmetry: TileSymmetry,
) -> Vec<NeighborhoodPattern> {
    let mut expanded_patterns = Vec::with_capacity(patterns.len());

    let mut add_pattern_if_new = |pattern| {
        if !expanded_patterns.contains(&pattern) {
            expanded_patterns.push(pattern);
        }
    };

    for &pattern in patterns {
        for additional_pattern in additional_patterns(&pattern, tile_symmetry) {
            add_pattern_if_new(additional_pattern);
        }
        add_pattern_if_new(pattern);
    }
    expanded_patterns
}

pub struct TilesGroupInfo {
    patterns: Vec<NeighborhoodPattern>,
    codes: TileCodesSet,
    composition: TileComposition,
    tile_type: TileType,
    terrain_visible_type: TerrainVisibleType,
    group_number: usize,
}

impl TilesGroupInfo {
    fn new(
        patterns: &[NeighborhoodPattern],
        codes: &TileCodesSet,
        composition: TileComposition,
        tile_type: TileType,
        terrain_visible_type: TerrainVisibleType,
        tile_symmetry: TileSymmetry,
        group_number: usize,
    ) -> TilesGroupInfo {
        TilesGroupInfo {
            patterns: expand_patterns(patterns, tile_symmetry),
            codes: codes.clone(),
            composition,
            tile_type,
            terrain_visible_type,
            group_number,
        }
    }

    pub fn patterns(&self) -> &[NeighborhoodPattern] {
        &self.patterns
    }

    pub fn codes(&self) -> &TileCodesSet {
        &self.codes
    }

    pub fn composition(&self) -> TileComposition {
        self.composition
    }

    pub fn tile_type(&self) -> TileType {
        self.tile_type
    }

    pub fn terrain_visible_type(&self) -> TerrainVisibleType {
        self.terrain_visible_type
    }

    pub fn group_number(&self) -> usize {
        self.group_number
    }
}

static VERTICAL_HALF_DIRT: TileType = TileType::HalfDiff(Orientation::Vertical, Terrain::Dirt);
static HORIZONTAL_HALF_DIRT: TileType = TileType::HalfDiff(Orientation::Horizontal, Terrain::Dirt);

static VERTICAL_HALF_SAND: TileType = TileType::HalfDiff(Orientation::Vertical, Terrain::Sand);
static HORIZONTAL_HALF_SAND: TileType = TileType::HalfDiff(Orientation::Horizontal, Terrain::Sand);

static VERTICAL_HALF_DIRT_SAND: TileType =
    TileType::HalfDiff2(Orientation::Vertical, Terrain::Dirt, Terrain::Sand);
static HORIZONTAL_HALF_DIRT_SAND: TileType =
    TileType::HalfDiff2(Orientation::Horizontal, Terrain::Dirt, Terrain::Sand);

static VERTICAL_HALF_DIRT_ARR: &[TileType] = &[VERTICAL_HALF_DIRT, VERTICAL_HALF_DIRT_SAND];
static HORIZONTAL_HALF_DIRT_ARR: &[TileType] = &[HORIZONTAL_HALF_DIRT, HORIZONTAL_HALF_DIRT_SAND];

static VERTICAL_HALF_SAND_ARR: &[TileType] = &[VERTICAL_HALF_SAND];
static HORIZONTAL_HALF_SAND_ARR: &[TileType] = &[HORIZONTAL_HALF_SAND];

pub struct TilesTable {
    inner: HashMap<Terrain, Vec<TilesGroupInfo>>,
}

impl TilesTable {
    #[rustfmt::skip]
    pub fn new(one_tile_water: bool) -> TilesTable {
        use TerrainCategory::*;
        use TerrainRelation::*;

        let dirt_table = vec![
            (
                vec![[  Any,            Other(Sandy),   DiffAny,
                        Other(Sandy),                   EqOr(Dirty),
                        DiffAny,        EqOr(Dirty),    EqOr(Dirty),
                    ],
                    [   Any,            Other(Sandy),   DiffAny,
                        Any,                            EqOr(Dirty),
                        Other(Sandy),   EqOr(Dirty),    EqOr(Dirty),
                    ],
                ],
                TileCodesSet::new(0..=3),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Any,            EqOr(Dirty),    EqOr(Dirty),
                        Other(Sandy),                   EqOr(Dirty),
                        Any,            EqOr(Dirty),    EqOr(Dirty),
                    ],
                ],
                TileCodesSet::new(4..=7),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, VERTICAL_HALF_SAND),
            ),
            (
                vec![[  Any,            Other(Sandy),   Any,
                        EqOr(Dirty),                    EqOr(Dirty),
                        EqOr(Dirty),    EqOr(Dirty),    EqOr(Dirty),
                    ],
                ],
                TileCodesSet::new(8..=11),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, HORIZONTAL_HALF_SAND),
            ),
            (
                vec![[  EqOr(Dirty),             EqOr(Dirty),                       EqOr(Dirty),
                        EqOr(Dirty),                                                SameTyped(HORIZONTAL_HALF_SAND_ARR),
                        EqOr(Dirty),             SameTyped(VERTICAL_HALF_SAND_ARR), Other(Sandy),
                    ],
                ],
                TileCodesSet::new(12..=15),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Any,            Other(Sandy),   EqOr(Dirty),
                        Other(Sandy),                   EqOr(Dirty),
                        Any,            EqOr(Dirty),    EqOr(Dirty)
                    ],
                    [   Any,            Other(Sandy),   EqOr(Dirty),
                        Any,                            EqOr(Dirty),
                        Other(Sandy),   EqOr(Dirty),    EqOr(Dirty),
                    ],
                ],
                TileCodesSet::new(16..=17),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Sand)),
            ),
            (
                vec![[  EqOr(Dirty),    EqOr(Dirty),    EqOr(Dirty),
                        EqOr(Dirty),                    EqOr(Dirty),
                        EqOr(Dirty),    EqOr(Dirty),    Other(Sandy),
                    ],
                ],
                TileCodesSet::new(18..=19),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Other(Sandy),   EqOr(Dirty),    EqOr(Dirty),
                        EqOr(Dirty),                    EqOr(Dirty),
                        EqOr(Dirty),    EqOr(Dirty),    Other(Sandy),
                    ],
                ],
                TileCodesSet::from_code(20),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[EqOr(Dirty); NEIGHBORHOOD_SIZE]],
                TileCodesSet::with_frequency(21..=28, 4).add_codes(29..=44, 1),
                (TerrainVisibleType::Same, TileSymmetry::Full, TileComposition::Main, TileType::Solid),
            ),
            (
                vec![[  Any,            Any,            Any,
                        Other(Sandy),                   Other(Sandy),
                        Any,            Any,            Any,
                    ],
                    [   Other(Sandy),   Any,            Any,
                        Any,                            Other(Sandy),
                        Any,            Other(Sandy),   Any,
                    ],
                ],
                TileCodesSet::from_code(45),
                (TerrainVisibleType::Diff(Terrain::Sand), TileSymmetry::Full, TileComposition::Main, TileType::Solid),
            ),
        ];

        let sand_table = vec![
            (
                vec![[Any; NEIGHBORHOOD_SIZE]],
                TileCodesSet::with_frequency(0..=7, 4).add_codes(8..=23, 1),
                (TerrainVisibleType::Same, TileSymmetry::Full, TileComposition::Main, TileType::Solid),
            ),
        ];

        let make_common_ground_table = |is_old| {
            vec![
                (
                    vec![[  Other(Dirty),   EqOr(Dirty),    Other(Dirty),
                            EqOr(Dirty),                    Eq,
                            Other(Dirty),   Eq,             Eq,
                        ],
                        [   EqOr(Dirty),    Other(Dirty),   Diff(Dirty),
                            EqOr(Dirty),                    Eq,
                            Other(Dirty),   Eq,             Eq,
                        ],
                        [   EqOr(Dirty),    Other(Dirty),   Diff(Dirty),
                            Other(Dirty),                   Eq,
                            Diff(Dirty),    Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(0..=3) } else { TileCodesSet::new(0..=5) },
                    (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  EqOr(Dirty),    Eq,             Eq,
                            Other(Dirty),                   Eq,
                            EqOr(Dirty),    Eq,             Eq,
                        ],
                        [   Other(Dirty),   Eq,             Eq,
                            EqOr(Dirty),                    Eq,
                            Other(Dirty),   Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(4..=7) } else { TileCodesSet::new(6..=13) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, VERTICAL_HALF_DIRT),
                ),
                (
                    vec![[  EqOr(Dirty),    Other(Dirty),   EqOr(Dirty),
                            Eq,                             Eq,
                            Eq,             Eq,             Eq,
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    Other(Dirty),
                            Eq,                             Eq,
                            Eq,             Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(8..=11) } else { TileCodesSet::new(14..=21) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, HORIZONTAL_HALF_DIRT),
                ),
                (
                    vec![[  Eq,             Eq,                                 Eq,
                            Eq,                                                 SameTyped(HORIZONTAL_HALF_DIRT_ARR),
                            Eq,             SameTyped(VERTICAL_HALF_DIRT_ARR),  Other(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::new(12..=15) } else { TileCodesSet::new(22..=27) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  EqOr(Dirty),    Other(Dirty),   Eq,
                            Other(Dirty),                   Eq,
                            EqOr(Dirty),    Eq,             Eq
                        ],
                        [   EqOr(Dirty),    Other(Dirty),   Eq,
                            EqOr(Dirty),                    Eq,
                            Other(Dirty),   Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(16..=17) } else { TileCodesSet::new(28..=30) },
                    (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Dirt)),
                ),
                (
                    vec![[  Eq,             Eq,             Eq,
                            Eq,                             Eq,
                            Eq,             Eq,             Other(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::new(18..=19) } else { TileCodesSet::new(31..=33) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            Other(Sandy),   DiffAny,
                            Other(Sandy),                   Eq,
                            DiffAny,        Eq,             Eq,
                        ],
                        [   Any,            Other(Sandy),   DiffAny,
                            Any,                            Eq,
                            Other(Sandy),   Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(20..=23) } else { TileCodesSet::new(34..=39) },
                    (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            Eq,             Eq,
                            Other(Sandy),                   Eq,
                            Any,            Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(24..=27) } else { TileCodesSet::new(40..=47) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, VERTICAL_HALF_SAND),
                ),
                (
                    vec![[  Any,            Other(Sandy),   Any,
                            Eq,                             Eq,
                            Eq,             Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(28..=31) } else { TileCodesSet::new(48..=55) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, HORIZONTAL_HALF_SAND),
                ),
                (
                    vec![[  Eq,             Eq,                                 Eq,
                            Eq,                                                 SameTyped(HORIZONTAL_HALF_SAND_ARR),
                            Eq,             SameTyped(VERTICAL_HALF_SAND_ARR),  Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::new(32..=35) } else { TileCodesSet::new(56..=61) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            Other(Sandy),   Eq,
                            Other(Sandy),                   Eq,
                            Any,            Eq,             Eq
                        ],
                        [   Any,            Other(Sandy),   Eq,
                            Any,                            Eq,
                            Other(Sandy),   Eq,             Eq,
                        ],
                    ],
                    if is_old { TileCodesSet::new(36..=37) } else { TileCodesSet::new(62..=64) },
                    (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Sand)),
                ),
                (
                    vec![[  Eq,             Eq,             Eq,
                            Eq,                             Eq,
                            Eq,             Eq,             Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::new(38..=39) } else { TileCodesSet::new(65..=67) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Other(Dirty),   Eq,             Eq,
                            Eq,                             Eq,
                            Eq,             Eq,             Other(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(40) } else { TileCodesSet::from_code(68) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Other(Dirty),   Eq,             Eq,
                            Eq,                             Eq,
                            Eq,             Eq,             Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(41) } else { TileCodesSet::from_code(69) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Other(Sandy),   Eq,             Eq,
                            Eq,                             Eq,
                            Eq,             Eq,             Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(42) } else { TileCodesSet::from_code(70) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Eq,             Eq,             EqOr(Dirty),
                            Eq,                             Other(Dirty),
                            Other(Sandy),   EqOr(Dirty),    EqOr(Dirty),
                        ],
                        [   Eq,             Eq,             Other(Dirty),
                            Eq,                             EqOr(Dirty),
                            Other(Sandy),   Other(Dirty),   EqOr(Dirty),
                        ],
                        [   Eq,             Eq,             Other(Dirty),
                            Eq,                             EqOr(Dirty),
                            Other(Sandy),   EqOr(Dirty),    Other(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(43) } else { TileCodesSet::from_code(71) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Eq,             Eq,             Other(Sandy),
                            Eq,                             EqOr(Dirty),
                            EqOr(Dirty),    Other(Dirty),   EqOr(Dirty),
                        ],
                        [   Eq,             Eq,             Other(Sandy),
                            Eq,                             Other(Dirty),
                            Other(Dirty),   EqOr(Dirty),    EqOr(Dirty),
                        ],
                        [   Eq,             Eq,             Other(Sandy),
                            Eq,                             EqOr(Dirty),
                            Other(Dirty),   EqOr(Dirty),    Other(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(44) } else { TileCodesSet::from_code(72) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Eq,             Eq,             EqOr(Dirty),
                            Eq,                             Other(Dirty),
                            Eq,             Eq,             Other(Sandy),
                        ],
                        [   Eq,             Eq,             Other(Dirty),
                            Eq,                             EqOr(Dirty),
                            Eq,             Eq,             Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(45) } else { TileCodesSet::from_code(73) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, VERTICAL_HALF_DIRT_SAND),
                ),
                (
                    vec![[  Eq,             Eq,             Eq,
                            Eq,                             Eq,
                            EqOr(Dirty),    Other(Dirty),   Other(Sandy),
                        ],
                        [   Eq,             Eq,             Eq,
                            Eq,                             Eq,
                            Other(Dirty),   EqOr(Dirty),    Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(46) } else { TileCodesSet::from_code(74) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, HORIZONTAL_HALF_DIRT_SAND),
                ),
                (
                    vec![[  Eq,             Eq,             Any,
                            Eq,                             Other(Sandy),
                            EqOr(Dirty),    Other(Dirty),   Any,
                        ],
                        [   Eq,             Eq,             Any,
                            Eq,                             Other(Sandy),
                            Other(Dirty),   EqOr(Dirty),    Any,
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(47) } else { TileCodesSet::from_code(75) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Eq,             Eq,             Other(Dirty),
                            Eq,                             EqOr(Dirty),
                            Any,            Other(Sandy),   Any,
                        ],
                        [   Eq,             Eq,             EqOr(Dirty),
                            Eq,                             Other(Dirty),
                            Any,            Other(Sandy),   Any,
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(48) } else { TileCodesSet::from_code(76) },
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    if is_old {
                        TileCodesSet::with_frequency(49..=56, 5).add_codes(57..=72, 1)
                    } else {
                        TileCodesSet::with_frequency(77..=101, 5).add_codes(102..=117, 1)
                    },
                    (TerrainVisibleType::Same, TileSymmetry::Full, TileComposition::Main, TileType::Solid),
                ),
                (
                    vec![[  EqOr(Dirty),    Other(Dirty),   EqOr(Dirty),
                            Other(Dirty),                   EqOr(Dirty),
                            EqOr(Dirty),    EqOr(Dirty),    Other(Sandy),
                        ],
                        [   EqOr(Dirty),    Other(Dirty),   EqOr(Dirty),
                            EqOr(Dirty),                    EqOr(Dirty),
                            Other(Dirty),   EqOr(Dirty),    Other(Sandy),
                        ],
                        [   EqOr(Dirty),    Other(Dirty),   EqOr(Dirty),
                            EqOr(Dirty),                    EqOr(Dirty),
                            EqOr(Dirty),    Other(Dirty),   Other(Sandy),
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    EqOr(Dirty),
                            EqOr(Dirty),                    Other(Dirty),
                            EqOr(Dirty),    Other(Dirty),   Other(Sandy),
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    Other(Dirty),
                            EqOr(Dirty),                    EqOr(Dirty),
                            Other(Dirty),   EqOr(Dirty),    Other(Sandy),
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    EqOr(Dirty),
                            EqOr(Dirty),                    Other(Dirty),
                            Other(Dirty),   EqOr(Dirty),    Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(75) } else { TileCodesSet::from_code(120) },
                    (TerrainVisibleType::DiffMixed(Terrain::Dirt), TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            Other(Sandy),   Any,
                            Other(Sandy),                   EqOr(Dirty),
                            Any,            EqOr(Dirty),    Other(Dirty),
                        ],
                        [   Any,            Other(Sandy),   Any,
                            Other(Sandy),                   Other(Dirty),
                            Any,            EqOr(Dirty),    EqOr(Dirty),
                        ],

                        [   Any,            Other(Sandy),   Any,
                            Any,                            EqOr(Dirty),
                            Other(Sandy),   EqOr(Dirty),    Other(Dirty),
                        ],
                        [   Any,            Other(Sandy),   Any,
                            Any,                            EqOr(Dirty),
                            Other(Sandy),   Other(Dirty),   EqOr(Dirty),
                        ],
                        [   Any,            Other(Sandy),   Any,
                            Any,                            Other(Dirty),
                            Other(Sandy),   EqOr(Dirty),    EqOr(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(76) } else { TileCodesSet::from_code(121) },
                    (TerrainVisibleType::DiffMixed(Terrain::Sand), TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Eq,             Eq,             Other(Sandy),
                            Eq,                             EqOr(Dirty),
                            Other(Sandy),   Other(Dirty),   EqOr(Dirty),
                        ],
                        [   Eq,             Eq,             Other(Sandy),
                            Eq,                             EqOr(Dirty),
                            Other(Sandy),   EqOr(Dirty),    Other(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(77) } else { TileCodesSet::from_code(122) },
                    (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Eq,             Eq,             EqOr(Dirty),
                            Eq,                             Other(Dirty),
                            EqOr(Dirty),    Other(Dirty),   Other(Sandy),
                        ],
                        [   Eq,             Eq,             Other(Dirty),
                            Eq,                             EqOr(Dirty),
                            EqOr(Dirty),    Other(Dirty),   Other(Sandy),
                        ],
                        [   Eq,             Eq,             Other(Dirty),
                            Eq,                             EqOr(Dirty),
                            Other(Dirty),   EqOr(Dirty),    Other(Sandy),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(78) } else { TileCodesSet::from_code(123) },
                    (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
                ),

                (
                    vec![[  EqOr(Dirty),    EqOr(Dirty),    EqOr(Dirty),
                            Other(Dirty),                   Other(Dirty),
                            EqOr(Dirty),    EqOr(Dirty),    EqOr(Dirty),
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    EqOr(Dirty),
                            EqOr(Dirty),                    Other(Dirty),
                            EqOr(Dirty),    Other(Dirty),   EqOr(Dirty),
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    Other(Dirty),
                            EqOr(Dirty),                    EqOr(Dirty),
                            EqOr(Dirty),    Other(Dirty),   EqOr(Dirty),
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    Other(Dirty),
                            EqOr(Dirty),                    EqOr(Dirty),
                            Other(Dirty),   EqOr(Dirty),    Other(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(73) } else { TileCodesSet::from_code(118) },
                    (TerrainVisibleType::Diff(Terrain::Dirt), TileSymmetry::Full, TileComposition::Main, TileType::Solid),
                ),
                (
                    vec![[  Any,            Any,            Any,
                            Other(Sandy),                   Other(Sandy),
                            Any,            Any,            Any,
                        ],
                        [   Other(Sandy),   Any,            Any,
                            Any,                            Other(Sandy),
                            Any,            Other(Sandy),   Any,
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(74) } else { TileCodesSet::from_code(119) },
                    (TerrainVisibleType::Diff(Terrain::Sand), TileSymmetry::Full, TileComposition::Main, TileType::Solid),
                ),

                // fallback
                (
                    vec![[  Any,            EqOr(Dirty),    EqOr(Dirty),
                            Other(Sandy),                   Other(Dirty),
                            Any,            EqOr(Dirty),    EqOr(Dirty),
                        ],
                        [   Any,            EqOr(Dirty),    Other(Dirty),
                            Other(Sandy),                   EqOr(Dirty),
                            Any,            EqOr(Dirty),    Other(Dirty),
                        ],
                        [   Any,            Other(Dirty),   EqOr(Dirty),
                            Other(Sandy),                   EqOr(Dirty),
                            Any,            EqOr(Dirty),    Other(Dirty),
                        ],
                        [   Any,            Other(Dirty),   EqOr(Dirty),
                            Other(Sandy),                   EqOr(Dirty),
                            Any,            Other(Dirty),   EqOr(Dirty),
                        ],

                        [   Other(Dirty),   EqOr(Dirty),    Other(Sandy),
                            EqOr(Dirty),                    EqOr(Dirty),
                            Other(Sandy),   EqOr(Dirty),    Other(Dirty),
                        ],
                        [   Other(Dirty),   EqOr(Dirty),    Other(Sandy),
                            EqOr(Dirty),                    Other(Dirty),
                            Other(Sandy),   EqOr(Dirty),    EqOr(Dirty),
                        ],
                        [   EqOr(Dirty),    EqOr(Dirty),    Other(Sandy),
                            Other(Dirty),                   EqOr(Dirty),
                            Other(Sandy),   EqOr(Dirty),    Other(Dirty),
                        ],
                        [   EqOr(Dirty),    Other(Dirty),   Other(Sandy),
                            EqOr(Dirty),                    Other(Dirty),
                            Other(Sandy),   EqOr(Dirty),    EqOr(Dirty),
                        ],
                        [   EqOr(Dirty),    Other(Dirty),   Other(Sandy),
                            EqOr(Dirty),                    EqOr(Dirty),
                            Other(Sandy),   Other(Dirty),   EqOr(Dirty),
                        ],
                    ],
                    if is_old { TileCodesSet::from_code(74) } else { TileCodesSet::from_code(119) },
                    (TerrainVisibleType::Diff(Terrain::Sand), TileSymmetry::Full, TileComposition::Fallback, TileType::Solid),
                ),
            ]
        };

        let old_common_ground_table = make_common_ground_table(true);
        let new_common_ground_table = make_common_ground_table(false);

        let main_water_table = vec![
            (
                vec![[  Any,            OtherAny,   DiffAny,
                        Any,                        Eq,
                        OtherAny,        Eq,        Eq,
                    ],
                    [   Any,            OtherAny,   DiffAny,
                        OtherAny,                   Eq,
                        DiffAny,        Eq,         Eq,
                    ],
                ],
                TileCodesSet::new(0..=3),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Any,            Eq,         Eq,
                        OtherAny,                   Eq,
                        Any,            Eq,         Eq,
                    ],
                ],
                TileCodesSet::new(4..=7),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, VERTICAL_HALF_SAND),
            ),
            (
                vec![[  Any,            OtherAny,   Any,
                        Eq,                         Eq,
                        Eq,             Eq,         Eq,
                    ],
                ],
                TileCodesSet::new(8..=11),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, HORIZONTAL_HALF_SAND),
            ),
            (
                vec![[  Eq,             Eq,                                 Eq,
                        Eq,                                                 SameTyped(HORIZONTAL_HALF_SAND_ARR),
                        Eq,             SameTyped(VERTICAL_HALF_SAND_ARR),  OtherAny,
                    ],
                ],
                TileCodesSet::new(12..=15),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Any,            OtherAny,   Any,
                        OtherAny,                   Eq,
                        Same,           Eq,         Eq,
                    ],
                    [   Any,            Any,        OtherAny,
                        OtherAny,                   Eq,
                        Same,           Eq,         Eq,
                    ],
                ],
                TileCodesSet::new(16..=17),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Sand)),
            ),
            (
                vec![[  Eq,             Eq,         Eq,
                        Eq,                         Eq,
                        Eq,             Eq,         OtherAny,
                    ],
                ],
                TileCodesSet::new(18..=19),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[Eq; NEIGHBORHOOD_SIZE]],
                TileCodesSet::new(21..=32),
                (TerrainVisibleType::Same, TileSymmetry::Full, TileComposition::Main, TileType::Solid),
            ),
        ];

        let water_table = if !one_tile_water {
            main_water_table
        } else {
            let mut water_table = vec![
                (
                    vec![[  Any,            OtherAny,        Any,
                            OtherAny,                        OtherAny,
                            Any,            OtherAny,        Any,
                        ],
                    ],
                    TileCodesSet::new(33..=36),
                    (TerrainVisibleType::Mixed, TileSymmetry::Full, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            OtherAny,       Any,
                            OtherAny,                       OtherAny,
                            Any,            Eq,             Any,
                        ],
                    ],
                    TileCodesSet::new(37..=40),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            Eq,             Any,
                            OtherAny,                       OtherAny,
                            Any,            Eq,             Any,
                        ],
                    ],
                    TileCodesSet::new(41..=44),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            OtherAny,       Any,
                            Eq,                             Eq,
                            Any,            OtherAny,       Any,
                        ],
                    ],
                    TileCodesSet::new(45..=48),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            OtherAny,       Any,
                            OtherAny,                       Eq,
                            Any,            Eq,             OtherAny,
                        ],
                    ],
                    TileCodesSet::new(49..=52),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  OtherAny,       Eq,             Eq,
                            Eq,                             Eq,
                            OtherAny,       Eq,             Eq,
                        ],
                    ],
                    TileCodesSet::new(53..=54),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  OtherAny,       Eq,             OtherAny,
                            Eq,                             Eq,
                            Eq,             Eq,             Eq,
                        ],
                    ],
                    TileCodesSet::new(55..=56),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  OtherAny,       Eq,             Eq,
                            Eq,                             Eq,
                            Eq,             Eq,             OtherAny,
                        ],
                    ],
                    TileCodesSet::new(57..=58),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  OtherAny,       Eq,             OtherAny,
                            Eq,                             Eq,
                            OtherAny,       Eq,             Eq,
                        ],
                    ],
                    TileCodesSet::new(59..=62),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            Eq,             OtherAny,
                            OtherAny,                       Eq,
                            Any,            Eq,             Eq,
                        ],
                    ],
                    TileCodesSet::new(63..=66),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            OtherAny,       Any,
                            Eq,                             Eq,
                            OtherAny,       Eq,             Eq,
                        ],
                    ],
                    TileCodesSet::new(67..=70),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            Eq,             OtherAny,
                            OtherAny,                       Eq,
                            Any,            Eq,             OtherAny,
                        ],
                    ],
                    TileCodesSet::new(71..=72),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            OtherAny,       Any,
                            Eq,                             Eq,
                            OtherAny,       Eq,             OtherAny,
                        ],
                    ],
                    TileCodesSet::new(73..=74),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  OtherAny,       Eq,             OtherAny,
                            Eq,                             Eq,
                            OtherAny,       Eq,             OtherAny,
                        ],
                    ],
                    TileCodesSet::from_code(75),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
                (
                    vec![[  Any,            OtherAny,       Any,
                            OtherAny,                       Eq,
                            Any,            OtherAny,       Any,
                        ],
                    ],
                    TileCodesSet::new(76..=79),
                    (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
                ),
            ];
            water_table.extend(main_water_table);
            water_table
        };

        let rock_table = vec![
            (
                vec![[Eq; NEIGHBORHOOD_SIZE]],
                TileCodesSet::new(0..=7),
                (TerrainVisibleType::Same, TileSymmetry::Full, TileComposition::Main, TileType::Solid),
            ),
            (
                vec![[  Any,            OtherAny,       DiffAny,
                        OtherAny,                       Eq,
                        DiffAny,        Eq,             Eq,
                    ],
                    [   Any,            OtherAny,       DiffAny,
                        Any,                            Eq,
                        OtherAny,       Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(8..=9),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  DiffAny,        OtherAny,       Any,
                        Eq,                             OtherAny,
                        Eq,             Eq,             DiffAny,
                    ],
                    [   DiffAny,        OtherAny,       Any,
                        Eq,                             Any,
                        Eq,             Eq,             OtherAny,
                    ],
                ],
                TileCodesSet::new(10..=11),
                (TerrainVisibleType::Mixed, TileSymmetry::SideDiagonal, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  DiffAny,        Eq,             Eq,
                        OtherAny,                       Eq,
                        Any,            OtherAny,       DiffAny,
                    ],
                    [   OtherAny,       Eq,             Eq,
                        Any,                            Eq,
                        Any,            OtherAny,       DiffAny,
                ],
                ],
                TileCodesSet::new(12..=13),
                (TerrainVisibleType::Mixed, TileSymmetry::SideDiagonal, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Eq,             Eq,             DiffAny,
                        Eq,                             OtherAny,
                        DiffAny,        OtherAny,       Any,
                    ],
                    [   Eq,             Eq,             OtherAny,
                        Eq,                             Any,
                        DiffAny,        OtherAny,       Any,
                    ],
                ],
                TileCodesSet::new(14..=15),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Any,            Eq,             Eq,
                        OtherAny,                       Eq,
                        Any,            Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(16..=17),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, VERTICAL_HALF_SAND),
            ),
            (
                vec![[  Eq,             Eq,             Any,
                        Eq,                             OtherAny,
                        Eq,             Eq,             Any,
                    ],
                ],
                TileCodesSet::new(18..=19),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, VERTICAL_HALF_SAND),
            ),
            (
                vec![[  Any,            OtherAny,       Any,
                        Eq,                             Eq,
                        Eq,             Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(20..=21),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, HORIZONTAL_HALF_SAND),
            ),
            (
                vec![[  Eq,             Eq,             Eq,
                        Eq,                             Eq,
                        Any,            OtherAny,       Any,
                    ],
                ],
                TileCodesSet::new(22..=23),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, HORIZONTAL_HALF_SAND),
            ),
            (
                vec![[  Eq,             Eq,                                     Eq,
                        Eq,                                                     SameTyped(HORIZONTAL_HALF_SAND_ARR),
                        Eq,             SameTyped(VERTICAL_HALF_SAND_ARR),      OtherAny,
                    ],
                ],
                TileCodesSet::new(24..=25),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Eq,                                     Eq,                                 Eq,
                        SameTyped(HORIZONTAL_HALF_SAND_ARR),                                        Eq,
                        OtherAny,                               SameTyped(VERTICAL_HALF_SAND_ARR),  Eq,
                    ],
                ],
                TileCodesSet::new(26..=27),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Eq,         SameTyped(VERTICAL_HALF_SAND_ARR),  OtherAny,
                        Eq,                                             SameTyped(HORIZONTAL_HALF_SAND_ARR),
                        Eq,         Eq,                                 Eq,
                    ],
                ],
                TileCodesSet::new(28..=29),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  OtherAny,                               SameTyped(VERTICAL_HALF_SAND_ARR),  Eq,
                        SameTyped(HORIZONTAL_HALF_SAND_ARR),                                        Eq,
                        Eq,                                     Eq,                                 Eq,
                    ],
                ],
                TileCodesSet::new(30..=31),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Any,            OtherAny,       Any,
                        OtherAny,                       Eq,
                        Same,           Eq,             Eq,
                    ],
                    [   Any,            Any,            OtherAny,
                        OtherAny,                       Eq,
                        Same,           Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(32..=33),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Sand)),
            ),
            (
                vec![[  Any,            OtherAny,       Any,
                        Eq,                             OtherAny,
                        Eq,             Eq,             Same,
                    ],
                    [   OtherAny,       Any,            Any,
                        Eq,                             OtherAny,
                        Eq,             Eq,             Same,
                    ],
                ],
                TileCodesSet::new(34..=35),
                (TerrainVisibleType::Mixed, TileSymmetry::SideDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Sand)),
            ),
            (
                vec![[  Any,            Eq,             Eq,
                        OtherAny,                       Eq,
                        Any,            OtherAny,       Same,
                    ],
                    [   OtherAny,       Eq,             Eq,
                        Any,                            Eq,
                        Any,            OtherAny,       Same,
                    ],
                ],
                TileCodesSet::new(36..=37),
                (TerrainVisibleType::Mixed, TileSymmetry::SideDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Sand)),
            ),
            (
                vec![[  Eq,             Eq,             Any,
                        Eq,                             OtherAny,
                        Same,           OtherAny,       Any,
                    ],
                    [   Eq,             Eq,             OtherAny,
                        Eq,                             Any,
                        Same,           OtherAny,       Any,
                    ],
                ],
                TileCodesSet::new(38..=39),
                (TerrainVisibleType::Mixed, TileSymmetry::MainDiagonal, TileComposition::Main, TileType::WideObliqueAngle(Terrain::Sand)),
            ),
            (
                vec![[  Eq,             Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             OtherAny,
                    ],
                ],
                TileCodesSet::new(40..=41),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Eq,             Eq,             Eq,
                        Eq,                             Eq,
                        OtherAny,       Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(42..=43),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  Eq,             Eq,             OtherAny,
                        Eq,                             Eq,
                        Eq,             Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(44..=45),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
            (
                vec![[  OtherAny,       Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(46..=47),
                (TerrainVisibleType::Mixed, TileSymmetry::None, TileComposition::Main, TileType::Undefined),
            ),
        ];

        let result_table = [
            (
                Terrain::Dirt,
                &dirt_table,
            ),
            (
                Terrain::Sand,
                &sand_table,
            ),
            (
                Terrain::Grass,
                &old_common_ground_table,
            ),
            (
                Terrain::Snow,
                &old_common_ground_table,
            ),
            (
                Terrain::Swamp,
                &old_common_ground_table,
            ),
            (
                Terrain::Rough,
                &old_common_ground_table,
            ),
            (
                Terrain::Subterranean,
                &old_common_ground_table,
            ),
            (
                Terrain::Lava,
                &old_common_ground_table,
            ),
            (
                Terrain::Highlands,
                &new_common_ground_table,
            ),
            (
                Terrain::Wasteland,
                &new_common_ground_table,
            ),
            (
                Terrain::Water,
                &water_table,
            ),
            (
                Terrain::Rock,
                &rock_table,
            ),
        ];

        TilesTable {
            inner: result_table
                .into_iter()
                .map(|element| {
                    (
                        element.0,
                        element
                            .1
                            .iter()
                            .enumerate()
                            .map(|( group_number,
                                    (   patterns,
                                        codes,
                                        (   terrain_visible_type,
                                            tile_symmetry,
                                            composition,
                                            tile_type
                                        )
                                    )
                                )|
                            {
                                TilesGroupInfo::new(
                                    patterns,
                                    codes,
                                    *composition,
                                    *tile_type,
                                    *terrain_visible_type,
                                    *tile_symmetry,
                                    group_number)
                            })
                            .collect::<Vec<TilesGroupInfo>>(),
                    )
                })
                .collect(),
        }
    }

    pub fn terrain_tile_groups(&self, terrain: Terrain) -> &[TilesGroupInfo] {
        &self.inner[&terrain]
    }
}
