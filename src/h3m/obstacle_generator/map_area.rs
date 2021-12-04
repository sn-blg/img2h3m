use super::obstacle_template::{Delta, ObstacleTemplate};
use crate::h3m::result::*;
use crate::h3m::terrain_map::TerrainMap;
use std::cmp::min;

#[derive(Clone, Copy)]
pub struct Position<T: Clone + Copy> {
    row: T,
    column: T,
}

impl<T: Clone + Copy> Position<T> {
    fn new(row: T, column: T) -> Position<T> {
        Position { row, column }
    }

    pub fn row(&self) -> T {
        self.row
    }

    pub fn column(&self) -> T {
        self.column
    }
}

#[derive(Clone, Copy)]
struct AreaCell {
    position: Position<u8>,
    terrain_group: u16, // terrain editor group, 0 means no obstacles to place
}

impl AreaCell {
    fn new(row: u8, column: u8) -> AreaCell {
        AreaCell {
            position: Position::new(row, column),
            terrain_group: 0,
        }
    }

    fn reset_terrain_group(&mut self) {
        self.terrain_group = 0;
    }
}

pub struct MapArea {
    width: usize,
    cells: Vec<AreaCell>,
}

impl MapArea {
    fn new(width: usize, cells: Vec<AreaCell>) -> MapArea {
        MapArea { width, cells }
    }

    pub fn try_position_obstacle(&self, obstacle: &ObstacleTemplate) -> Option<usize> {
        let is_valid_neighbour = |position: Position<usize>, delta: &Delta| {
            let row = position.row().checked_sub(delta.row());
            let column = position.column().checked_sub(delta.column());
            match (row, column) {
                (Some(row), Some(column)) => {
                    let neighbour = &self.cells[self.calc_index(row, column)];
                    obstacle.is_valid_terrain(neighbour.terrain_group)
                }
                _ => false,
            }
        };

        'cell_traversal: for index in 0..self.cells.len() {
            let position = self.local_position(index);
            for delta in obstacle.shape() {
                if !is_valid_neighbour(position, delta) {
                    continue 'cell_traversal;
                }
            }
            return Some(index);
        }
        None
    }

    pub fn add_obstacle(&mut self, index: usize, obstacle: &ObstacleTemplate) {
        let position = self.local_position(index);
        for delta in obstacle.shape() {
            let row = position.row() - delta.row();
            let column = position.column() - delta.column();
            let index = self.calc_index(row, column);
            self.cells[index].reset_terrain_group();
        }
    }

    pub fn position(&self, index: usize) -> Position<u8> {
        self.cells[index].position
    }

    fn local_position(&self, index: usize) -> Position<usize> {
        Position::new(index / self.width, index % self.width)
    }

    fn calc_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
}

pub fn make_map_areas(
    terrain_map: &TerrainMap,
    width: usize,
    height: usize,
) -> H3mResult<Vec<MapArea>> {
    let map_size = terrain_map.size();

    let ceil = |a: usize, b: usize| (a as f64 / b as f64).ceil() as usize;

    let areas_at_row = ceil(map_size, width);
    let areas_count = areas_at_row * ceil(map_size, height);

    let mut areas_cells = vec![Vec::new(); areas_count];

    for (index, map_cell) in terrain_map.cells().iter().enumerate() {
        let row = index / map_size;
        let column = index % map_size;

        let mut area_cell = AreaCell::new(row.try_into()?, column.try_into()?);
        if let Some(map_cell) = map_cell {
            if map_cell.surface().obstacle {
                area_cell.terrain_group = map_cell.surface().terrain.group();
            }
        }

        let area_index = (row / height) * areas_at_row + (column / width);

        areas_cells[area_index].push(area_cell);
    }

    let area_width = |area_index: usize| {
        let local_row_index = area_index % areas_at_row;
        let area_row_offset = width * local_row_index;
        assert!(map_size > area_row_offset);
        min(width, map_size - area_row_offset)
    };

    Ok(areas_cells
        .into_iter()
        .enumerate()
        .map(|(area_index, cells)| MapArea::new(area_width(area_index), cells))
        .collect())
}
