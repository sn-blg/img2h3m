use crate::h3m::result::*;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Terrain {
    Dirt,
    Sand,
    Grass,
    Snow,
    Swamp,
    Rough,
    Subterranean,
    Lava,
    Highlands,
    Wasteland,
    Water,
    Rock,
}

impl Terrain {
    pub fn from_code(code: u8) -> H3mResult<Terrain> {
        match code {
            0 => Ok(Terrain::Dirt),
            1 => Ok(Terrain::Sand),
            2 => Ok(Terrain::Grass),
            3 => Ok(Terrain::Snow),
            4 => Ok(Terrain::Swamp),
            5 => Ok(Terrain::Rough),
            6 => Ok(Terrain::Subterranean),
            7 => Ok(Terrain::Lava),
            8 => Ok(Terrain::Water),
            9 => Ok(Terrain::Rock),
            10 => Ok(Terrain::Highlands),
            11 => Ok(Terrain::Wasteland),
            _ => Err(H3mError::Internal(InternalError::new(format!(
                "Can't convert code {} to Terrain type.",
                code
            )))),
        }
    }

    pub fn is_ground(self) -> bool {
        !matches!(self, Terrain::Water | Terrain::Rock)
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
            Terrain::Highlands => 10,
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
