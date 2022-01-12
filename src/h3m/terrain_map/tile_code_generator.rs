use super::draft_map_cell::DraftMapCell;
use super::tile::Tile;
use super::tile_codes_set::TileCodesSet;
use crate::h3m::Terrain;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum TerrainСategory {
    Sandy,
    Dirty,
}

impl Terrain {
    fn category(self) -> TerrainСategory {
        if matches!(self, Terrain::Sand | Terrain::Water | Terrain::Rock) {
            TerrainСategory::Sandy
        } else {
            TerrainСategory::Dirty
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum TerrainRelation {
    Eq,                     // None or Some neighbour == central terrain
    Diff(TerrainСategory), // Some neighbour != central terrain and neighbour in TerrainСategory
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

pub struct TileCodeGenerator {
    // todo: TileGenerator
    tile_codes: HashMap<Terrain, Vec<(NeighborhoodPattern, TileCodesSet)>>,
}

impl TileCodeGenerator {
    pub fn new() -> TileCodeGenerator {
        use TerrainRelation::*;
        use TerrainСategory::*;

        let tile_codes = HashMap::from([
            (
                Terrain::Dirt,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(21..=28, 4).add_codes(29..=44, 1),
                )],
            ),
            (
                Terrain::Sand,
                vec![(
                    [Any; 8],
                    TileCodesSet::with_frequency(0..=7, 4).add_codes(8..=23, 1),
                )],
            ),
            (
                Terrain::Grass,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                )],
            ),
            (
                Terrain::Snow,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                )],
            ),
            (
                Terrain::Swamp,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                )],
            ),
            (
                Terrain::Rough,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                )],
            ),
            (
                Terrain::Subterranean,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                )],
            ),
            (
                Terrain::Lava,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(49..=56, 4).add_codes(57..=72, 1),
                )],
            ),
            (
                Terrain::Highland,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                )],
            ),
            (
                Terrain::Wasteland,
                vec![(
                    [Eq; 8],
                    TileCodesSet::with_frequency(77..=101, 4).add_codes(102..=117, 1),
                )],
            ),
            (Terrain::Water, vec![([Eq; 8], TileCodesSet::new(21..=32))]),
            (Terrain::Rock, vec![([Eq; 8], TileCodesSet::new(0..=7))]),
        ]);
        TileCodeGenerator { tile_codes }
    }

    fn try_generate_code(
        &self,
        terrain: Terrain,
        neighborhood: &Neighborhood,
        excluded_tile_codes: &[u8],
    ) -> Option<u8> {
        for (pattern, tile_codes_set) in &self.tile_codes[&terrain] {
            if is_neighborhood_pattern_matched(terrain, neighborhood, pattern) {
                return Some(
                    tile_codes_set
                        .random_not_excluded_code(excluded_tile_codes)
                        .unwrap_or_else(|| tile_codes_set.random_code()),
                );
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
        let excluded_tile_codes = TileCodeGenerator::excluded_tile_codes(cell, neighbors);
        let neighborhood = neighbors.map(|c| Some(c?.surface.terrain));

        for vertical_mirroring in [false, true] {
            for horizontal_mirroring in [false, true] {
                let code = if (false, false) == (vertical_mirroring, horizontal_mirroring) {
                    self.try_generate_code(terrain, &neighborhood, &excluded_tile_codes)
                } else {
                    let mirroring_neighborhood = mirroring_neighborhood(
                        &neighborhood,
                        vertical_mirroring,
                        horizontal_mirroring,
                    );
                    self.try_generate_code(terrain, &mirroring_neighborhood, &excluded_tile_codes)
                };
                if let Some(code) = code {
                    return Some(Tile::new(code, vertical_mirroring, horizontal_mirroring));
                }
            }
        }
        None
    }
}
