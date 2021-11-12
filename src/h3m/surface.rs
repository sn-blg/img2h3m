#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Surface {
    Dirt,
    Sand,
    Grass,
    Snow,
    Swamp,
    Rough,
    Subterranean,
    Lava,
    Highland,
    Wasteland,
    Water,
    Rock,
}

impl Surface {
    pub fn code(&self) -> u8 {
        match *self {
            Surface::Dirt => 0,
            Surface::Sand => 1,
            Surface::Grass => 2,
            Surface::Snow => 3,
            Surface::Swamp => 4,
            Surface::Rough => 5,
            Surface::Subterranean => 6,
            Surface::Lava => 7,
            Surface::Highland => 10,
            Surface::Wasteland => 11,
            Surface::Water => 8,
            Surface::Rock => 9,
        }
    }

    pub fn is_ground(&self) -> bool {
        !matches!(*self, Surface::Water | Surface::Rock)
    }
}
