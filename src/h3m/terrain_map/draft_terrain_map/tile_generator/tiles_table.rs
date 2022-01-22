use super::terrain_relation::{
    NeighborhoodPattern, TerrainCategory, TerrainRelation, NEIGHBORHOOD_SIZE,
};
use super::TileCodesSet;
use crate::h3m::terrain_map::tile::TerrainVisibleType;
use crate::h3m::Terrain;
use std::collections::HashMap;

pub struct TilesGroupInfo {
    pattern: NeighborhoodPattern,
    codes: TileCodesSet,
    terrain_visible_type: TerrainVisibleType,
}

impl TilesGroupInfo {
    fn new(
        pattern: NeighborhoodPattern,
        codes: TileCodesSet,
        terrain_visible_type: TerrainVisibleType,
    ) -> TilesGroupInfo {
        TilesGroupInfo {
            pattern,
            codes,
            terrain_visible_type,
        }
    }

    pub fn pattern(&self) -> &NeighborhoodPattern {
        &self.pattern
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
    pub fn new() -> TilesTable {
        use TerrainCategory::*;
        use TerrainRelation::*;

        let table = [
            (
                Terrain::Dirt,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(21..=28, 4).add_codes(29..=44, 1),
                    Some(Terrain::Dirt),
                )],
            ),
            (
                Terrain::Sand,
                vec![(
                    [Any; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(0..=7, 4).add_codes(8..=23, 1),
                    Some(Terrain::Sand),
                )],
            ),
            (
                Terrain::Grass,
                vec![
                    (
                        [Any, Diff(Sandy), DiffAny, Diff(Sandy), Eq, DiffAny, Eq, Eq],
                        TileCodesSet::new(20..=23),
                        None,
                    ),
                    (
                        [Any, Diff(Sandy), DiffAny, Any, Eq, Diff(Sandy), Eq, Eq],
                        TileCodesSet::new(20..=23),
                        None,
                    ),
                    (
                        [Any, Eq, Eq, Diff(Sandy), Eq, Any, Eq, Eq],
                        TileCodesSet::new(24..=27),
                        None,
                    ),
                    (
                        [Diff(Sandy), Eq, Eq, Any, Eq, Diff(Sandy), Eq, Eq],
                        TileCodesSet::new(24..=27),
                        None,
                    ),
                    (
                        [Any, Diff(Sandy), Any, Eq, Eq, Eq, Eq, Eq],
                        TileCodesSet::new(28..=31),
                        None,
                    ),
                    (
                        [Diff(Sandy), Any, Diff(Sandy), Eq, Eq, Eq, Eq, Eq],
                        TileCodesSet::new(28..=31),
                        None,
                    ),
                    (
                        [Eq, Eq, Eq, Eq, Eq, Eq, Eq, Diff(Sandy)],
                        TileCodesSet::new(32..=35).add_codes(38..=39, 2),
                        None,
                    ),
                    (
                        [Any, Diff(Sandy), Any, Diff(Sandy), Eq, Eq, Eq, Eq],
                        TileCodesSet::new(36..=37),
                        None,
                    ),
                    (
                        [Any, Diff(Sandy), Eq, Diff(Sandy), Eq, Any, Eq, Eq],
                        TileCodesSet::new(36..=37),
                        None,
                    ),
                    (
                        [Any, Diff(Sandy), Eq, Any, Eq, Diff(Sandy), Eq, Eq],
                        TileCodesSet::new(36..=37),
                        None,
                    ),
                    (
                        [Diff(Sandy), Eq, Eq, Eq, Eq, Eq, Eq, Diff(Sandy)],
                        TileCodesSet::from_code(42),
                        None,
                    ),
                    (
                        [Eq; NEIGHBORHOOD_SIZE],
                        TileCodesSet::with_frequency(49..=56, 5).add_codes(57..=72, 1),
                        Some(Terrain::Grass),
                    ),
                    (
                        [Any, Any, Any, Diff(Sandy), Diff(Sandy), Any, Any, Any],
                        TileCodesSet::from_code(74),
                        Some(Terrain::Sand),
                    ),
                    (
                        [Any, Diff(Sandy), Any, Any, Any, Any, Diff(Sandy), Any],
                        TileCodesSet::from_code(74),
                        Some(Terrain::Sand),
                    ),
                ],
            ),
            (
                Terrain::Snow,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Snow),
                )],
            ),
            (
                Terrain::Swamp,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Swamp),
                )],
            ),
            (
                Terrain::Rough,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Rough),
                )],
            ),
            (
                Terrain::Subterranean,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Subterranean),
                )],
            ),
            (
                Terrain::Lava,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Lava),
                )],
            ),
            (
                Terrain::Highland,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    Some(Terrain::Highland),
                )],
            ),
            (
                Terrain::Wasteland,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    Some(Terrain::Wasteland),
                )],
            ),
            (
                Terrain::Water,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::new(21..=32),
                    Some(Terrain::Water),
                )],
            ),
            (
                Terrain::Rock,
                vec![(
                    [Eq; NEIGHBORHOOD_SIZE],
                    TileCodesSet::new(0..=7),
                    Some(Terrain::Rock),
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
                            .map(|(pattern, codes, terrain_visible_type)| {
                                TilesGroupInfo::new(pattern, codes, terrain_visible_type)
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
