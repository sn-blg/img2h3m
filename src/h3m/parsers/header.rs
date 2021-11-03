use crate::h3m::parsers::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

#[derive(PartialEq)]
enum Version {
    RoE,
    AB,
    SoD,
    Chr,
    WoG,
    HotA,
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

fn read_size<R: Read>(input: &mut R) -> H3mResult<usize> {
    let size = input.read_u32::<LE>()?;
    usize::try_from(size).or(Err(H3mError::ParseError))
}

pub struct H3mHeaderInfo {
    pub map_size: usize,
    pub has_underground: bool,
}

pub fn read_header<RS: Read + Seek>(input: &mut RS) -> H3mResult<H3mHeaderInfo> {
    let version = read_version(input)?;
    if version != Version::HotA {
        return Err(H3mError::ParseError);
    }

    let hota_additional_header_data_size = input.read_u32::<LE>()?;
    if hota_additional_header_data_size != 3 {
        return Err(H3mError::ParseError);
    }

    skip_bytes(input, hota_additional_header_data_size * 2)?;

    skip_bool(input)?; // players_existence

    let map_size = read_size(input)?;
    let has_underground = read_bool(input)?;

    skip_string(input)?; // map name
    skip_string(input)?; // map description

    skip_bytes(input, 1)?; // map difficulty
    skip_bytes(input, 1)?; // level cap

    Ok(H3mHeaderInfo {
        map_size,
        has_underground,
    })
}
