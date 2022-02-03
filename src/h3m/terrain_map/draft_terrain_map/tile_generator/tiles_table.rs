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
    patterns: &[NeighborhoodPattern],
    tile_symmetry: TileSymmetry,
) -> Vec<NeighborhoodPattern> {
    let mut expanded_patterns = Vec::with_capacity(patterns.len());
    for &pattern in patterns {
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
        patterns: &[NeighborhoodPattern],
        codes: &TileCodesSet,
        name: &'static str,
        terrain_visible_type: TerrainVisibleType,
        tile_symmetry: TileSymmetry,
    ) -> TilesGroupInfo {
        TilesGroupInfo {
            patterns: expand_patterns(patterns, tile_symmetry),
            codes: codes.clone(),
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

        let dirt_table = vec![
            (
                vec![[  Any,            Diff(Sandy),    DiffAny,
                        Diff(Sandy),                    Eq,
                        DiffAny,        Eq,             Eq,
                    ],
                    [   Any,            Diff(Sandy),    DiffAny,
                        Any,                            Eq,
                        Diff(Sandy),    Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(0..=3),
                (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
            ),
            (
                vec![[  Any,            Eq,             Eq,
                        Diff(Sandy),                    Eq,
                        Any,            Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(4..=7),
                (TerrainVisibleType::None, TileSymmetry::None, "vertical_half_sand"),
            ),
            (
                vec![[  Any,            Diff(Sandy),    Any,
                        Eq,                             Eq,
                        Eq,             Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(8..=11),
                (TerrainVisibleType::None, TileSymmetry::None, "horizontal_half_sand"),
            ),
            (
                vec![[  Eq,             Eq,                                 Eq,
                        Eq,                                                 SameNamed("horizontal_half_sand"),
                        Eq,             SameNamed("vertical_half_sand"),    Diff(Sandy),
                    ],
                ],
                TileCodesSet::new(12..=15),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Any,            Diff(Sandy),    Eq,
                        Diff(Sandy),                    Eq,
                        Any,            Eq,             Eq
                    ],
                    [   Any,            Diff(Sandy),    Eq,
                        Any,                            Eq,
                        Diff(Sandy),    Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(16..=17),
                (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
            ),
            (
                vec![[  Eq,             Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Diff(Sandy),
                    ],
                ],
                TileCodesSet::new(18..=19),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Diff(Sandy),    Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Diff(Sandy),
                    ],
                ],
                TileCodesSet::from_code(20),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[AnyExcept(Sandy); NEIGHBORHOOD_SIZE]],
                TileCodesSet::with_frequency(21..=28, 4).add_codes(29..=44, 1),
                (TerrainVisibleType::Same, TileSymmetry::None, ""),
            ),
            (
                vec![[  Any,            Any,            Any,
                        Diff(Sandy),                    Diff(Sandy),
                        Any,            Any,            Any,
                    ],
                    [   Diff(Sandy),    Any,            Any,
                        Any,                            Diff(Sandy),
                        Any,            Diff(Sandy),    Any,
                    ],
                ],
                TileCodesSet::from_code(45),
                (TerrainVisibleType::Diff(Terrain::Sand), TileSymmetry::MainDiagonal, ""),
            ),
        ];

        let sand_table = vec![
            (
                vec![[Any; NEIGHBORHOOD_SIZE]],
                TileCodesSet::with_frequency(0..=7, 4).add_codes(8..=23, 1),
                (TerrainVisibleType::Same, TileSymmetry::None, ""),
            ),
        ];

        let old_common_ground_table = vec![
            (
                vec![[  AnyExcept(Sandy),   Diff(Dirty),    Diff(Dirty),
                        AnyExcept(Sandy),                   Eq,
                        Diff(Dirty),        Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(0..=3),
                (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
            ),
            (
                vec![[  AnyExcept(Sandy),   Eq,             Eq,
                        Diff(Dirty),                        Eq,
                        AnyExcept(Sandy),   Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(4..=7),
                (TerrainVisibleType::None, TileSymmetry::None, "vertical_half_dirt"),
            ),
            (
                vec![[  AnyExcept(Sandy),   Diff(Dirty),    AnyExcept(Sandy),
                        Eq,                                 Eq,
                        Eq,                 Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(8..=11),
                (TerrainVisibleType::None, TileSymmetry::None, "horizontal_half_dirt"),
            ),
            (
                vec![[  Eq,                 Eq,                                 Eq,
                        Eq,                                                     SameNamed("horizontal_half_dirt"),
                        Eq,                 SameNamed("vertical_half_dirt"),    Diff(Dirty),
                    ],
                ],
                TileCodesSet::new(12..=15),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  AnyExcept(Sandy),   Diff(Dirty),    Eq,
                        Diff(Dirty),                        Eq,
                        AnyExcept(Sandy),   Eq,             Eq
                    ],
                    [   AnyExcept(Sandy),   Diff(Dirty),    Eq,
                        AnyExcept(Sandy),                   Eq,
                        Diff(Dirty),        Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(16..=17),
                (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
            ),
            (
                vec![[  Eq,             Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Diff(Dirty),
                    ],
                ],
                TileCodesSet::new(18..=19),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Any,            Diff(Sandy),    DiffAny,
                        Diff(Sandy),                    Eq,
                        DiffAny,        Eq,             Eq,
                    ],
                    [   Any,            Diff(Sandy),    DiffAny,
                        Any,                            Eq,
                        Diff(Sandy),    Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(20..=23),
                (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
            ),
            (
                vec![[  Any,            Eq,             Eq,
                        Diff(Sandy),                    Eq,
                        Any,            Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(24..=27),
                (TerrainVisibleType::None, TileSymmetry::None, "vertical_half_sand"),
            ),
            (
                vec![[  Any,            Diff(Sandy),    Any,
                        Eq,                             Eq,
                        Eq,             Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(28..=31),
                (TerrainVisibleType::None, TileSymmetry::None, "horizontal_half_sand"),
            ),
            (
                vec![[  Eq,             Eq,                                 Eq,
                        Eq,                                                 SameNamed("horizontal_half_sand"),
                        Eq,             SameNamed("vertical_half_sand"),    Diff(Sandy),
                    ],
                ],
                TileCodesSet::new(32..=35),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Any,            Diff(Sandy),    Eq,
                        Diff(Sandy),                    Eq,
                        Any,            Eq,             Eq
                    ],
                    [   Any,            Diff(Sandy),    Eq,
                        Any,                            Eq,
                        Diff(Sandy),    Eq,             Eq,
                    ],
                ],
                TileCodesSet::new(36..=37),
                (TerrainVisibleType::None, TileSymmetry::MainDiagonal, ""),
            ),
            (
                vec![[  Eq,             Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Diff(Sandy),
                    ],
                ],
                TileCodesSet::new(38..=39),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Diff(Dirty),    Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Diff(Dirty),
                    ],
                ],
                TileCodesSet::from_code(40),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Diff(Dirty),    Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Diff(Sandy),
                    ],
                ],
                TileCodesSet::from_code(41),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Diff(Sandy),    Eq,             Eq,
                        Eq,                             Eq,
                        Eq,             Eq,             Diff(Sandy),
                    ],
                ],
                TileCodesSet::from_code(42),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[  Eq,             Eq,                 AnyExcept(Sandy),
                        Eq,                                 Diff(Dirty),
                        Diff(Sandy),    AnyExcept(Sandy),   AnyExcept(Sandy),
                    ],
                ],
                TileCodesSet::from_code(43),
                (TerrainVisibleType::None, TileSymmetry::None, ""),
            ),
            (
                vec![[Eq; NEIGHBORHOOD_SIZE]],
                TileCodesSet::with_frequency(49..=56, 5).add_codes(57..=72, 1),
                (TerrainVisibleType::Same, TileSymmetry::None, ""),
            ),
            (
                vec![[  AnyExcept(Sandy),   AnyExcept(Sandy),   AnyExcept(Sandy),
                        Diff(Dirty),                            Diff(Dirty),
                        AnyExcept(Sandy),   AnyExcept(Sandy),   AnyExcept(Sandy),
                    ],
                    [   Diff(Dirty),        AnyExcept(Sandy),   AnyExcept(Sandy),
                        AnyExcept(Sandy),                       Diff(Dirty),
                        AnyExcept(Sandy),   Diff(Dirty),        AnyExcept(Sandy),
                    ],
                ],
                TileCodesSet::from_code(73),
                (TerrainVisibleType::Diff(Terrain::Dirt), TileSymmetry::MainDiagonal, ""),
            ),
            (
                vec![[  Any,            Any,            Any,
                        Diff(Sandy),                    DiffAny,
                        Any,            Any,            Any,
                    ],
                    [   Diff(Sandy),    Any,            Any,
                        Any,                            Diff(Sandy),
                        Any,            Diff(Sandy),    Any,
                    ],
                    [   Diff(Dirty),    Any,            Any,
                        Any,                            Diff(Dirty),
                        Any,            Diff(Sandy),    Any,
                    ],
                ],
                TileCodesSet::from_code(74),
                (TerrainVisibleType::Diff(Terrain::Sand), TileSymmetry::MainDiagonal, ""),
            ),
        ];

        let new_common_ground_table =  vec![
            (
                vec![[Eq; NEIGHBORHOOD_SIZE]],
                TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                (TerrainVisibleType::Same, TileSymmetry::None, ""),
            ),
        ];

        let water_table = vec![
            (
                vec![[Eq; NEIGHBORHOOD_SIZE]],
                TileCodesSet::new(21..=32),
                (TerrainVisibleType::Same, TileSymmetry::None, ""),
            ),
        ];

        let rock_table = vec![
            (
                vec![[Eq; NEIGHBORHOOD_SIZE]],
                TileCodesSet::new(0..=7),
                (TerrainVisibleType::Same, TileSymmetry::None, ""),
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
                Terrain::Highland,
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
                            .map(|( patterns,
                                    codes,
                                    (terrain_visible_type, tile_symmetry, name))| {
                                TilesGroupInfo::new(patterns, codes, name, *terrain_visible_type, *tile_symmetry)
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
