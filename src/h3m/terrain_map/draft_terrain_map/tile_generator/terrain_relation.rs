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
    Eq,                    // None or Some neighbour == central terrain
    Diff(TerrainCategory), // Some neighbour != central terrain and neighbour in TerrainСategory
    DiffAny,               // Some neighbour != central terrain
    Any,                   // any neighbour, including None
}

pub const NEIGHBORHOOD_SIZE: usize = 8;

pub type NeighborhoodPattern = [TerrainRelation; NEIGHBORHOOD_SIZE];