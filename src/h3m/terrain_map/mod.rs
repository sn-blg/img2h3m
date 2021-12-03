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
    pub fn generate(size: usize, underground: bool, surfaces: &[Option<Surface>]) -> TerrainMap {
        TerrainMap {
            size,
            underground,
            has_obstacles: surfaces
                .iter()
                .map(|s| if let Some(s) = s { s.obstacle } else { false })
                .any(|obstacle| obstacle),
            cells: tiles_generator::generate(surfaces),
        }
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
