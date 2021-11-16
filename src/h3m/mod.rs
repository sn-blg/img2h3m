use libflate::gzip::{Decoder, Encoder};
use obstacle_generator::ObstacleGenerator;
use parsers::*;
use rand::Rng;
use result::*;
use std::io::{self, Read, Write};
pub use surface::{Surface, Terrain};

mod obstacle_generator;
mod parsers;
pub mod result;
mod surface;

pub struct H3m {
    info: H3mInfo,
    raw_map: Vec<u8>,
    obstacle_generator: Option<ObstacleGenerator>,
}

impl H3m {
    pub fn load<R: io::Read>(input: R) -> H3mResult<H3m> {
        let mut decoder = Decoder::new(input)?;
        let mut raw_map = Vec::new();
        decoder.read_to_end(&mut raw_map)?;

        Ok(H3m {
            info: parse(&raw_map)?,
            raw_map,
            obstacle_generator: None,
        })
    }

    pub fn save<W: io::Write>(&self, output: W) -> H3mResult<()> {
        let mut encoder = Encoder::new(output)?;

        if let Some(obstacle_generator) = &self.obstacle_generator {
            encoder.write_all(&self.raw_map[..self.info.objects_templates_offset])?;
            write_object_templates(obstacle_generator.object_templates(), &mut encoder)?;
            write_objects(obstacle_generator.objects(), &mut encoder)?;
            encoder.write_all(&[0u8; 124])?;
        } else {
            encoder.write_all(&self.raw_map)?;
        }

        encoder.finish().into_result()?;
        Ok(())
    }

    pub fn map_size(&self) -> usize {
        self.info.map_size
    }

    pub fn set_surfaces(
        &mut self,
        underground: bool,
        surfaces: &[Option<Surface>],
    ) -> H3mResult<()> {
        let mut has_obstacles = false;

        for (index, surface) in surfaces.iter().enumerate() {
            if let Some(surface) = surface {
                self.set_terrain_by_index(index, underground, surface.terrain)?;
                if surface.obstacle {
                    has_obstacles = true;
                }
            }
        }

        if has_obstacles {
            let map_size = self.map_size();
            self.obstacle_generator
                .get_or_insert_with(|| {
                    ObstacleGenerator::new(map_size, &self.info.default_object_templates)
                })
                .generate(underground, surfaces)?;
        }

        Ok(())
    }

    fn set_terrain_by_index(
        &mut self,
        index: usize,
        underground: bool,
        terrain: Terrain,
    ) -> H3mResult<()> {
        let map_length = self.map_size() * self.map_size();
        if index >= map_length {
            return Err(H3mError::Parameter(ParameterError::new(format!(
                "Invalid map index: {}, map length: {}.",
                index, map_length
            ))));
        }

        let map_cell_offset = if underground {
            self.info.underground_offset.ok_or_else(|| {
                H3mError::Parameter(ParameterError::new(
                    "Can't set underground map, input map has not underground.",
                ))
            })?
        } else {
            self.info.land_offset
        } + index * MAP_CELL_SIZE;

        let terrain_picture_type = self.terrain_picture_type(terrain);

        let map_cell = &mut self.raw_map[map_cell_offset..map_cell_offset + MAP_CELL_SIZE];

        map_cell[0] = terrain_code(terrain);
        map_cell[1] = terrain_picture_type;

        Ok(())
    }

    fn terrain_picture_type(&self, terrain: Terrain) -> u8 {
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
}

fn terrain_code(terrain: Terrain) -> u8 {
    match terrain {
        Terrain::Dirt => 0,
        Terrain::Sand => 1,
        Terrain::Grass => 2,
        Terrain::Snow => 3,
        Terrain::Swamp => 4,
        Terrain::Rough => 5,
        Terrain::Subterranean => 6,
        Terrain::Lava => 7,
        Terrain::Highland => 10,
        Terrain::Wasteland => 11,
        Terrain::Water => 8,
        Terrain::Rock => 9,
    }
}
