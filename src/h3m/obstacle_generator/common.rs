use crate::h3m::Terrain;

impl Terrain {
    pub fn group(self) -> u16 {
        1 << (self.code() as u16)
    }
}
