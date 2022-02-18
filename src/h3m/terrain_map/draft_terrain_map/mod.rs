use crate::common::position::{Position, SignedDeltaPos};
use crate::h3m::{terrain_map::map_cell::MapCell, Surface, MAX_MAP_SIZE};
use draft_map_cell::DraftMapCell;
use num::Integer;
use tile_generator::{Neighborhood, TileGeneratingMode, TileGenerator};

mod draft_map_cell;
mod tile_generator;

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

    fn neighborhood(&self, index: usize) -> Neighborhood {
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

    pub fn set_tile_codes(&mut self, one_tile_water: bool) {
        let mut generator = TileGenerator::new(one_tile_water);
        for mode in [TileGeneratingMode::Main, TileGeneratingMode::Fallback] {
            let is_done =
                self.set_tile_codes_iterations_with_mode(&mut generator, mode, MAX_MAP_SIZE);
            if !is_done {
                panic!();
            }
        }
    }

    fn set_tile_codes_iterations_with_mode(
        &mut self,
        generator: &mut TileGenerator,
        mode: TileGeneratingMode,
        max_iter_count: usize,
    ) -> bool {
        for iter_index in 0..max_iter_count {
            let was_changed = self.set_tile_codes_iteration(generator, mode, iter_index.is_odd());
            if !was_changed {
                return true;
            }
        }
        false
    }

    fn set_tile_codes_iteration(
        &mut self,
        generator: &mut TileGenerator,
        mode: TileGeneratingMode,
        backward_direction: bool,
    ) -> bool {
        let map_len = self.size * self.size;
        let mut was_changed = false;

        let mut try_change_tile = |index| {
            let neighborhood = self.neighborhood(index);
            if let Some(cell) = &mut self.cells[index] {
                let tile = generator.try_generate_tile(cell, &neighborhood, mode);
                if tile != cell.tile {
                    was_changed = true
                }
                cell.tile = tile;
            }
        };

        if backward_direction {
            for index in (0..map_len).rev() {
                try_change_tile(index);
            }
        } else {
            for index in 0..map_len {
                try_change_tile(index);
            }
        }

        was_changed
    }

    pub fn into_map_cells(self) -> Vec<Option<MapCell>> {
        self.cells
            .into_iter()
            .map(|cell| cell.map(|cell| cell.to_map_cell()))
            .collect()
    }
}
