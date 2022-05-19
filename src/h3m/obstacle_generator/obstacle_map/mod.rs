use super::obstacle_template::ObstacleTemplate;
use crate::common::position::generic::Position;
use crate::h3m::result::*;
use crate::h3m::terrain_map::{MapCell, TerrainMap};
pub use obstacle_map_area::*;
use obstacle_map_cell::ObstacleMapCell;

mod obstacle_map_area;
mod obstacle_map_cell;

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

        Ok(ObstacleMap { size, cells })
    }

    pub fn try_position_obstacle(
        &self,
        area: &ObstacleMapArea,
        obstacle: &ObstacleTemplate,
    ) -> Option<usize> {
        let is_valid_neighbour = |neighbour_position: Option<Position<usize>>| {
            if let Some(neighbour_position) = neighbour_position {
                let neighbour_index = neighbour_position.index(self.size);
                let neighbour = &self.cells[neighbour_index];
                obstacle.is_valid_terrain(neighbour.terrain_group())
                    && obstacle.is_valid_tile(neighbour.map_cell().unwrap().tile())
            } else {
                false
            }
        };

        'cell_traversal: for &index in area.indexes() {
            let position = Position::from_index(self.size, index);
            for delta in obstacle.shape() {
                if !is_valid_neighbour(position.checked_sub(delta)) {
                    continue 'cell_traversal;
                }
            }
            return Some(index);
        }
        None
    }

    pub fn add_obstacle(
        &mut self,
        index: usize,
        template_index: usize,
        obstacle: &ObstacleTemplate,
    ) {
        let position = Position::from_index(self.size, index);
        for delta in obstacle.shape() {
            let index = position.sub(delta).index(self.size);
            self.cells[index].set_template(template_index);
        }
    }

    pub fn position(&self, index: usize) -> Position<u8> {
        self.cells[index].position()
    }
}
