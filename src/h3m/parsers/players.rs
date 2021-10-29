use crate::h3m::parsers::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

fn skip_hero<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    skip_bytes(input, 1)?;
    skip_string(input)?;
    Ok(())
}

fn skip_player<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    skip_bytes(input, 7)?;

    let has_main_town = read_bool(input)?;
    if has_main_town {
        skip_bytes(input, 5)?;
    }

    skip_bool(input)?; // random_hero

    let hero_type = input.read_u8()?;
    if hero_type != 0xFF {
        skip_hero(input)?;
    }

    skip_bytes(input, 1)?; // garbage

    let heroes_count = input.read_u32::<LE>()?;
    for _ in 0..heroes_count {
        skip_hero(input)?;
    }

    Ok(())
}

pub fn skip_players<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    for _ in 0..8 {
        skip_player(input)?;
    }
    Ok(())
}
