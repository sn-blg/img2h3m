use crate::h3m::result::*;
use crate::h3m::Surface;
use draft_map_cell::DraftMapCell;
pub use map_cell::MapCell;
use tile_code_generator::TileCodeGenerator;
use tile_type::TileType;

mod draft_map_cell;
mod map_cell;
mod tile_code_generator;
mod tile_codes_set;
mod tile_type;

pub struct TerrainMap {
    size: usize,
    underground: bool,
    has_obstacles: bool,
    cells: Vec<Option<MapCell>>,
}

impl TerrainMap {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn underground(&self) -> bool {
        self.underground
    }

    pub fn has_obstacles(&self) -> bool {
        self.has_obstacles
    }

    pub fn cells(&self) -> &[Option<MapCell>] {
        &self.cells
    }

    pub fn generate(
        size: usize,
        underground: bool,
        surfaces: &[Option<Surface>],
    ) -> H3mResult<TerrainMap> {
        let map_len = size * size;
        if surfaces.len() != map_len {
            return Err(H3mError::Parameter(ParameterError::new(format!(
                "surfaces length ({}) not equal map length ({}).",
                surfaces.len(),
                map_len
            ))));
        }

        let mut draft_map_cells = make_draft_map_cells(surfaces);
        set_tile_types(&mut draft_map_cells);
        set_tile_codes(&mut draft_map_cells);

        Ok(TerrainMap {
            size,
            underground,
            has_obstacles: surfaces
                .iter()
                .map(|s| if let Some(s) = s { s.obstacle } else { false })
                .any(|obstacle| obstacle),
            cells: draft_map_cells
                .into_iter()
                .map(|cell| cell.map(|cell| cell.to_map_cell()))
                .collect(),
        })
    }
}

fn make_draft_map_cells(surfaces: &[Option<Surface>]) -> Vec<Option<DraftMapCell>> {
    surfaces.iter().map(|s| s.map(DraftMapCell::new)).collect()
}

fn set_tile_types(draft_map_cells: &mut [Option<DraftMapCell>]) {
    for cell in draft_map_cells.iter_mut().flatten() {
        cell.tile.tile_type = Some(TileType::Common);
    }
}

fn set_tile_codes(draft_map_cells: &mut [Option<DraftMapCell>]) {
    let generator = TileCodeGenerator::new();
    for cell in draft_map_cells.iter_mut().flatten() {
        cell.tile.code =
            Some(generator.generate(cell.surface.terrain, cell.tile.tile_type.unwrap(), &[]));
    }
}
