use libflate::gzip::{Decoder, Encoder};
use parsers::*;
use rand::Rng;
use result::*;
use std::io::{self, Read, Write};
pub use surface::Surface;

mod parsers;
pub mod result;
mod surface;

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

    pub fn map_size(&self) -> usize {
        self.info.map_size
    }

    pub fn set_surfaces(
        &mut self,
        underground: bool,
        surfaces: &[Option<Surface>],
    ) -> H3mResult<()> {
        for (index, surface) in surfaces.iter().enumerate() {
            if let Some(surface) = surface {
                self.set_surface_by_index(index, underground, *surface)?;
            }
        }
        Ok(())
    }

    fn set_surface_by_index(
        &mut self,
        index: usize,
        underground: bool,
        surface: Surface,
    ) -> H3mResult<()> {
        let land_length = self.map_size() * self.map_size();

        if index >= land_length {
            return Err(H3mError::Internal(InternalError::new(format!(
                "Invalid surface index: {}, land length: {}.",
                index, land_length
            ))));
        }

        let surface_cell_offset = if underground {
            self.info.underground_offset.ok_or_else(|| {
                H3mError::Parameter(ParameterError::new(
                    "Can't set underground surface, input map has not underground.",
                ))
            })?
        } else {
            self.info.land_offset
        } + index * SURFACE_CELL_SIZE;

        let surface_picture_type = self.surface_picture_type(surface);

        let surface_cell =
            &mut self.raw_map[surface_cell_offset..surface_cell_offset + SURFACE_CELL_SIZE];

        surface_cell[0] = surface.code();
        surface_cell[1] = surface_picture_type;

        Ok(())
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
