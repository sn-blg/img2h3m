use crate::h3m::result::*;
use crate::h3m::terrain_map::{MapCell, TerrainMap};
use obstacle_map_cell::ObstacleMapCell;

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
}
