use crate::h3m::Terrain;

pub type TerrainVisibleType = Option<Terrain>; // None if transitional tail

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

    pub fn terrain_visible_type(&self) -> TerrainVisibleType {
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

    pub fn set_vertical_mirroring(&mut self, value: bool) {
        self.vertical_mirroring = value;
    }

    pub fn set_horizontal_mirroring(&mut self, value: bool) {
        self.horizontal_mirroring = value;
    }
}
