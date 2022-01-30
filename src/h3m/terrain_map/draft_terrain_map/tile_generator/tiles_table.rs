use super::terrain_relation::{
    NeighborhoodPattern, TerrainCategory, TerrainRelation, NEIGHBORHOOD_SIZE,
};
use super::TileCodesSet;
use crate::h3m::terrain_map::tile::TerrainVisibleType;
use crate::h3m::Terrain;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum TileSymmetry {
    MainDiagonal,
    None,
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

fn additional_patterns(
    pattern: &NeighborhoodPattern,
    tile_symmetry: TileSymmetry,
) -> Vec<NeighborhoodPattern> {
    let mut additional_patterns = Vec::new();
    match tile_symmetry {
        TileSymmetry::MainDiagonal => {
            additional_patterns.push(rotate_pattern_with_main_diagonal_symmetry(pattern))
        }
        TileSymmetry::None => (),
    }
    additional_patterns
}

fn expand_patterns(
    patterns: Vec<NeighborhoodPattern>,
    tile_symmetry: TileSymmetry,
) -> Vec<NeighborhoodPattern> {
    let mut expanded_patterns = Vec::with_capacity(patterns.len());
    for pattern in patterns {
        for additional_pattern in additional_patterns(&pattern, tile_symmetry) {
            if additional_pattern != pattern {
                expanded_patterns.push(additional_pattern);
            }
        }
        expanded_patterns.push(pattern);
    }
    expanded_patterns
}

pub struct TilesGroupInfo {
    patterns: Vec<NeighborhoodPattern>,
    codes: TileCodesSet,
    name: &'static str,
    terrain_visible_type: TerrainVisibleType,
}

impl TilesGroupInfo {
    fn new(
        patterns: Vec<NeighborhoodPattern>,
        codes: TileCodesSet,
        name: &'static str,
        terrain_visible_type: TerrainVisibleType,
        tile_symmetry: TileSymmetry,
    ) -> TilesGroupInfo {
        TilesGroupInfo {
            patterns: expand_patterns(patterns, tile_symmetry),
            codes,
            name,
            terrain_visible_type,
        }
    }

    pub fn patterns(&self) -> &[NeighborhoodPattern] {
        &self.patterns
    }

    pub fn codes(&self) -> &TileCodesSet {
        &self.codes
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn terrain_visible_type(&self) -> &TerrainVisibleType {
        &self.terrain_visible_type
    }
}

pub struct TilesTable {
    inner: HashMap<Terrain, Vec<TilesGroupInfo>>,
}

impl TilesTable {
    #[rustfmt::skip]
    pub fn new() -> TilesTable {
        use TerrainCategory::*;
        use TerrainRelation::*;

        let table = [
            (
                Terrain::Dirt,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(21..=28, 4).add_codes(29..=44, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Sand,
                vec![(
                    vec![[Any; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(0..=7, 4).add_codes(8..=23, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Grass,
                vec![
                    (
                        vec![[  Any,            Diff(Sandy),    DiffAny,
                                Diff(Sandy),                    Eq,
                                DiffAny,        Eq,             Eq
                            ],
                            [   Any,            Diff(Sandy),    DiffAny,
                                Any,                            Eq,
                                Diff(Sandy),    Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(20..=23),
                        (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
                    ),
                    (
                        vec![[  Any,            Eq,             Eq,
                                Diff(Sandy),                    Eq,
                                Any,            Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(24..=27),
                        (TerrainVisibleType::None, TileSymmetry::None, ""),
                    ),
                    (
                        vec![[  Any,            Diff(Sandy),    Any,
                                Eq,                             Eq,
                                Eq,             Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(28..=31),
                        (TerrainVisibleType::None, TileSymmetry::None, ""),
                    ),
                    (
                        vec![[  Eq,             Eq,             Eq,
                                Eq,                             Eq,
                                Eq,             Eq,             Diff(Sandy)
                            ],
                        ],
                        TileCodesSet::new(32..=35).add_codes(38..=39, 5),
                        (TerrainVisibleType::None, TileSymmetry::None, ""),
                    ),
                    (
                        vec![[  Any,            Diff(Sandy),    Eq,
                                Diff(Sandy),                    Eq,
                                Any,            Eq,             Eq
                            ],
                            [   Any,            Diff(Sandy),    Eq,
                                Any,                            Eq,
                                Diff(Sandy),    Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(36..=37),
                        (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
                    ),
                    (
                        vec![[  Diff(Sandy),    Eq,             Eq,
                                Eq,                             Eq,
                                Eq,             Eq,             Diff(Sandy)
                            ],
                        ],
                        TileCodesSet::from_code(42),
                        (TerrainVisibleType::None, TileSymmetry::None, ""),
                    ),
                    (
                        vec![[Eq; NEIGHBORHOOD_SIZE]],
                        TileCodesSet::with_frequency(49..=56, 5).add_codes(57..=72, 1),
                        (TerrainVisibleType::Same, TileSymmetry::None, ""),
                    ),
                    (
                        vec![[  Any,            Any,            Any,
                                Diff(Sandy),                    Diff(Sandy),
                                Any,            Any,            Any
                            ],
                        ],
                        TileCodesSet::from_code(74),
                        (TerrainVisibleType::Diff(Terrain::Sand), TileSymmetry::MainDiagonal, ""),
                    ),
                ],
            ),
            (
                Terrain::Snow,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Swamp,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Rough,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Subterranean,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Lava,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Highland,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Wasteland,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Water,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::new(21..=32),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
            (
                Terrain::Rock,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::new(0..=7),
                    (TerrainVisibleType::Same, TileSymmetry::None, ""),
                )],
            ),
        ];

        TilesTable {
            inner: table
                .into_iter()
                .map(|element| {
                    (
                        element.0,
                        element
                            .1
                            .into_iter()
                            .map(|( patterns,
                                    codes,
                                    (terrain_visible_type, tile_symmetry, name))| {
                                TilesGroupInfo::new(patterns, codes, name, terrain_visible_type, tile_symmetry)
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
