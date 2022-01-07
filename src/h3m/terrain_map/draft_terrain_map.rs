use super::draft_map_cell::DraftMapCell;
use super::map_cell::MapCell;
use super::tile_code_generator::TileCodeGenerator;
use super::tile_type::TileType;
use crate::common::position::{Position, SignedDeltaPos};
use crate::h3m::{Surface, Terrain};
use rand::Rng;

fn gen_common_tile_type(terrain: Terrain) -> TileType {
    if matches!(terrain, Terrain::Water | Terrain::Rock) {
        return TileType::Common;
    }
    let number = rand::thread_rng().gen_range(0..5);
    if number == 0 {
        TileType::Pothole
    } else {
        TileType::Common
    }
}

fn neighborhood_area() -> [SignedDeltaPos; 8] {
    [
        SignedDeltaPos::new(-1, -1),
        SignedDeltaPos::new(-1, 0),
        SignedDeltaPos::new(-1, 1),
        SignedDeltaPos::new(0, -1),
        SignedDeltaPos::new(0, 1),
        SignedDeltaPos::new(1, -1),
        SignedDeltaPos::new(1, 0),
        SignedDeltaPos::new(1, 1),
    ]
}

pub struct DraftTerrainMap {
    size: usize,
    cells: Vec<Option<DraftMapCell>>,
    neighborhood_area: [SignedDeltaPos; 8],
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
            neighborhood_area: neighborhood_area(),
        }
    }

    pub fn set_tile_types(&mut self) {
        for cell in self.cells.iter_mut().flatten() {
            cell.tile.tile_type = Some(gen_common_tile_type(cell.surface.terrain));
        }
    }

    fn neighbour_tile_codes(&self, index: usize) -> Vec<u8> {
        let mut neighbour_tiles = Vec::new();

        let cell = match &self.cells[index] {
            Some(cell) => cell,
            None => return neighbour_tiles,
        };

        let terrain = cell.surface.terrain;
        let tile_type = cell.tile.tile_type.unwrap();

        for delta in &self.neighborhood_area {
            let neighbour_position = match cell.position.checked_apply(self.size, self.size, delta)
            {
                Some(neighbour_position) => neighbour_position,
                None => continue,
            };
            let neighbour_cell = match &self.cells[neighbour_position.index(self.size)] {
                Some(neighbour_cell) => neighbour_cell,
                None => continue,
            };
            if (terrain != neighbour_cell.surface.terrain)
                || (tile_type != neighbour_cell.tile.tile_type.unwrap())
            {
                continue;
            }
            let neighbour_tile_code = match neighbour_cell.tile.code {
                Some(neighbour_tile_code) => neighbour_tile_code,
                None => continue,
            };
            neighbour_tiles.push(neighbour_tile_code)
        }
        neighbour_tiles
    }

    pub fn set_tile_codes(&mut self) {
        let generator = TileCodeGenerator::new();
        for index in 0..self.size * self.size {
            let neighbour_tile_codes = self.neighbour_tile_codes(index);
            if let Some(cell) = &mut self.cells[index] {
                cell.tile.code = Some(generator.generate(
                    cell.surface.terrain,
                    cell.tile.tile_type.unwrap(),
                    &neighbour_tile_codes,
                ));
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
