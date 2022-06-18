use crate::h3m::Terrain;

#[derive(Clone, Copy, PartialEq)]
pub enum TerrainVisibleType {
    Same,
    Diff(Terrain),
    Mixed,
    DiffMixed(Terrain),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    HalfDiff(Orientation, Terrain),
    HalfDiff2(Orientation, Terrain, Terrain),
    Solid,
    Undefined,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    terrain_visible_type: TerrainVisibleType,
    tile_type: TileType,
    code: u8,
    vertical_mirroring: bool,
    horizontal_mirroring: bool,
}

impl Tile {
    pub fn new(
        terrain_visible_type: TerrainVisibleType,
        tile_type: TileType,
        code: u8,
        vertical_mirroring: bool,
        horizontal_mirroring: bool,
    ) -> Tile {
        assert!(
            if tile_type == TileType::Solid {
                matches!(
                    terrain_visible_type,
                    TerrainVisibleType::Same | TerrainVisibleType::Diff(_)
                )
            } else {
                matches!(
                    terrain_visible_type,
                    TerrainVisibleType::Mixed | TerrainVisibleType::DiffMixed(_)
                )
            },
            "invalid tile type, tile code: {}",
            code
        );

        Tile {
            terrain_visible_type,
            tile_type,
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
}
