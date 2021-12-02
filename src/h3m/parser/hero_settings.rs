use crate::h3m::parser::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

fn skip_one_hero_settings<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let is_settings_set = read_bool(input)?;
    if !is_settings_set {
        return Ok(());
    }

    let is_experience_set = read_bool(input)?;
    if is_experience_set {
        skip_bytes(input, 4)?; // experience
    }

    let is_secondary_skills_set = read_bool(input)?;
    if is_secondary_skills_set {
        let skills_count = input.read_u32::<LE>()?;
        for _ in 0..skills_count {
            skip_bytes(input, 2)?; // skill identifier + skill level
        }
    }

    let is_artifacts_set = read_bool(input)?;
    if is_artifacts_set {
        skip_bytes(input, 38)?;
        let things_in_backpack = input.read_u16::<LE>()?;
        for _ in 0..things_in_backpack {
            skip_bytes(input, 2)?;
        }
    }

    let is_biography_set = read_bool(input)?;
    if is_biography_set {
        skip_string(input)?;
    }

    skip_bytes(input, 1)?; // gender ( FF - default, 00 - male, 01 - female)

    let is_spells_set = read_bool(input)?;
    if is_spells_set {
        skip_bytes(input, 9)?; // bitfield for spells
    }

    let is_primary_skills_set = read_bool(input)?;
    if is_primary_skills_set {
        skip_bytes(input, 4)?; // attack, defence, spell, knowledge
    }

    Ok(())
}

pub fn skip_hero_settings<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let heroes_count = input.read_u32::<LE>()?;
    for _ in 0..heroes_count {
        skip_one_hero_settings(input)?;
    }

    Ok(())
}
