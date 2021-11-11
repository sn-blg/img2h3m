use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn skip_bytes<S: Seek>(input: &mut S, count: u32) -> H3mResult<()> {
    let count = i64::try_from(count).map_err(|_| {
        H3mError::Internal(InternalError::new(format!(
            "Can't convert bytes count {} to i64.",
            count
        )))
    })?;

    input.seek(SeekFrom::Current(count))?;
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

pub fn position(cursor: &Cursor<&[u8]>) -> H3mResult<usize> {
    let position = cursor.position();
    usize::try_from(position).map_err(|_| {
        H3mError::Internal(InternalError::new(format!(
            "Can't convert position value {} to usize.",
            position
        )))
    })
}
