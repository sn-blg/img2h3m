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
}

#[derive(Clone, Copy)]
pub struct Surface {
    pub terrain: Terrain,
    pub obstacle: bool,
}
