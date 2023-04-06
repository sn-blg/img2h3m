use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use common::*;
use conditions::*;
use header::*;
use hero_settings::*;
pub use object_templates::*;
pub use objects::*;
use players::*;
use std::io::{Cursor, Read, Seek};

mod common;
mod conditions;
mod header;
mod hero_settings;
mod object_templates;
mod objects;
mod players;

fn skip_teams<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let teams_count = input.read_u8()?;
    if teams_count > 0 {
        skip_bytes(input, 8)?;
    }

    Ok(())
}

fn skip_available_heroes<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let heroes_count = input.read_u32::<LE>()?;
    let heroes_bitmask_size_in_bytes = (heroes_count as f64 / 8.0).ceil() as u32;

    skip_bytes(input, heroes_bitmask_size_in_bytes)?;

    skip_bytes(input, 4)?; // empty

    let custom_heroes_count = input.read_u8()?;
    for _ in 0..custom_heroes_count {
        skip_bytes(input, 1)?; // hero ID
        skip_bytes(input, 1)?; // hero portrait
        skip_string(input)?; // name
        skip_bytes(input, 1)?; // which players can hire him
    }

    Ok(())
}

fn skip_rumors<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let rumors_count = input.read_u32::<LE>()?;
    for _ in 0..rumors_count {
        skip_string(input)?; // rumor name
        skip_string(input)?; // rumor text
    }

    Ok(())
}

fn skip_land<RS: Read + Seek>(input: &mut RS, map_size: usize) -> H3mResult<()> {
    let count = map_size * map_size * MAP_CELL_SIZE;
    let count = u32::try_from(count)?;
    skip_bytes(input, count)
}

pub const MAP_CELL_SIZE: usize = 7;

pub struct H3mInfo {
    pub map_size: usize,
    pub land_offset: usize,
    pub underground_offset: Option<usize>,
    pub objects_templates_offset: usize,
    pub default_object_templates: [H3mObjectTemplate; 2],
}

struct LandOffsetInfo {
    land_offset: usize,
    underground_offset: Option<usize>,
}

fn land_offset_from_objects_templates_offset(
    header_info: &H3mHeaderInfo,
    objects_templates_offset: usize,
) -> H3mResult<LandOffsetInfo> {
    let map_size = header_info.map_size;
    let land_size_in_bytes = map_size * map_size * MAP_CELL_SIZE;

    let mut current_offset = objects_templates_offset;
    let mut underground_offset = None;

    if header_info.has_underground {
        current_offset = current_offset.checked_sub(land_size_in_bytes).ok_or_else(|| {
            H3mError::Parameter(ParameterError::new(
                "Invalid input map format. Can't obtain underground land offset - templates offset is too small.",
            ))
        })?;
        underground_offset = Some(current_offset);
    }

    let land_offset = current_offset
        .checked_sub(land_size_in_bytes)
        .ok_or_else(|| {
            H3mError::Parameter(ParameterError::new(
            "Invalid input map format. Can't obtain land offset - templates offset is too small.",
        ))
        })?;

    Ok(LandOffsetInfo {
        land_offset,
        underground_offset,
    })
}

pub fn parse(raw_map: &[u8]) -> H3mResult<H3mInfo> {
    let objects_templates_offset = find_objects_templates_offset(raw_map)?;

    let mut raw_map = Cursor::new(raw_map);

    let header_info = read_header(&mut raw_map)?;

    raw_map.set_position(u64::try_from(objects_templates_offset)?);
    let default_object_templates = read_default_object_templates(&mut raw_map)?;

    let land_offset_info =
        land_offset_from_objects_templates_offset(&header_info, objects_templates_offset)?;

    Ok(H3mInfo {
        map_size: header_info.map_size,
        land_offset: land_offset_info.land_offset,
        underground_offset: land_offset_info.underground_offset,
        objects_templates_offset,
        default_object_templates,
    })
}
