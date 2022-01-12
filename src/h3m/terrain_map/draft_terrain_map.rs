use super::draft_map_cell::DraftMapCell;
use super::map_cell::MapCell;
use super::tile_code_generator::TileCodeGenerator;
use crate::common::position::{Position, SignedDeltaPos};
use crate::h3m::Surface;

pub struct DraftTerrainMap {
    size: usize,
    cells: Vec<Option<DraftMapCell>>,
}

impl DraftTerrainMap {
    pub fn new(size: usize, surfaces: &[Option<Surface>]) -> DraftTerrainMap {
        DraftTerrainMap {
            size,
            cells: surfaces
                .iter()
                .enumerate()
                .map(|(index, surface)| {
                    surface.map(|surface| {
                        DraftMapCell::new(surface, Position::from_index(size, index))
                    })
                })
                .collect(),
        }
    }

    fn neighbours(&self, index: usize) -> [Option<DraftMapCell>; 8] {
        let cell = match &self.cells[index] {
            Some(cell) => cell,
            None => return [None; 8],
        };

        let neighbour_getter = |delta_row: i32, delta_column: i32| {
            let neighbour_position = cell.position.checked_apply(
                self.size,
                self.size,
                &SignedDeltaPos::new(delta_row, delta_column),
            );
            self.cells[neighbour_position?.index(self.size)]
        };

        [
            neighbour_getter(-1, -1),
            neighbour_getter(-1, 0),
            neighbour_getter(-1, 1),
            neighbour_getter(0, -1),
            neighbour_getter(0, 1),
            neighbour_getter(1, -1),
            neighbour_getter(1, 0),
            neighbour_getter(1, 1),
        ]
    }

    pub fn set_tile_codes(&mut self) {
        let generator = TileCodeGenerator::new();
        for index in 0..self.size * self.size {
            let neighbors = self.neighbours(index);
            if let Some(cell) = &mut self.cells[index] {
                cell.tile = generator.try_generate(cell, &neighbors);
            }
        }
    }

    pub fn into_map_cells(self) -> Vec<Option<MapCell>> {
        self.cells
            .into_iter()
            .map(|cell| cell.map(|cell| cell.to_map_cell()))
            .collect()
    }
}
