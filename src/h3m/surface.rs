#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terrain {
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

impl Terrain {
    pub fn is_ground(&self) -> bool {
        !matches!(*self, Terrain::Water | Terrain::Rock)
    }

    pub fn code(self) -> u8 {
        match self {
            Terrain::Dirt => 0,
            Terrain::Sand => 1,
            Terrain::Grass => 2,
            Terrain::Snow => 3,
            Terrain::Swamp => 4,
            Terrain::Rough => 5,
            Terrain::Subterranean => 6,
            Terrain::Lava => 7,
            Terrain::Highland => 10,
            Terrain::Wasteland => 11,
            Terrain::Water => 8,
            Terrain::Rock => 9,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Surface {
    pub terrain: Terrain,
    pub obstacle: bool,
}
