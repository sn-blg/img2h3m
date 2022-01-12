use super::tile::Tile;
use crate::h3m::Surface;

pub struct MapCell {
    surface: Surface,
    tile: Tile,
}

impl MapCell {
    pub fn new(surface: Surface, tile: Tile) -> MapCell {
        MapCell { surface, tile }
    }

    pub fn surface(&self) -> Surface {
        self.surface
    }

    pub fn tile(&self) -> &Tile {
        &self.tile
    }
}
