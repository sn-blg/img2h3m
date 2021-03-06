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

pub fn parse(raw_map: &[u8]) -> H3mResult<H3mInfo> {
    let mut raw_map = Cursor::new(raw_map);

    let H3mHeaderInfo {
        map_size,
        has_underground,
    } = read_header(&mut raw_map)?;

    skip_players(&mut raw_map)?;

    skip_victory_condition(&mut raw_map)?;
    skip_loss_condition(&mut raw_map)?;

    skip_teams(&mut raw_map)?;

    skip_available_heroes(&mut raw_map)?;

    skip_bytes(&mut raw_map, 31)?; // 31 bytes filled with 00
    skip_bytes(&mut raw_map, 52)?; // banned artifacts, spells, skills (?)

    skip_rumors(&mut raw_map)?;

    skip_hero_settings(&mut raw_map)?;

    let land_offset = position(&raw_map)?;
    skip_land(&mut raw_map, map_size)?; // land

    let underground_offset = if has_underground {
        Some(position(&raw_map)?)
    } else {
        None
    };
    if has_underground {
        skip_land(&mut raw_map, map_size)?; // underground
    }

    let objects_templates_offset = position(&raw_map)?;

    let default_object_templates = read_default_and_skip_other_object_templates(&mut raw_map)?;

    Ok(H3mInfo {
        map_size,
        land_offset,
        underground_offset,
        objects_templates_offset,
        default_object_templates,
    })
}
