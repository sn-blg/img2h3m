use super::obstacle_template::{Delta, ObstacleTemplate};
use crate::h3m::result::*;
use crate::h3m::Surface;

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

struct MapCell {
    position: Position<u8>,
    terrain_group: u16, // surface editor group, 0 means no obstacles to place
}

impl MapCell {
    fn new(row: u8, column: u8) -> MapCell {
        MapCell {
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
    cells: Vec<MapCell>,
}

impl MapArea {
    fn new(width: usize, cells: Vec<MapCell>) -> MapArea {
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
    map_size: usize,
    surfaces: &[Option<Surface>],
    width: usize,
    _height: usize,
) -> H3mResult<Vec<MapArea>> {
    let mut cells = Vec::with_capacity(surfaces.len());

    for (index, surface) in surfaces.iter().enumerate() {
        let row = (index / map_size).try_into()?;
        let column = (index % map_size).try_into()?;
        let mut cell = MapCell::new(row, column);

        if let Some(surface) = surface {
            if surface.obstacle {
                cell.terrain_group = surface.terrain.group();
            }
        }

        cells.push(cell)
    }

    Ok(vec![MapArea::new(width, cells)])
}
