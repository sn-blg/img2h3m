use crate::h3m::result::*;
use crate::h3m::Surface;
pub use map_cell::MapCell;

mod map_cell;
mod tiles_generator;

pub struct TerrainMap {
    size: usize,
    underground: bool,
    has_obstacles: bool,
    cells: Vec<Option<MapCell>>,
}

impl TerrainMap {
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

        Ok(TerrainMap {
            size,
            underground,
            has_obstacles: surfaces
                .iter()
                .map(|s| if let Some(s) = s { s.obstacle } else { false })
                .any(|obstacle| obstacle),
            cells: tiles_generator::generate(surfaces),
        })
    }

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
}
