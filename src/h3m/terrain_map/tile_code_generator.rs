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
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(21..=28, 7).with_tiles(29..=44, 1),
                )]),
            ),
            (
                Terrain::Sand,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(0..=7, 7).with_tiles(8..=23, 1),
                )]),
            ),
            (
                Terrain::Grass,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Snow,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Swamp,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Rough,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Subterranean,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Lava,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Highland,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(77..=101, 5).with_tiles(102..=117, 2),
                )]),
            ),
            (
                Terrain::Wasteland,
                HashMap::from([(
                    TileType::Common,
                    TileIndexSet::new(77..=101, 5).with_tiles(102..=117, 2),
                )]),
            ),
            (
                Terrain::Water,
                HashMap::from([(TileType::Common, TileIndexSet::new(21..=32, 1))]),
            ),
            (
                Terrain::Rock,
                HashMap::from([(TileType::Common, TileIndexSet::new(0..=7, 1))]),
            ),
        ]);

        TileCodeGenerator { tile_indexes }
    }

    pub fn generate(&self, terrain: Terrain, tile_type: TileType) -> u8 {
        self.tile_indexes[&terrain][&tile_type].random_index()
    }
}
