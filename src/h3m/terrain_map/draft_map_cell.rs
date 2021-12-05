use super::map_cell::{MapCell, Tile};
use super::tile_type::TileType;
use crate::h3m::Surface;

pub struct DraftTile {
    pub code: Option<u8>,
    pub tile_type: Option<TileType>,
}

impl DraftTile {
    fn new() -> DraftTile {
        DraftTile {
            code: None,
            tile_type: None,
        }
    }
}

pub struct DraftMapCell {
    pub surface: Surface,
    pub tile: DraftTile,
}

impl DraftMapCell {
    pub fn new(surface: Surface) -> DraftMapCell {
        DraftMapCell {
            surface,
            tile: DraftTile::new(),
        }
    }

    pub fn to_map_cell(&self) -> MapCell {
        MapCell::new(self.surface, Tile::new(self.tile.code.unwrap()))
    }
}
