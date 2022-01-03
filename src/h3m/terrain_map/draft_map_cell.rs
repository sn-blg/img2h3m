use super::map_cell::{MapCell, Tile};
use super::tile_type::TileType;
use crate::common::position::Position;
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
    pub position: Position<usize>,
}

impl DraftMapCell {
    pub fn new(surface: Surface, position: Position<usize>) -> DraftMapCell {
        DraftMapCell {
            surface,
            tile: DraftTile::new(),
            position,
        }
    }

    pub fn to_map_cell(&self) -> MapCell {
        MapCell::new(self.surface, Tile::new(self.tile.code.unwrap()))
    }
}
