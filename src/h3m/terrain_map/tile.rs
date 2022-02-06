use crate::h3m::Terrain;

#[derive(Clone, Copy, PartialEq)]
pub enum TileComposition {
    Main,
    Fallback,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TerrainVisibleType {
    Same,
    Diff(Terrain),
    None, // transitional tail
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    composition: TileComposition,
    name: &'static str,
    terrain_visible_type: TerrainVisibleType,
    code: u8,
    vertical_mirroring: bool,
    horizontal_mirroring: bool,
}

impl Tile {
    pub fn new(
        composition: TileComposition,
        name: &'static str,
        terrain_visible_type: TerrainVisibleType,
        code: u8,
        vertical_mirroring: bool,
        horizontal_mirroring: bool,
    ) -> Tile {
        Tile {
            composition,
            name,
            terrain_visible_type,
            code,
            vertical_mirroring,
            horizontal_mirroring,
        }
    }

    pub fn composition(&self) -> TileComposition {
        self.composition
    }

    pub fn name(&self) -> &'static str {
        self.name
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

impl Default for Tile {
    fn default() -> Self {
        Tile {
            composition: TileComposition::Main,
            name: "",
            terrain_visible_type: TerrainVisibleType::None,
            code: 0,
            vertical_mirroring: false,
            horizontal_mirroring: false,
        }
    }
}
