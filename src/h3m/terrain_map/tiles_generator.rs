use super::map_cell::{MapCell, Tile};
use crate::h3m::{Surface, Terrain};
use rand::Rng;

pub fn generate(surfaces: &[Option<Surface>]) -> Vec<Option<MapCell>> {
    surfaces
        .iter()
        .map(|surface| {
            surface
                .map(|surface| MapCell::new(surface, Tile::new(random_tile_code(surface.terrain))))
        })
        .collect()
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
