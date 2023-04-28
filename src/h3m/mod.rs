use libflate::gzip::{Decoder, Encoder};
use obstacle_generator::ObstacleGenerator;
use parser::{H3mInfo, MAP_CELL_SIZE};
use result::*;
use std::io::{self, Read, Write};
pub use surface::{Surface, Terrain};
use terrain_map::{MapCell, TerrainMap};

mod obstacle_generator;
mod parser;
pub mod result;
mod surface;
mod terrain_map;

pub const MAX_MAP_SIZE: usize = 252;

pub struct H3m {
    info: H3mInfo,
    raw_map: Vec<u8>,
    obstacle_generator: Option<ObstacleGenerator>,
}

fn set_map_cell(map_cell: &MapCell, data: &mut [u8]) {
    data[0] = map_cell.surface().terrain.code();
    data[1] = map_cell.tile().code();

    let mut set_mirroring = |bit: u8, value| {
        if value {
            data[6] |= bit;
        } else {
            data[6] &= !(bit);
        }
    };

    const HORIZONTAL_MIRRORING_BIT: u8 = 0b0000_0001;
    const VERTICAL_MIRRORING_BIT: u8 = 0b0000_0010;

    set_mirroring(
        HORIZONTAL_MIRRORING_BIT,
        map_cell.tile().horizontal_mirroring(),
    );
    set_mirroring(VERTICAL_MIRRORING_BIT, map_cell.tile().vertical_mirroring());
}

impl H3m {
    pub fn load<R: io::Read>(input: R) -> H3mResult<H3m> {
        let mut decoder = Decoder::new(input)?;
        let mut raw_map = Vec::new();
        decoder.read_to_end(&mut raw_map)?;

        Ok(H3m {
            info: parser::parse(&raw_map)?,
            raw_map,
            obstacle_generator: None,
        })
    }

    pub fn save<W: io::Write>(&self, output: W) -> H3mResult<()> {
        let mut encoder = Encoder::new(output)?;

        if let Some(obstacle_generator) = &self.obstacle_generator {
            encoder.write_all(&self.raw_map[..self.info.objects_templates_offset])?;
            parser::write_object_templates(obstacle_generator.object_templates(), &mut encoder)?;
            parser::write_objects(obstacle_generator.objects(), &mut encoder)?;
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
        one_tile_water: bool,
        obstacles: bool,
        underground: bool,
        surfaces: &[Option<Surface>],
    ) -> H3mResult<()> {
        if obstacles && self.info.has_non_default_templates {
            return Err(H3mError::Parameter(ParameterError::new(
                "Unable to add obstacles: there are several objects on the input map.",
            )));
        }

        let terrain_map =
            TerrainMap::generate(self.map_size(), one_tile_water, underground, surfaces)?;

        for (index, map_cell) in terrain_map.cells().iter().enumerate() {
            if let Some(map_cell) = map_cell {
                self.set_map_cell_by_index(index, underground, map_cell)?;
            }
        }

        if obstacles {
            let obstacle_generator = self
                .obstacle_generator
                .get_or_insert_with(|| ObstacleGenerator::new(&self.info.default_object_templates));

            if terrain_map.has_obstacles() {
                obstacle_generator.generate(&terrain_map)?;
            }
        } else {
            assert!(!terrain_map.has_obstacles());
        }

        Ok(())
    }

    fn set_map_cell_by_index(
        &mut self,
        index: usize,
        underground: bool,
        map_cell: &MapCell,
    ) -> H3mResult<()> {
        let map_length = self.map_size() * self.map_size();
        if index >= map_length {
            return Err(H3mError::Parameter(ParameterError::new(format!(
                "Invalid map index: {}, map length: {}.",
                index, map_length
            ))));
        }

        let offset = if underground {
            self.info.underground_offset.ok_or_else(|| {
                H3mError::Parameter(ParameterError::new(
                    "Can't set underground map, input map has not underground.",
                ))
            })?
        } else {
            self.info.land_offset
        } + index * MAP_CELL_SIZE;

        let data = &mut self.raw_map[offset..offset + MAP_CELL_SIZE];
        set_map_cell(map_cell, data);

        Ok(())
    }
}
