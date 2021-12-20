use super::tile_codes_set::TileCodesSet;
use super::tile_type::TileType;
use crate::h3m::Terrain;
use std::collections::HashMap;

pub struct TileCodeGenerator {
    tile_codes: HashMap<Terrain, HashMap<TileType, TileCodesSet>>,
}

impl TileCodeGenerator {
    pub fn new() -> TileCodeGenerator {
        let tile_codes = HashMap::from([
            (
                Terrain::Dirt,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(21..=28, 7).with_tiles(29..=44, 1),
                )]),
            ),
            (
                Terrain::Sand,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(0..=7, 7).with_tiles(8..=23, 1),
                )]),
            ),
            (
                Terrain::Grass,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Snow,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Swamp,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Rough,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Subterranean,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Lava,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(49..=56, 7).with_tiles(57..=72, 1),
                )]),
            ),
            (
                Terrain::Highland,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(77..=101, 5).with_tiles(102..=117, 2),
                )]),
            ),
            (
                Terrain::Wasteland,
                HashMap::from([(
                    TileType::Common,
                    TileCodesSet::new(77..=101, 5).with_tiles(102..=117, 2),
                )]),
            ),
            (
                Terrain::Water,
                HashMap::from([(TileType::Common, TileCodesSet::new(21..=32, 1))]),
            ),
            (
                Terrain::Rock,
                HashMap::from([(TileType::Common, TileCodesSet::new(0..=7, 1))]),
            ),
        ]);

        TileCodeGenerator { tile_codes }
    }

    pub fn generate(
        &self,
        terrain: Terrain,
        tile_type: TileType,
        excluded_tile_codes: &[u8],
    ) -> u8 {
        let mut tile_codes_set = self.tile_codes[&terrain][&tile_type].clone();

        for &code in excluded_tile_codes {
            tile_codes_set.remove(code);
        }

        tile_codes_set.random_code()
    }
}
