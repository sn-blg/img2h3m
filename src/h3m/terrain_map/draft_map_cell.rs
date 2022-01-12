use super::map_cell::MapCell;
use super::tile::Tile;
use crate::common::position::Position;
use crate::h3m::Surface;

#[derive(Clone, Copy)]
pub struct DraftMapCell {
    pub surface: Surface,
    pub tile: Option<Tile>,
    pub position: Position,
}

impl DraftMapCell {
    pub fn new(surface: Surface, position: Position) -> DraftMapCell {
        DraftMapCell {
            surface,
            tile: None,
            position,
        }
    }

    pub fn to_map_cell(&self) -> MapCell {
        MapCell::new(
            self.surface,
            self.tile.expect(&format!(
                "Invalid tile at row: {}, column: {}.",
                self.position.row(),
                self.position.column()
            )),
        )
    }
}
