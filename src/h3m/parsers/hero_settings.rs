use crate::h3m::parsers::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

fn skip_hero<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    skip_bytes(input, 1)?;
    Ok(())
}

pub fn skip_hero_settings<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let heroes_count = input.read_u32::<LE>()?;
    for _ in 0..heroes_count {
        skip_hero(input)?;
    }
    Ok(())
}
