use crate::h3m::result::*;
use crate::h3m::Surface;
use draft_terrain_map::DraftTerrainMap;
pub use map_cell::MapCell;

mod draft_map_cell;
mod draft_terrain_map;
mod map_cell;
mod tile;
mod tile_code_generator;
mod tile_codes_set;

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

        let mut draft_terrain_map = DraftTerrainMap::new(size, surfaces);
        draft_terrain_map.set_tile_codes();

        Ok(TerrainMap {
            size,
            underground,
            has_obstacles: surfaces
                .iter()
                .map(|s| if let Some(s) = s { s.obstacle } else { false })
                .any(|obstacle| obstacle),
            cells: draft_terrain_map.into_map_cells(),
        })
    }
}
