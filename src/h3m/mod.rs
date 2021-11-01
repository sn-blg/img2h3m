pub use enums::Size;
use libflate::gzip::{Decoder, Encoder};
use parsers::*;
use rand::Rng;
use result::*;
use std::io::{self, Read, Write};

mod enums;
mod parsers;
pub mod result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Surface {
    Dirt,
    Sand,
    Grass,
    Snow,
    Swamp,
    Rough,
    Subterranean,
    Lava,
    Highland,
    Wasteland,
    Water,
    Rock,
}

impl Surface {
    fn to_code(&self) -> u8 {
        match self {
            Surface::Dirt => 0,
            Surface::Sand => 1,
            Surface::Grass => 2,
            Surface::Snow => 3,
            Surface::Swamp => 4,
            Surface::Rough => 5,
            Surface::Subterranean => 6,
            Surface::Lava => 7,
            Surface::Highland => 10,
            Surface::Wasteland => 11,
            Surface::Water => 8,
            Surface::Rock => 9,
        }
    }
}

pub struct H3m {
    info: H3mInfo,
    raw_map: Vec<u8>,
}

impl H3m {
    pub fn load<R: io::Read>(input: R) -> H3mResult<H3m> {
        let mut decoder = Decoder::new(input)?;
        let mut raw_map = Vec::new();
        decoder.read_to_end(&mut raw_map)?;

        Ok(H3m {
            info: parse(&raw_map)?,
            raw_map,
        })
    }

    pub fn save<W: io::Write>(&self, output: W) -> H3mResult<()> {
        let mut encoder = Encoder::new(output)?;
        encoder.write_all(&self.raw_map)?;
        encoder.finish().into_result()?;
        Ok(())
    }

    pub fn size(&self) -> Size {
        self.info.map_size
    }

    pub fn set_land(&mut self, index: usize, surface: Surface) -> H3mResult<()> {
        if index >= self.land_length() {
            return Err(H3mError::InvalidArgument);
        }

        let surface_cell_offset = self.land_offset() + index * 7;
        let surface_picture_type = self.surface_picture_type(surface);

        let surface_cell = &mut self.raw_map[surface_cell_offset..surface_cell_offset + 7];

        surface_cell[0] = surface.to_code();
        surface_cell[1] = surface_picture_type;

        Ok(())
    }

    fn land_length(&self) -> usize {
        let map_size = self.info.map_size as usize;
        map_size * map_size
    }

    fn land_offset(&self) -> usize {
        self.info.land_offset
    }

    fn surface_picture_type(&self, surface: Surface) -> u8 {
        let range = match surface {
            Surface::Dirt => 21..=44,
            Surface::Sand => 0..=23,
            Surface::Grass => 49..=72,
            Surface::Snow => 49..=72,
            Surface::Swamp => 49..=72,
            Surface::Rough => 49..=72,
            Surface::Subterranean => 49..=72,
            Surface::Lava => 49..=72,
            Surface::Highland => 77..=117,
            Surface::Wasteland => 77..=117,
            Surface::Water => 21..=32,
            Surface::Rock => 0..=7,
        };
        rand::thread_rng().gen_range(range)
    }
}
