use libflate::gzip::{Decoder, Encoder};
use parsers::*;
use rand::Rng;
use result::*;
use std::io::{self, Read, Write};

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
    fn code(&self) -> u8 {
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

    pub fn rgb_color(&self) -> (u8, u8, u8) {
        match self {
            Surface::Dirt => (0x52, 0x39, 0x08),
            Surface::Sand => (0xDE, 0xCE, 0x8C),
            Surface::Grass => (0x00, 0x42, 0x00),
            Surface::Snow => (0xB5, 0xC6, 0xC6),
            Surface::Swamp => (0x4A, 0x84, 0x6B),
            Surface::Rough => (0x84, 0x73, 0x31),
            Surface::Subterranean => (0x84, 0x31, 0x00),
            Surface::Lava => (0x4A, 0x4A, 0x4A),
            Surface::Highland => (0x29, 0x73, 0x18),
            Surface::Wasteland => (0xBD, 0x5A, 0x08),
            Surface::Water => (0x08, 0x52, 0x94),
            Surface::Rock => (0x00, 0x00, 0x00),
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

    pub fn size(&self) -> usize {
        self.info.map_size
    }

    pub fn set_surface_by_index(
        &mut self,
        index: usize,
        underground: bool,
        surface: Surface,
    ) -> H3mResult<()> {
        const SURFACE_CELL_SIZE: usize = 7;

        let land_length = self.size() * self.size();

        if index >= land_length {
            return Err(H3mError::InvalidArgument);
        }

        let surface_cell_offset = self.info.land_offset
            + (if underground { land_length } else { 0 } + index) * SURFACE_CELL_SIZE;

        let surface_picture_type = self.surface_picture_type(surface);

        let surface_cell =
            &mut self.raw_map[surface_cell_offset..surface_cell_offset + SURFACE_CELL_SIZE];

        surface_cell[0] = surface.code();
        surface_cell[1] = surface_picture_type;

        Ok(())
    }

    pub fn set_surface(
        &mut self,
        row: usize,
        column: usize,
        underground: bool,
        surface: Surface,
    ) -> H3mResult<()> {
        self.set_surface_by_index(row * self.size() + column, underground, surface)
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
