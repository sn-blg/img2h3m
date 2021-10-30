use crate::h3m::enums::*;
use crate::h3m::result::*;
use byteorder::ReadBytesExt;
use common::*;
use conditions::*;
use header::*;
use players::*;
use std::io::{Cursor, Read, Seek};

mod common;
mod conditions;
mod header;
mod players;

fn skip_teams<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let teams_count = input.read_u8()?;
    if teams_count > 0 {
        skip_bytes(input, 8)?;
    }

    Ok(())
}

pub struct H3mInfo {
    pub map_size: Size,
    pub has_underground: bool,
}

pub fn parse(raw_map: &[u8]) -> H3mResult<H3mInfo> {
    let mut raw_map = Cursor::new(raw_map);

    let header_info = read_header(&mut raw_map)?;

    skip_players(&mut raw_map)?;

    skip_victory_condition(&mut raw_map)?;
    skip_loss_condition(&mut raw_map)?;

    skip_teams(&mut raw_map)?;

    Ok(H3mInfo {
        map_size: header_info.map_size,
        has_underground: header_info.has_underground,
    })
}
