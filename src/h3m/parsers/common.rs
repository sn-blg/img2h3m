use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek, SeekFrom};

pub fn skip_bytes<S: Seek>(input: &mut S, count: u32) -> H3mResult<()> {
    input.seek(SeekFrom::Current(count as i64))?;
    Ok(())
}

pub fn read_bool<RS: Read + Seek>(input: &mut RS) -> H3mResult<bool> {
    let value = input.read_u8()?;
    match value {
        0x00 => Ok(false),
        0x01 => Ok(true),
        other => Err(H3mError::Parsing(ParsingError::new(
            input.stream_position()?,
            format!("Invalid bool value 0x{:02x}.", other),
        ))),
    }
}

pub fn skip_bool<S: Seek>(input: &mut S) -> H3mResult<()> {
    skip_bytes(input, 1)?;
    Ok(())
}

pub fn skip_string<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let size = input.read_u32::<LE>()?;
    skip_bytes(input, size)?;
    Ok(())
}
