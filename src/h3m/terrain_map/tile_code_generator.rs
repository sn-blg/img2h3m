use super::tile_type::TileType;
use crate::h3m::Terrain;
use rand::Rng;

pub struct TileCodeGenerator {}

impl TileCodeGenerator {
    pub fn new() -> TileCodeGenerator {
        TileCodeGenerator {}
    }

    pub fn generate(&self, terrain: Terrain, tile_type: TileType) -> u8 {
        assert_eq!(tile_type, TileType::Common);
        random_tile_code(terrain)
    }
}

fn random_tile_code(terrain: Terrain) -> u8 {
    let range = match terrain {
        Terrain::Dirt => 21..=44,
        Terrain::Sand => 0..=23,
        Terrain::Grass => 49..=72,
        Terrain::Snow => 49..=72,
        Terrain::Swamp => 49..=72,
        Terrain::Rough => 49..=72,
        Terrain::Subterranean => 49..=72,
        Terrain::Lava => 49..=72,
        Terrain::Highland => 77..=117,
        Terrain::Wasteland => 77..=117,
        Terrain::Water => 21..=32,
        Terrain::Rock => 0..=7,
    };
    rand::thread_rng().gen_range(range)
}
