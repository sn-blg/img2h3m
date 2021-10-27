pub use enums::Size;
use libflate::gzip::{Decoder, Encoder};
use parsers::*;
use result::H3mResult;
use std::io::{self, Read, Write};

mod enums;
mod parsers;
pub mod result;

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
}
