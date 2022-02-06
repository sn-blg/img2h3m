use super::draft_map_cell::DraftMapCell;
use crate::h3m::{
    terrain_map::tile::{TerrainVisibleType, Tile, TileComposition},
    Terrain,
};
use rand::{rngs::ThreadRng, Rng};
use std::cmp::Ordering;
use terrain_relation::{NeighborhoodPattern, TerrainRelation, NEIGHBORHOOD_SIZE};
use tile_codes_set::TileCodesSet;
use tiles_table::{TilesGroupInfo, TilesTable};

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
            .map(|t| match t.terrain_visible_type() {
                TerrainVisibleType::Diff(terrain_visible_type) => Some(terrain_visible_type),
                _ => None,
            })
            .flatten()
            .unwrap_or(neighbour.surface.terrain);
        match relation {
            TerrainRelation::Eq => neighbour_terrain == terrain,
            TerrainRelation::EqOr(category) => {
                (neighbour_terrain == terrain) || (neighbour_terrain.category() == category)
            }
            TerrainRelation::SameNamed(names) => {
                if let Some(neighbour_tile) = neighbour.tile {
                    (neighbour_terrain == terrain) && (names.contains(&neighbour_tile.name()))
                } else {
                    false
                }
            }
            TerrainRelation::Diff(category) => {
                (neighbour_terrain != terrain) && (neighbour_terrain.category() == category)
            }
            TerrainRelation::DiffAny => (neighbour_terrain != terrain),
            TerrainRelation::Any => true,
        }
    } else {
        matches!(
            relation,
            TerrainRelation::Eq | TerrainRelation::EqOr(_) | TerrainRelation::Any
        )
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
        neighborhood: &Neighborhood,
        excluded_tile_codes: &[u8],
        composition: TileComposition,
    ) -> Option<(u8, &TilesGroupInfo)> {
        let generate_code = |tile_codes_set: &TileCodesSet| {
            tile_codes_set
                .random_not_excluded_code(excluded_tile_codes)
                .unwrap_or_else(|| tile_codes_set.random_code())
        };
        for tiles_group_info in self.tiles_table.terrain_tile_groups(cell.surface.terrain) {
            if tiles_group_info.composition() != composition {
                continue;
            }
            for pattern in tiles_group_info.patterns() {
                if is_neighborhood_pattern_matched(cell, neighborhood, pattern) {
                    return Some((generate_code(tiles_group_info.codes()), tiles_group_info));
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
                    return tiles_group_info.codes().contains_code(code);
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

    fn try_generate_impl(
        &self,
        cell: &DraftMapCell,
        neighborhood: &Neighborhood,
        composition: TileComposition,
    ) -> Option<Tile> {
        let excluded_tile_codes = TileGenerator::excluded_tile_codes(cell, neighborhood);
        for horizontal_mirroring in [false, true] {
            for vertical_mirroring in [false, true] {
                let code_info = if (false, false) == (vertical_mirroring, horizontal_mirroring) {
                    self.try_generate_code(cell, neighborhood, &excluded_tile_codes, composition)
                } else {
                    let mirroring_neighborhood = mirroring_neighborhood(
                        neighborhood,
                        vertical_mirroring,
                        horizontal_mirroring,
                    );
                    self.try_generate_code(
                        cell,
                        &mirroring_neighborhood,
                        &excluded_tile_codes,
                        composition,
                    )
                };
                if let Some((code, tiles_group_info)) = code_info {
                    return Some(Tile::new(
                        tiles_group_info.name(),
                        tiles_group_info.terrain_visible_type(),
                        code,
                        vertical_mirroring,
                        horizontal_mirroring,
                    ));
                }
            }
        }
        None
    }

    fn is_valid_tile(
        &self,
        cell: &DraftMapCell,
        neighborhood: &Neighborhood,
        use_fallback: bool,
    ) -> bool {
        if let Some(tile) = cell.tile {
            if use_fallback {
                if let TerrainVisibleType::Diff(Terrain::Sand) = tile.terrain_visible_type() {
                    return true;
                }
            }
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

    fn try_randomize_mirroring(&mut self, neighborhood: &Neighborhood, tile: &mut Tile) {
        let get_neighbour_tile = |neighbour_cell: &Option<DraftMapCell>, terrain_visible_type| {
            let neighbour_cell = match neighbour_cell {
                Some(neighbour_cell) => neighbour_cell,
                None => return None,
            };
            let neighbour_tile = neighbour_cell.tile?;

            if neighbour_tile.terrain_visible_type()
                != TerrainVisibleType::Diff(terrain_visible_type)
            {
                return None;
            }

            Some(neighbour_tile)
        };

        let get_bad_combinations_count =
            |terrain_visible_type, vertical_mirroring, horizontal_mirroring| {
                let vertical_neighbour_indexes = [1, 6];
                let horizontal_neighbour_indexes = [3, 4];
                let mut count = 0;
                for (neighbour_tile, index) in
                    [vertical_neighbour_indexes, horizontal_neighbour_indexes]
                        .concat()
                        .into_iter()
                        .map(|index| {
                            Some((
                                get_neighbour_tile(&neighborhood[index], terrain_visible_type)?,
                                index,
                            ))
                        })
                        .flatten()
                {
                    if (neighbour_tile.horizontal_mirroring() == horizontal_mirroring)
                        && (neighbour_tile.vertical_mirroring() == vertical_mirroring)
                    {
                        count += 1;
                    }

                    if (terrain_visible_type == Terrain::Sand)
                        && horizontal_neighbour_indexes.contains(&index)
                        && (neighbour_tile.horizontal_mirroring() != horizontal_mirroring)
                    {
                        count += 1;
                    }
                }
                count
            };

        let terrain_visible_type = match tile.terrain_visible_type() {
            TerrainVisibleType::Diff(terrain_visible_type) => terrain_visible_type,
            _ => return,
        };

        let mut mirrorings = Vec::new();
        let mut current_bad_combinations_count = None;
        for vertical_mirroring in [false, true] {
            for horizontal_mirroring in [false, true] {
                let bad_combinations_count = get_bad_combinations_count(
                    terrain_visible_type,
                    vertical_mirroring,
                    horizontal_mirroring,
                );
                if let Some(current_bad_combinations_count_value) = current_bad_combinations_count {
                    match bad_combinations_count.cmp(&current_bad_combinations_count_value) {
                        Ordering::Equal => {
                            mirrorings.push((vertical_mirroring, horizontal_mirroring))
                        }
                        Ordering::Less => {
                            mirrorings.clear();
                            mirrorings.push((vertical_mirroring, horizontal_mirroring));
                            current_bad_combinations_count = Some(bad_combinations_count);
                        }
                        Ordering::Greater => (),
                    }
                } else {
                    current_bad_combinations_count = Some(bad_combinations_count);
                    mirrorings.push((vertical_mirroring, horizontal_mirroring));
                }
            }
        }

        let (vertical_mirroring, horizontal_mirroring) =
            mirrorings[self.rng.gen_range(0..mirrorings.len())];

        tile.set_vertical_mirroring(vertical_mirroring);
        tile.set_horizontal_mirroring(horizontal_mirroring);
    }

    pub fn try_generate(
        &mut self,
        cell: &DraftMapCell,
        neighborhood: &Neighborhood,
        use_fallback: bool,
    ) -> Option<Tile> {
        let tile = if self.is_valid_tile(cell, neighborhood, use_fallback) {
            cell.tile.unwrap()
        } else {
            let mut tile = if let Some(tile) =
                self.try_generate_impl(cell, neighborhood, TileComposition::Main)
            {
                tile
            } else if use_fallback {
                self.try_generate_impl(cell, neighborhood, TileComposition::Fallback)?
            } else {
                return None;
            };
            self.try_randomize_mirroring(neighborhood, &mut tile);
            tile
        };
        Some(tile)
    }
}
