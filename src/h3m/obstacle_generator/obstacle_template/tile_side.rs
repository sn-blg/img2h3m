use crate::h3m::terrain_map::{TerrainVisibleType, Tile, TileType};
use crate::h3m::Terrain;

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CornerSide {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

impl Tile {
    pub fn mixed_only_with(&self, terrain: Terrain) -> bool {
        matches!(
            self.terrain_visible_type(),
            TerrainVisibleType::Mixed(t) | TerrainVisibleType::DiffMixed(_, t)
            if t == terrain
        )
    }

    pub fn is_scrap(&self) -> bool {
        self.tile_type() == TileType::Scrap
    }

    pub fn is_scrap_on_corner(&self, corner_side: CornerSide) -> bool {
        if !self.is_scrap() {
            return false;
        }

        match corner_side {
            CornerSide::TopLeft => !self.horizontal_mirroring() && !self.vertical_mirroring(),
            CornerSide::BottomLeft => !self.horizontal_mirroring() && self.vertical_mirroring(),
            CornerSide::TopRight => self.horizontal_mirroring() && !self.vertical_mirroring(),
            CornerSide::BottomRight => self.horizontal_mirroring() && self.vertical_mirroring(),
        }
    }

    pub fn is_scrap_on(&self, side: Side) -> bool {
        if !self.is_scrap() {
            return false;
        }

        match side {
            Side::Top => !self.vertical_mirroring(),
            Side::Bottom => self.vertical_mirroring(),
            Side::Left => !self.horizontal_mirroring(),
            Side::Right => self.horizontal_mirroring(),
        }
    }
}
