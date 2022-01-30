use super::terrain_relation::{
    NeighborhoodPattern, TerrainCategory, TerrainRelation, NEIGHBORHOOD_SIZE,
};
use super::TileCodesSet;
use crate::h3m::terrain_map::tile::TerrainVisibleType;
use crate::h3m::Terrain;
use std::collections::HashMap;

enum TileSymmetry {
    MainDiagonal,
    None,
}

pub struct TilesGroupInfo {
    patterns: Vec<NeighborhoodPattern>,
    codes: TileCodesSet,
    terrain_visible_type: TerrainVisibleType,
}

impl TilesGroupInfo {
    fn new(
        patterns: Vec<NeighborhoodPattern>,
        codes: TileCodesSet,
        terrain_visible_type: TerrainVisibleType,
        _tile_symmetry: TileSymmetry,
    ) -> TilesGroupInfo {
        TilesGroupInfo {
            patterns,
            codes,
            terrain_visible_type,
        }
    }

    pub fn patterns(&self) -> &[NeighborhoodPattern] {
        &self.patterns
    }

    pub fn codes(&self) -> &TileCodesSet {
        &self.codes
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
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Sand,
                vec![(
                    vec![[Any; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(0..=7, 4).add_codes(8..=23, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
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
                            [   Any,            Any,            Diff(Sandy),
                                Diff(Sandy),                    Eq,
                                DiffAny,        Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(20..=23),
                        (TerrainVisibleType::None, TileSymmetry::None),
                    ),
                    (
                        vec![[  Any,            Eq,             Eq,
                                Diff(Sandy),                    Eq,
                                Any,            Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(24..=27),
                        (TerrainVisibleType::None, TileSymmetry::None),
                    ),
                    (
                        vec![[  Any,            Diff(Sandy),    Any,
                                Eq,                             Eq,
                                Eq,             Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(28..=31),
                        (TerrainVisibleType::None, TileSymmetry::None),
                    ),
                    (
                        vec![[  Eq,             Eq,             Eq,
                                Eq,                             Eq,
                                Eq,             Eq,             Diff(Sandy)
                            ],
                        ],
                        TileCodesSet::new(32..=35).add_codes(38..=39, 5),
                        (TerrainVisibleType::None, TileSymmetry::None),
                    ),
                    (
                        vec![[  Any,            Diff(Sandy),    Eq,
                                Diff(Sandy),                    Eq,
                                Eq,             Eq,             Eq
                            ],
                            [   Any,            Diff(Sandy),    DiffAny,
                                Diff(Sandy),                    Eq,
                                Eq,             Eq,             Eq
                            ],
                            [   Any,            Diff(Sandy),    Eq,
                                Diff(Sandy),                    Eq,
                                DiffAny,        Eq,             Eq
                            ],
                            [   Any,            Diff(Sandy),    Eq,
                                Any,                            Eq,
                                Diff(Sandy),    Eq,             Eq
                            ],
                            [   Any,            Any,            Diff(Sandy),
                                Diff(Sandy),                    Eq,
                                Eq,             Eq,             Eq
                            ],
                        ],
                        TileCodesSet::new(36..=37),
                        (TerrainVisibleType::None, TileSymmetry::None),
                    ),
                    (
                        vec![[  Diff(Sandy),    Eq,             Eq,
                                Eq,                             Eq,
                                Eq,             Eq,             Diff(Sandy)
                            ],
                        ],
                        TileCodesSet::from_code(42),
                        (TerrainVisibleType::None, TileSymmetry::None),
                    ),
                    (
                        vec![[Eq; NEIGHBORHOOD_SIZE]],
                        TileCodesSet::with_frequency(49..=56, 5).add_codes(57..=72, 1),
                        (TerrainVisibleType::Same, TileSymmetry::None),
                    ),
                    (
                        vec![[  Any,            Any,            Any,
                                Diff(Sandy),                    Diff(Sandy),
                                Any,            Any,            Any
                            ],
                            [   Any,            Diff(Sandy),    Any,
                                Any,                            Any,
                                Any,            Diff(Sandy),    Any
                            ],
                        ],
                        TileCodesSet::from_code(74),
                        (TerrainVisibleType::Diff(Terrain::Sand), TileSymmetry::None),
                    ),
                ],
            ),
            (
                Terrain::Snow,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Swamp,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Rough,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Subterranean,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Lava,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Highland,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Wasteland,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Water,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::new(21..=32),
                    (TerrainVisibleType::Same, TileSymmetry::None),
                )],
            ),
            (
                Terrain::Rock,
                vec![(
                    vec![[Eq; NEIGHBORHOOD_SIZE]],
                    TileCodesSet::new(0..=7),
                    (TerrainVisibleType::Same, TileSymmetry::None),
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
                            .map(|(patterns, codes, (terrain_visible_type, tile_symmetry))| {
                                TilesGroupInfo::new(patterns, codes, terrain_visible_type, tile_symmetry)
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
