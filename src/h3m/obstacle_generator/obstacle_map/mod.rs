use super::obstacle_template::{CellValidationResult, ObstacleTemplate};
use super::FilenameToTemplateIndexMap;
use crate::common::position::generic::{DeltaPos, Position, SignedDeltaPos};
use crate::h3m::result::*;
use crate::h3m::terrain_map::TerrainMap;
pub use located_obstacle::LocatedObstacle;
pub use obstacle_map_area::*;
pub use obstacle_map_cell::{NeighborhoodSameRelation, ObstacleMapCell};
use rand::{rngs::ThreadRng, Rng};
use sparsity_validator::SparsityValidator;

mod areas_layout;
mod located_obstacle;
mod obstacle_map_area;
mod obstacle_map_cell;
mod sparsity_validator;

fn max_sparsity(obstacle: &ObstacleTemplate) -> usize {
    let max_multi_sparsity = obstacle
        .multi_sparsity()
        .iter()
        .map(|multi_sparsity_entry| multi_sparsity_entry.sparsity().max())
        .max();

    let max_sparsity = obstacle.sparsity().max();

    if let Some(max_multi_sparsity) = max_multi_sparsity {
        std::cmp::max(max_multi_sparsity, max_sparsity)
    } else {
        max_sparsity
    }
}

impl ObstacleMapCell {
    fn from_map_cell_index(index: usize, terrain_map: &TerrainMap) -> H3mResult<ObstacleMapCell> {
        let size = terrain_map.size();
        let row = index / size;
        let column = index % size;
        let map_cell_position = Position::new(row, column);
        let map_cell = &terrain_map.cells()[index];

        let neighbour_same_relation_getter = |delta_row: i32, delta_column: i32| -> bool {
            let neighbour_position = map_cell_position.checked_apply(
                size,
                size,
                &SignedDeltaPos::new(delta_row, delta_column),
            );
            let neighbour_position = match neighbour_position {
                Some(neighbour_position) => neighbour_position,
                None => return true,
            };

            let neighbour = &terrain_map.cells()[neighbour_position.index(size)];
            match (map_cell, neighbour) {
                (Some(map_cell), Some(neighbour)) => {
                    obstacle_map_cell::calc_terrain(map_cell)
                        == obstacle_map_cell::calc_terrain(neighbour)
                }
                _ => true,
            }
        };

        Ok(ObstacleMapCell::new(
            row.try_into()?,
            column.try_into()?,
            *map_cell,
            [
                neighbour_same_relation_getter(-1, -1),
                neighbour_same_relation_getter(-1, 0),
                neighbour_same_relation_getter(-1, 1),
                neighbour_same_relation_getter(0, -1),
                neighbour_same_relation_getter(0, 1),
                neighbour_same_relation_getter(1, -1),
                neighbour_same_relation_getter(1, 0),
                neighbour_same_relation_getter(1, 1),
            ],
        ))
    }
}

pub struct ObstacleMap {
    size: usize,
    cells: Vec<ObstacleMapCell>,
    sparsity_penalty: usize,
    sparsity_validator: SparsityValidator,
}

impl ObstacleMap {
    pub fn new(terrain_map: &TerrainMap) -> H3mResult<ObstacleMap> {
        let size = terrain_map.size();
        let map_len = size * size;
        let cells_len = terrain_map.cells().len();

        if cells_len != map_len {
            return Err(H3mError::Internal(InternalError::new(format!(
                "terrain map cells length ({}) not equal squared map size ({}).",
                cells_len, map_len
            ))));
        }

        let cells = {
            let mut cells = Vec::new();
            for index in 0..map_len {
                cells.push(ObstacleMapCell::from_map_cell_index(index, terrain_map)?);
            }
            cells
        };

        Ok(ObstacleMap {
            size,
            cells,
            sparsity_penalty: 0,
            sparsity_validator: SparsityValidator::new(size),
        })
    }

    pub fn set_sparsity_penalty(&mut self, sparsity_penalty: usize) {
        self.sparsity_penalty = sparsity_penalty;
    }

