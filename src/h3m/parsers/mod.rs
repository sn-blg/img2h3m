use crate::h3m::enums::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use common::*;
use conditions::*;
use header::*;
use hero_settings::*;
use players::*;
use std::io::{Cursor, Read, Seek};

mod common;
mod conditions;
mod header;
mod hero_settings;
mod players;

fn skip_teams<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let teams_count = input.read_u8()?;
    if teams_count > 0 {
        skip_bytes(input, 8)?;
    }

    Ok(())
}

fn skip_available_heroes<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    skip_bytes(input, 31)?; // todo: 31 - use count of all heroes: see B3 00 00-00

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

pub struct H3mInfo {
    pub map_size: Size,
    pub has_underground: bool,
    pub land_offset: u64,
}

pub fn parse(raw_map: &[u8]) -> H3mResult<H3mInfo> {
    let mut raw_map = Cursor::new(raw_map);

    let header_info = read_header(&mut raw_map)?;

    skip_players(&mut raw_map)?;

    skip_victory_condition(&mut raw_map)?;
    skip_loss_condition(&mut raw_map)?;

    skip_teams(&mut raw_map)?;

    skip_available_heroes(&mut raw_map)?;

    skip_bytes(&mut raw_map, 83)?; // banned artifacts, spells, skills (?)

    skip_rumors(&mut raw_map)?;

    skip_hero_settings(&mut raw_map)?;

    let land_offset = raw_map.position();

    Ok(H3mInfo {
        map_size: header_info.map_size,
        has_underground: header_info.has_underground,
        land_offset,
    })
}
