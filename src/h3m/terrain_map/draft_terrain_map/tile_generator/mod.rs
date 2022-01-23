use super::draft_map_cell::DraftMapCell;
use crate::h3m::terrain_map::tile::{TerrainVisibleType, Tile};
use rand::{rngs::ThreadRng, Rng};
use terrain_relation::{NeighborhoodPattern, TerrainRelation, NEIGHBORHOOD_SIZE};
use tile_codes_set::TileCodesSet;
use tiles_table::TilesTable;

mod terrain_relation;
mod tile_codes_set;
mod tiles_table;

fn is_terrain_relation_matched(
    cell: &DraftMapCell,
    neighbour: &Option<DraftMapCell>,
    relation: TerrainRelation,
) -> bool {
    if let Some(neighbour) = neighbour {
        let terrain = cell.surface.terrain;
        let neighbour_terrain = neighbour
            .tile
            .map(|t| t.terrain_visible_type())
            .flatten()
            .unwrap_or(neighbour.surface.terrain);
        match relation {
            TerrainRelation::Eq => neighbour_terrain == terrain,
            TerrainRelation::Diff(category) => {
                (neighbour_terrain != terrain) && (neighbour_terrain.category() == category)
            }
            TerrainRelation::DiffAny => (neighbour_terrain != terrain),
            TerrainRelation::Any => true,
        }
    } else {
        matches!(relation, TerrainRelation::Eq | TerrainRelation::Any)
    }
}

type Neighborhood = [Option<DraftMapCell>; NEIGHBORHOOD_SIZE];

fn is_neighborhood_pattern_matched(
    cell: &DraftMapCell,
    neighborhood: &Neighborhood,
    neighborhood_pattern: &NeighborhoodPattern,
) -> bool {
    for (neighbour, &relation) in neighborhood.iter().zip(neighborhood_pattern) {
        if !is_terrain_relation_matched(cell, neighbour, relation) {
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
        (false, false) => *neighborhood,
    }
}

pub struct TileGenerator {
    tiles_table: TilesTable,
    rng: ThreadRng,
}

impl TileGenerator {
    pub fn new() -> TileGenerator {
        TileGenerator {
            tiles_table: TilesTable::new(),
            rng: rand::thread_rng(),
        }
    }

    fn try_generate_code(
        &self,
        cell: &DraftMapCell,
        neighborhood: Neighborhood,
        excluded_tile_codes: &[u8],
    ) -> Option<(u8, TerrainVisibleType)> {
        let generate_code = |tile_codes_set: &TileCodesSet| {
            tile_codes_set
                .random_not_excluded_code(excluded_tile_codes)
                .unwrap_or_else(|| tile_codes_set.random_code())
        };
        for tiles_group_info in self.tiles_table.terrain_tile_groups(cell.surface.terrain) {
            for pattern in tiles_group_info.patterns() {
                if is_neighborhood_pattern_matched(cell, &neighborhood, pattern) {
                    return Some((
                        generate_code(tiles_group_info.codes()),
                        *tiles_group_info.terrain_visible_type(),
                    ));
                }
            }
        }
        None
    }

    fn is_valid_code(&self, cell: &DraftMapCell, neighborhood: &Neighborhood) -> bool {
        if let Some(tile) = cell.tile {
            let code = tile.code();
            for tiles_group_info in self.tiles_table.terrain_tile_groups(cell.surface.terrain) {
                for pattern in tiles_group_info.patterns() {
                    if !is_neighborhood_pattern_matched(cell, neighborhood, pattern) {
                        continue;
                    }
                    if !tiles_group_info.codes().contains_code(code) {
                        continue;
                    }
                    return true;
                }
            }
        }
        false
    }

    fn excluded_tile_codes(cell: &DraftMapCell, neighborhood: &Neighborhood) -> Vec<u8> {
        neighborhood
            .iter()
            .filter_map(|c| c.as_ref())
            .filter(|neighbour| neighbour.surface.terrain == cell.surface.terrain)
            .filter_map(|c| Some(c.tile?.code()))
            .collect()
    }

    fn try_generate_impl(&self, cell: &DraftMapCell, neighborhood: Neighborhood) -> Option<Tile> {
        let excluded_tile_codes = TileGenerator::excluded_tile_codes(cell, &neighborhood);
        for vertical_mirroring in [false, true] {
            for horizontal_mirroring in [false, true] {
                let code_info = if (false, false) == (vertical_mirroring, horizontal_mirroring) {
                    self.try_generate_code(cell, neighborhood, &excluded_tile_codes)
                } else {
                    let mirroring_neighborhood = mirroring_neighborhood(
                        &neighborhood,
                        vertical_mirroring,
                        horizontal_mirroring,
                    );
                    self.try_generate_code(cell, mirroring_neighborhood, &excluded_tile_codes)
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

    fn is_valid_tile(&self, cell: &DraftMapCell, neighborhood: &Neighborhood) -> bool {
        if let Some(tile) = cell.tile {
            let vertical_mirroring = tile.vertical_mirroring();
            let horizontal_mirroring = tile.horizontal_mirroring();
            if (false, false) == (vertical_mirroring, horizontal_mirroring) {
                self.is_valid_code(cell, neighborhood)
            } else {
                let mirroring_neighborhood =
                    mirroring_neighborhood(neighborhood, vertical_mirroring, horizontal_mirroring);
                self.is_valid_code(cell, &mirroring_neighborhood)
            }
        } else {
            false
        }
    }

    pub fn try_generate(
        &mut self,
        cell: &DraftMapCell,
        neighborhood: Neighborhood,
    ) -> Option<Tile> {
        let tile = if self.is_valid_tile(cell, &neighborhood) {
            cell.tile.unwrap()
        } else {
            let mut tile = self.try_generate_impl(cell, neighborhood)?;
            if let Some(terrain_visible_type) = tile.terrain_visible_type() {
                if terrain_visible_type != cell.surface.terrain {
                    tile.set_vertical_mirroring(self.rng.gen());
                    tile.set_horizontal_mirroring(self.rng.gen());
                }
            }
            tile
        };
        Some(tile)
    }
}
