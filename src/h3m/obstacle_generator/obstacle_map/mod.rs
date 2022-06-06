use super::obstacle_template::ObstacleTemplate;
use crate::common::position::generic::Position;
use crate::h3m::result::*;
use crate::h3m::terrain_map::{MapCell, TerrainMap};
pub use obstacle_map_area::*;
use obstacle_map_cell::ObstacleMapCell;
use rand::{rngs::ThreadRng, Rng};
use sparsity_validator::SparsityValidator;

mod areas_layout;
mod obstacle_map_area;
mod obstacle_map_cell;
mod sparsity_validator;

impl ObstacleMapCell {
    fn from_map_cell(
        index: usize,
        size: usize,
        map_cell: &Option<MapCell>,
    ) -> H3mResult<ObstacleMapCell> {
        let row = index / size;
        let column = index % size;
        Ok(ObstacleMapCell::new(
            row.try_into()?,
            column.try_into()?,
            *map_cell,
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
            for (index, map_cell) in terrain_map.cells().iter().enumerate() {
                cells.push(ObstacleMapCell::from_map_cell(index, size, map_cell)?);
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
        obstacle: &ObstacleTemplate,
        rng: &mut ThreadRng,
    ) -> Option<usize> {
        let apply_sparsity_penalty = |sparsity| {
            if sparsity >= self.sparsity_penalty {
                sparsity - self.sparsity_penalty
            } else {
                0
            }
        };
        let sparsity = rng.gen_range(
            apply_sparsity_penalty(obstacle.sparsity().min())
                ..=apply_sparsity_penalty(obstacle.sparsity().max()),
        );

        let is_valid_delta = |delta_position: Option<Position<usize>>| {
            if let Some(delta_position) = delta_position {
                let delta_position_index = delta_position.index(self.size);
                let delta_cell = &self.cells[delta_position_index];

                obstacle.is_valid_terrain(delta_cell.terrain_group())
                    && obstacle.is_valid_tile(delta_cell.map_cell().unwrap().tile())
                    && self.sparsity_validator.verify_position(
                        template_index,
                        sparsity,
                        delta_position,
                    )
            } else {
                false
            }
        };

        let is_valid_index = |index| {
            let position = Position::from_index(self.size, index);
            for delta in obstacle.shape() {
                if !is_valid_delta(position.checked_sub(delta)) {
                    return false;
                }
            }
            true
        };

        for &index in area.indexes().iter().rev() {
            if is_valid_index(index) {
                return Some(index);
            }
        }

        None
    }

    pub fn add_obstacle(
        &mut self,
        position_index: usize,
        template_index: usize,
        obstacle: &ObstacleTemplate,
    ) {
        let position = Position::from_index(self.size, position_index);
        for delta in obstacle.shape() {
            let delta_position = position.sub(delta);
            let delta_position_index = delta_position.index(self.size);
            self.cells[delta_position_index].set_template(template_index);

            self.sparsity_validator.add_position(
                template_index,
                obstacle.sparsity().max(),
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
            if cell.terrain_group() != 0 {
                return Some(Position::from_index(self.size, index));
            }
        }
        None
    }
}
