use crate::h3m::Terrain;

#[derive(Clone, Copy, PartialEq)]
pub enum TerrainVisibleType {
    Same,
    Diff(Terrain),
    None, // transitional tail
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    terrain_visible_type: TerrainVisibleType,
    code: u8,
    vertical_mirroring: bool,
    horizontal_mirroring: bool,
}

impl Tile {
    pub fn new(
        terrain_visible_type: TerrainVisibleType,
        code: u8,
        vertical_mirroring: bool,
        horizontal_mirroring: bool,
    ) -> Tile {
        Tile {
            terrain_visible_type,
            code,
            vertical_mirroring,
            horizontal_mirroring,
        }
    }

    pub fn _terrain_visible_type(&self) -> TerrainVisibleType {
        self.terrain_visible_type
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn vertical_mirroring(&self) -> bool {
        self.vertical_mirroring
    }

    pub fn horizontal_mirroring(&self) -> bool {
        self.horizontal_mirroring
    }
}
