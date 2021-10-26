use libflate::gzip::{Decoder, Encoder};
use result::H3mResult;
use std::io::{self, Read, Write};

pub mod result;

pub struct H3m {
    data: Vec<u8>,
}

impl H3m {
    pub fn load<R: io::Read>(input: R) -> H3mResult<H3m> {
        let mut decoder = Decoder::new(input)?;
        let mut data = Vec::new();
        decoder.read_to_end(&mut data)?;
        Ok(H3m { data })
    }

    pub fn save<W: io::Write>(&self, output: W) -> H3mResult<()> {
        let mut encoder = Encoder::new(output)?;
        encoder.write_all(&self.data)?;
        encoder.finish().into_result()?;
        Ok(())
    }
}
