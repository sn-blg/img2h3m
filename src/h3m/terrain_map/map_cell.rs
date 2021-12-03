use crate::h3m::Surface;

pub struct Tile {
    code: u8,
}

impl Tile {
    pub fn new(code: u8) -> Tile {
        Tile { code }
    }

    pub fn code(&self) -> u8 {
        self.code
    }
}

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
