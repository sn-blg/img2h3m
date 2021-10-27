use crate::h3m::enums::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Cursor, Read, Seek, SeekFrom};

fn skip_bytes<RS: Read + Seek>(input: &mut RS, count: u32) -> H3mResult<()> {
    input.seek(SeekFrom::Current(count as i64))?;
    Ok(())
}

fn read_bool<R: Read>(input: &mut R) -> H3mResult<bool> {
    let value = input.read_u8()?;
    match value {
        0x00 => Ok(false),
        0x01 => Ok(true),
        _ => Err(H3mError::ParseError),
    }
}

fn skip_string<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let size = input.read_u32::<LE>()?;
    skip_bytes(input, size)?;
    Ok(())
}

fn read_version<R: Read>(input: &mut R) -> H3mResult<Version> {
    let version = input.read_u32::<LE>()?;
    match version {
        0x0000000E => Ok(Version::RoE),
        0x00000015 => Ok(Version::AB),
        0x0000001C => Ok(Version::SoD),
        0x0000001D => Ok(Version::Chr),
        0x00000020 => Ok(Version::HotA),
        0x00000033 => Ok(Version::WoG),
        _ => Err(H3mError::ParseError),
    }
}

fn read_size<R: Read>(input: &mut R) -> H3mResult<Size> {
    let size = input.read_u32::<LE>()?;
    match size {
        36 => Ok(Size::S),
        72 => Ok(Size::M),
        108 => Ok(Size::L),
        144 => Ok(Size::XL),
        180 => Ok(Size::H),
        216 => Ok(Size::XH),
        252 => Ok(Size::G),
        _ => Err(H3mError::ParseError),
    }
}

fn skip_hota_additional_header_data<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let size = input.read_u32::<LE>()?;
    skip_bytes(input, size * 2)?;
    Ok(())
}

struct H3mHeaderInfo {
    map_size: Size,
    has_underground: bool,
}

fn read_header<RS: Read + Seek>(input: &mut RS) -> H3mResult<H3mHeaderInfo> {
    let version = read_version(input)?;
    if version != Version::HotA {
        return Err(H3mError::ParseError);
    }

    skip_hota_additional_header_data(input)?;
    skip_bytes(input, 1)?; // players_existence

    let map_size = read_size(input)?;
    let has_underground = read_bool(input)?;

    skip_string(input)?; // map name
    skip_string(input)?; // map description

    Ok(H3mHeaderInfo {
        map_size,
        has_underground,
    })
}

pub struct H3mInfo {
    pub map_size: Size,
    pub has_underground: bool,
}

pub fn parse(raw_map: &[u8]) -> H3mResult<H3mInfo> {
    let mut raw_map = Cursor::new(raw_map);

    let header_info = read_header(&mut raw_map)?;

    Ok(H3mInfo {
        map_size: header_info.map_size,
        has_underground: header_info.has_underground,
    })
}
