use crate::h3m::Terrain;

#[derive(Clone, Copy)]
pub struct Tile {
    terrain: Terrain,
    code: u8,
    vertical_mirroring: bool,
    horizontal_mirroring: bool,
}

impl Tile {
    pub fn new(terrain: Terrain, code: u8, vertical_mirroring: bool, horizontal_mirroring: bool) -> Tile {
        Tile {
            terrain,
            code,
            vertical_mirroring,
            horizontal_mirroring,
        }
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
