use crate::h3m::parsers::common::*;
use crate::h3m::result::*;
use byteorder::ReadBytesExt;
use std::io::{Read, Seek};

enum VictoryCondition {
    Default,
    AcquireArtifact,
    AccumCreatures,
    AccumResources,
    UpgradeTown,
    BuildGrail,
    DefeatHero,
    CaptureTown,
    DefeatMonster,
    FlagAllDwellings,
    FlagAllMines,
    TransportArtifact,
    DefeatAllMonsters,
    SurviveCertainTime,
}

fn read_victory_condition<RS: Read + Seek>(input: &mut RS) -> H3mResult<VictoryCondition> {
    let victory_condition = input.read_u8()?;
    match victory_condition {
        0xFF => Ok(VictoryCondition::Default),
        0x00 => Ok(VictoryCondition::AcquireArtifact),
        0x01 => Ok(VictoryCondition::AccumCreatures),
        0x02 => Ok(VictoryCondition::AccumResources),
        0x03 => Ok(VictoryCondition::UpgradeTown),
        0x04 => Ok(VictoryCondition::BuildGrail),
        0x05 => Ok(VictoryCondition::DefeatHero),
        0x06 => Ok(VictoryCondition::CaptureTown),
        0x07 => Ok(VictoryCondition::DefeatMonster),
        0x08 => Ok(VictoryCondition::FlagAllDwellings),
        0x09 => Ok(VictoryCondition::FlagAllMines),
        0x0A => Ok(VictoryCondition::TransportArtifact),
        0x0B => Ok(VictoryCondition::DefeatAllMonsters),
        0x0C => Ok(VictoryCondition::SurviveCertainTime),
        other => Err(H3mError::Parsing(ParsingError::new(
            input.stream_position()?,
            format!("Unexpected victory condition value 0x{:02x}.", other),
        ))),
    }
}

pub fn skip_victory_condition<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let victory_condition = read_victory_condition(input)?;

    let bytes_to_skip: u32 = match victory_condition {
        VictoryCondition::Default => 0,
        VictoryCondition::AcquireArtifact => 3,
        VictoryCondition::AccumCreatures => 8,
        VictoryCondition::AccumResources => 7,
        VictoryCondition::UpgradeTown => 7,
        VictoryCondition::BuildGrail => 5,
        VictoryCondition::DefeatHero => 5,
        VictoryCondition::CaptureTown => 5,
        VictoryCondition::DefeatMonster => 5,
        VictoryCondition::FlagAllDwellings => 2,
        VictoryCondition::FlagAllMines => 2,
        VictoryCondition::TransportArtifact => 6,
        VictoryCondition::DefeatAllMonsters => 2,
        VictoryCondition::SurviveCertainTime => 6,
    };

    skip_bytes(input, bytes_to_skip)?;

    Ok(())
}

enum LossCondition {
    Default,
    LoseTown,
    LoseHero,
    TimeExpires,
}

fn read_loss_condition<RS: Read + Seek>(input: &mut RS) -> H3mResult<LossCondition> {
    let loss_condition = input.read_u8()?;
    match loss_condition {
        0xFF => Ok(LossCondition::Default),
        0x00 => Ok(LossCondition::LoseTown),
        0x01 => Ok(LossCondition::LoseHero),
        0x02 => Ok(LossCondition::TimeExpires),
        other => Err(H3mError::Parsing(ParsingError::new(
            input.stream_position()?,
            format!("Unexpected loss condition value 0x{:02x}.", other),
        ))),
    }
}

pub fn skip_loss_condition<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let loss_condition = read_loss_condition(input)?;

    let bytes_to_skip: u32 = match loss_condition {
        LossCondition::Default => 0,
        LossCondition::LoseTown => 3,
        LossCondition::LoseHero => 3,
        LossCondition::TimeExpires => 2,
    };

    skip_bytes(input, bytes_to_skip)?;

    Ok(())
}
