use super::draft_map_cell::DraftMapCell;
use super::tile::{TerrainVisibleType, Tile};
use super::tile_codes_set::TileCodesSet;
use crate::h3m::Terrain;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum TerrainCategory {
    Sandy,
    Dirty,
}

impl Terrain {
    fn category(self) -> TerrainCategory {
        if matches!(self, Terrain::Sand | Terrain::Water | Terrain::Rock) {
            TerrainCategory::Sandy
        } else {
            TerrainCategory::Dirty
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TerrainRelation {
    Eq,                     // None or Some neighbour == central terrain
    Diff(TerrainCategory), // Some neighbour != central terrain and neighbour in TerrainСategory
    DiffAny,                // Some neighbour != central terrain
    Any,                    // any neighbour, including None
}

fn is_terrain_relation_matched(
    terrain: Terrain,
    neighbour: &Option<Terrain>,
    relation: TerrainRelation,
) -> bool {
    if let Some(neighbour) = neighbour {
        match relation {
            TerrainRelation::Eq => *neighbour == terrain,
            TerrainRelation::Diff(category) => {
                (*neighbour != terrain) && (neighbour.category() == category)
            }
            TerrainRelation::DiffAny => (*neighbour != terrain),
            TerrainRelation::Any => true,
        }
    } else {
        matches!(relation, TerrainRelation::Eq | TerrainRelation::Any)
    }
}

type Neighborhood = [Option<Terrain>; 8];
type NeighborhoodPattern = [TerrainRelation; 8];

fn is_neighborhood_pattern_matched(
    terrain: Terrain,
    neighborhood: &Neighborhood,
    neighborhood_pattern: &NeighborhoodPattern,
) -> bool {
    for (neighbour, &relation) in neighborhood.iter().zip(neighborhood_pattern) {
        if !is_terrain_relation_matched(terrain, neighbour, relation) {
            return false;
        }
    }
    true
}

#[rustfmt::skip]
fn vertical_mirroring_neighborhood(neighborhood: &Neighborhood) -> Neighborhood {
    [
        neighborhood[5], neighborhood[6], neighborhood[7],
        neighborhood[3],                  neighborhood[4],
        neighborhood[0], neighborhood[1], neighborhood[2],
    ]
}

#[rustfmt::skip]
fn horizontal_mirroring_neighborhood(neighborhood: &Neighborhood) -> Neighborhood {
    [
        neighborhood[2], neighborhood[1], neighborhood[0],
        neighborhood[4],                  neighborhood[3],
        neighborhood[7], neighborhood[6], neighborhood[5],
    ]
}

fn mirroring_neighborhood(
    neighborhood: &Neighborhood,
    vertical: bool,
    horizontal: bool,
) -> Neighborhood {
    match (vertical, horizontal) {
        (true, true) => {
            vertical_mirroring_neighborhood(&horizontal_mirroring_neighborhood(neighborhood))
        }
        (true, false) => vertical_mirroring_neighborhood(neighborhood),
        (false, true) => horizontal_mirroring_neighborhood(neighborhood),
        (false, false) => neighborhood.clone(),
    }
}

pub struct TileGenerator {
    tile_codes: HashMap<Terrain, Vec<(NeighborhoodPattern, TileCodesSet, TerrainVisibleType)>>,
}

impl TileGenerator {
    pub fn new() -> TileGenerator {
        use TerrainRelation::*;
        use TerrainCategory::*;

        let tile_codes = HashMap::from([
            (
                Terrain::Dirt,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(21..=28, 4).add_codes(29..=44, 1),
                    Some(Terrain::Dirt),
                )],
            ),
            (
                Terrain::Sand,
                vec![(
                    [Any; 8],
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
                        TileCodesSet::new(42..=42),
                        None,
                    ),
                    (
                        [Eq; 8],
                        TileCodesSet::with_frequency(49..=56, 5).add_codes(57..=72, 1),
                        Some(Terrain::Grass),
                    ),
                    (
                        [Any, Any, Any, Diff(Sandy), Diff(Sandy), Any, Any, Any],
                        TileCodesSet::new(74..=74),
                        Some(Terrain::Sand),
                    ),
                    (
                        [Any, Diff(Sandy), Any, Any, Any, Any, Diff(Sandy), Any],
                        TileCodesSet::new(74..=74),
                        Some(Terrain::Sand),
                    ),
                    (
                        [
                            Any,
                            Diff(Sandy),
                            Any,
                            Any,
                            Any,
                            Diff(Sandy),
                            Any,
                            Diff(Sandy),
                        ],
                        TileCodesSet::new(74..=74),
                        Some(Terrain::Sand),
                    ),
                ],
            ),
            (
                Terrain::Snow,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Snow),
                )],
            ),
            (
                Terrain::Swamp,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Swamp),
                )],
            ),
            (
                Terrain::Rough,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Rough),
                )],
            ),
            (
                Terrain::Subterranean,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Subterranean),
                )],
            ),
            (
                Terrain::Lava,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                    Some(Terrain::Lava),
                )],
            ),
            (
                Terrain::Highland,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    Some(Terrain::Highland),
                )],
            ),
            (
                Terrain::Wasteland,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                    Some(Terrain::Wasteland),
                )],
            ),
            (
                Terrain::Water,
                vec![([Eq; 8], TileCodesSet::new(21..=32), Some(Terrain::Water))],
            ),
            (
                Terrain::Rock,
                vec![([Eq; 8], TileCodesSet::new(0..=7), Some(Terrain::Rock))],
            ),
        ]);
        TileGenerator { tile_codes }
    }

    fn try_generate_code(
        &self,
        terrain: Terrain,
        current_code: Option<u8>,
        neighborhood: &Neighborhood,
        excluded_tile_codes: &[u8],
    ) -> Option<(u8, TerrainVisibleType)> {
        let generate_code = |tile_codes_set: &TileCodesSet| {
            if let Some(current_code) = current_code {
                if tile_codes_set.contains_code(current_code) {
                    return current_code;
                }
            }
            tile_codes_set
                .random_not_excluded_code(excluded_tile_codes)
                .unwrap_or_else(|| tile_codes_set.random_code())
        };

        for (pattern, tile_codes_set, terrain_visible_type) in &self.tile_codes[&terrain] {
            if is_neighborhood_pattern_matched(terrain, neighborhood, pattern) {
                return Some((generate_code(tile_codes_set), *terrain_visible_type));
            }
        }
        None
    }

    fn excluded_tile_codes(cell: &DraftMapCell, neighbors: &[Option<DraftMapCell>; 8]) -> Vec<u8> {
        neighbors
            .iter()
            .filter_map(|c| c.as_ref())
            .filter(|neighbour| neighbour.surface.terrain == cell.surface.terrain)
            .filter_map(|c| Some(c.tile?.code()))
            .collect()
    }

    pub fn try_generate(
        &self,
        cell: &DraftMapCell,
        neighbors: &[Option<DraftMapCell>; 8],
    ) -> Option<Tile> {
        let terrain = cell.surface.terrain;
        let current_code = cell.tile.map(|t| t.code());
        let excluded_tile_codes = TileGenerator::excluded_tile_codes(cell, neighbors);
        let neighborhood = neighbors.map(|c| Some(c?.surface.terrain));

        for vertical_mirroring in [false, true] {
            for horizontal_mirroring in [false, true] {
                let code_info = if (false, false) == (vertical_mirroring, horizontal_mirroring) {
                    self.try_generate_code(
                        terrain,
                        current_code,
                        &neighborhood,
                        &excluded_tile_codes,
                    )
                } else {
                    let mirroring_neighborhood = mirroring_neighborhood(
                        &neighborhood,
                        vertical_mirroring,
                        horizontal_mirroring,
                    );
                    self.try_generate_code(
                        terrain,
                        current_code,
                        &mirroring_neighborhood,
                        &excluded_tile_codes,
                    )
                };
                if let Some((code, terrain_visible_type)) = code_info {
                    return Some(Tile::new(
                        terrain_visible_type,
                        code,
                        vertical_mirroring,
                        horizontal_mirroring,
                    ));
                }
            }
        }
        None
    }
}
