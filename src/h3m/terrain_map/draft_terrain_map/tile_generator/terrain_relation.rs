use super::common::NEIGHBORHOOD_SIZE;
use crate::h3m::Terrain;

#[derive(Clone, Copy, PartialEq)]
pub enum TerrainCategory {
    Sandy,
    Dirty,
}

impl Terrain {
    pub fn category(self) -> TerrainCategory {
        if matches!(self, Terrain::Sand | Terrain::Water | Terrain::Rock) {
            TerrainCategory::Sandy
        } else {
            TerrainCategory::Dirty
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TerrainRelation {
    SameNamed(&'static [&'static str]), // Some neighbour == central terrain and neighbour tail name == any name in [str]
    Eq,                                 // None or Some neighbour == central terrain
    EqOr(TerrainCategory), // None or Some neighbour == central terrain or Some neighbour in TerrainCategory
    Diff(TerrainCategory), // Some neighbour != central terrain and neighbour in TerrainCategory
    DiffAny,               // Some neighbour != central terrain
    Any,                   // any neighbour, including None
}

pub type NeighborhoodPattern = [TerrainRelation; NEIGHBORHOOD_SIZE];
