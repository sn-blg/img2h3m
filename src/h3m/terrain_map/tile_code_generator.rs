use super::tile_index_set::TileIndexSet;
use super::tile_type::TileType;
use crate::h3m::Terrain;
use std::collections::HashMap;

pub struct TileCodeGenerator {
    tile_indexes: HashMap<Terrain, HashMap<TileType, TileIndexSet>>,
}

impl TileCodeGenerator {
    pub fn new() -> TileCodeGenerator {
        let tile_indexes = HashMap::from([
            (
                Terrain::Dirt,
                HashMap::from([(TileType::Common, TileIndexSet::new(21..=44, 100))]),
            ),
            (
                Terrain::Sand,
                HashMap::from([(TileType::Common, TileIndexSet::new(0..=23, 100))]),
            ),
            (
                Terrain::Grass,
                HashMap::from([(TileType::Common, TileIndexSet::new(49..=72, 100))]),
            ),
            (
                Terrain::Snow,
                HashMap::from([(TileType::Common, TileIndexSet::new(49..=72, 100))]),
            ),
            (
                Terrain::Swamp,
                HashMap::from([(TileType::Common, TileIndexSet::new(49..=72, 100))]),
            ),
            (
                Terrain::Rough,
                HashMap::from([(TileType::Common, TileIndexSet::new(49..=72, 100))]),
            ),
            (
                Terrain::Subterranean,
                HashMap::from([(TileType::Common, TileIndexSet::new(49..=72, 100))]),
            ),
            (
                Terrain::Lava,
                HashMap::from([(TileType::Common, TileIndexSet::new(49..=72, 100))]),
            ),
            (
                Terrain::Highland,
                HashMap::from([(TileType::Common, TileIndexSet::new(77..=117, 100))]),
            ),
            (
                Terrain::Wasteland,
                HashMap::from([(TileType::Common, TileIndexSet::new(77..=117, 100))]),
            ),
            (
                Terrain::Water,
                HashMap::from([(TileType::Common, TileIndexSet::new(21..=32, 100))]),
            ),
            (
                Terrain::Rock,
                HashMap::from([(TileType::Common, TileIndexSet::new(0..=7, 100))]),
            ),
        ]);

        TileCodeGenerator { tile_indexes }
    }

    pub fn generate(&self, terrain: Terrain, tile_type: TileType) -> u8 {
        self.tile_indexes[&terrain][&tile_type].random_index()
    }
}
