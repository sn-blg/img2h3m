use crate::h3m::parser::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq)]
enum Version {
    RoE,
    AB,
    SoD,
    Chr,
    WoG,
    HotA,
}

fn read_version<RS: Read + Seek>(input: &mut RS) -> H3mResult<Version> {
    let version = input.read_u32::<LE>()?;
    match version {
        0x0000000E => Ok(Version::RoE),
        0x00000015 => Ok(Version::AB),
        0x0000001C => Ok(Version::SoD),
        0x0000001D => Ok(Version::Chr),
        0x00000020 => Ok(Version::HotA),
        0x00000033 => Ok(Version::WoG),
        other => Err(H3mError::Parsing(ParsingError::new(
            input.stream_position()?,
            format!("Unexpected h3m map version value 0x{:08x}.", other),
        ))),
    }
}

fn read_size<RS: Read + Seek>(input: &mut RS) -> H3mResult<usize> {
    let size = input.read_u32::<LE>()?;
    usize::try_from(size).or(Err(H3mError::Parsing(ParsingError::new(
        input.stream_position()?,
        format!("Can't convert map size value {} to usize.", size),
    ))))
}

pub struct H3mHeaderInfo {
    pub map_size: usize,
    pub has_underground: bool,
}

pub fn read_header<RS: Read + Seek>(input: &mut RS) -> H3mResult<H3mHeaderInfo> {
    let version = read_version(input)?;
    if version != Version::HotA {
        return Err(H3mError::Parsing(ParsingError::new(
            input.stream_position()?,
            format!("Unsupported h3m map version: {:?}.", version),
        )));
    }

    let hota_subversion = input.read_u32::<LE>()?;
    let bytes_to_skip = match hota_subversion {
        0x00000001 => Ok(2),
        0x00000003 => Ok(6),
        0x00000006 => Ok(11),
        0x00000007 => Ok(12),
        other => Err(H3mError::Parsing(ParsingError::new(
            input.stream_position()?,
            format!("Unexpected hota subversion value 0x{:08x}.", other),
        ))),
    };

    skip_bytes(input, bytes_to_skip?)?;

    skip_bool(input)?; // players_existence

    let map_size = read_size(input)?;
    let has_underground = read_bool(input)?;

    Ok(H3mHeaderInfo {
        map_size,
        has_underground,
    })
}
