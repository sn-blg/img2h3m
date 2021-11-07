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

    pub fn rgb_color(&self) -> [u8; 3] {
        match *self {
            Surface::Dirt => [0x52, 0x39, 0x08],
            Surface::Sand => [0xDE, 0xCE, 0x8C],
            Surface::Grass => [0x00, 0x42, 0x00],
            Surface::Snow => [0xB5, 0xC6, 0xC6],
            Surface::Swamp => [0x4A, 0x84, 0x6B],
            Surface::Rough => [0x84, 0x73, 0x31],
            Surface::Subterranean => [0x84, 0x31, 0x00],
            Surface::Lava => [0x4A, 0x4A, 0x4A],
            Surface::Highland => [0x29, 0x73, 0x18],
            Surface::Wasteland => [0xBD, 0x5A, 0x08],
            Surface::Water => [0x08, 0x52, 0x94],
            Surface::Rock => [0x00, 0x00, 0x00],
        }
    }

    pub fn is_ground(&self) -> bool {
        !matches!(*self, Surface::Water | Surface::Rock)
    }
}
