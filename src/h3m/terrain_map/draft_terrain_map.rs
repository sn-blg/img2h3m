use super::draft_map_cell::DraftMapCell;
use super::map_cell::MapCell;
use super::tile_code_generator::TileCodeGenerator;
use super::tile_type::TileType;
use crate::h3m::Surface;

pub struct DraftTerrainMap {
    size: usize,
    cells: Vec<Option<DraftMapCell>>,
}

impl DraftTerrainMap {
    pub fn new(size: usize, surfaces: &[Option<Surface>]) -> DraftTerrainMap {
        DraftTerrainMap {
            size,
            cells: surfaces.iter().map(|s| s.map(DraftMapCell::new)).collect(),
        }
    }

    pub fn set_tile_types(&mut self) {
        for cell in self.cells.iter_mut().flatten() {
            cell.tile.tile_type = Some(TileType::Common);
        }
    }

    pub fn set_tile_codes(&mut self) {
        let generator = TileCodeGenerator::new();

        let tile_codes: Vec<Option<u8>> = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                if let Some(cell) = cell {
                    Some(generator.generate(
                        cell.surface.terrain,
                        cell.tile.tile_type.unwrap(),
                        &[],
                    ))
                } else {
                    None
                }
            })
            .collect();

        for (cell, tile_code) in self.cells.iter_mut().zip(tile_codes) {
            if let Some(cell) = cell {
                cell.tile.code = tile_code;
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