    pub fn try_position_obstacle(
        &self,
        area: &ObstacleMapArea,
        template_index: usize,
        filename_to_template_index_map: &FilenameToTemplateIndexMap,
        obstacle: &ObstacleTemplate,
        rng: &mut ThreadRng,
    ) -> Option<usize> {
        struct LocalMultiSparsityEntry {
            sparsity: usize,
            neighbor_index: usize,
        }

        let multi_sparsity: Vec<LocalMultiSparsityEntry> = obstacle
            .multi_sparsity()
            .iter()
            .map(|multi_sparsity_entry| LocalMultiSparsityEntry {
                sparsity: rng.gen_range(
                    multi_sparsity_entry.sparsity().min()..=multi_sparsity_entry.sparsity().max(),
                ),
                neighbor_index: filename_to_template_index_map
                    .template_index(multi_sparsity_entry.neighbor_name())
                    .unwrap(),
            })
            .collect();

        fn apply_sparsity_penalty(sparsity: usize, sparsity_penalty: usize) -> usize {
            if sparsity >= sparsity_penalty {
                sparsity - sparsity_penalty
            } else {
                0
            }
        }

        let sparsity = if self.sparsity_penalty == 0 {
            rng.gen_range(obstacle.sparsity().min()..=obstacle.sparsity().max())
        } else {
            apply_sparsity_penalty(obstacle.sparsity().min(), self.sparsity_penalty)
        };

        let validate_delta = |position: &Position<usize>, delta: &DeltaPos<usize>| {
            let delta_position = position.checked_sub_delta(delta);
            if let Some(delta_position) = delta_position {
                let delta_position_index = delta_position.index(self.size);
                let delta_cell = &self.cells[delta_position_index];
                obstacle.validate_cell(delta_cell, position)
            } else {
                CellValidationResult::Invalid
            }
        };

        let is_valid_delta_sparsity = |delta_position, is_overlapping| {
            let final_sparsity = if is_overlapping {
                apply_sparsity_penalty(sparsity, obstacle.overlap_obstacle_sparsity_penalty())
            } else {
                sparsity
            };

            self.sparsity_validator
                .verify_position(template_index, final_sparsity, delta_position)
        };

        let is_valid_delta_multi_sparsity = |delta_position| {
            for multi_sparsity_entry in &multi_sparsity {
                if !self.sparsity_validator.verify_position(
                    multi_sparsity_entry.neighbor_index,
                    multi_sparsity_entry.sparsity,
                    delta_position,
                ) {
                    return false;
                }
            }
            true
        };

        let is_valid_index = |index| {
            let position = Position::from_index(self.size, index);
            let mut is_overlapping = false;

            for delta in obstacle.shape() {
                match validate_delta(&position, delta) {
                    CellValidationResult::Valid => (),
                    CellValidationResult::ValidWithOverlapping => is_overlapping = true,
                    CellValidationResult::Invalid => return false,
                }
            }

            for delta in obstacle.shape() {
                let delta_position = position.checked_sub_delta(delta).unwrap();
                if !is_valid_delta_sparsity(delta_position, is_overlapping) {
                    return false;
                }
                if !is_valid_delta_multi_sparsity(delta_position) {
                    return false;
                }
            }

            true
        };

        area.indexes()
            .iter()
            .rev()
            .find(|&&index| is_valid_index(index))
            .copied()
    }

    pub fn add_obstacle(
        &mut self,
        position_index: usize,
        template_index: usize,
        obstacle: &ObstacleTemplate,
    ) {
        let position = Position::from_index(self.size, position_index);
        for delta in obstacle.shape() {
            let delta_position = position.sub_delta(delta);
            let delta_position_index = delta_position.index(self.size);
            self.cells[delta_position_index].set_obstacle(obstacle, position);

            self.sparsity_validator.add_position(
                template_index,
                max_sparsity(obstacle),
                delta_position,
            );
        }
    }

    pub fn position(&self, index: usize) -> Position<u8> {
        self.cells[index].position()
    }

    pub fn generalized_terrain_group(&self) -> u16 {
        self.cells
            .iter()
            .fold(0, |result, cell| result | cell.terrain_group())
    }

    pub fn first_position_to_place_obstacle(&self) -> Option<Position<usize>> {
        for (index, cell) in self.cells.iter().enumerate() {
            if cell.need_place_obstacle() {
                return Some(Position::from_index(self.size, index));
            }
        }
        None
    }
}
