use crate::h3m::parser::common::*;
use crate::h3m::result::*;
use byteorder::{WriteBytesExt, LE};
use std::io::Write;

#[derive(Debug)]
pub struct H3mLocation {
    column: u8,
    row: u8,
    underground: bool,
}

impl H3mLocation {
    fn new(column: u8, row: u8, underground: bool) -> H3mLocation {
        H3mLocation {
            column,
            row,
            underground,
        }
    }
}

#[derive(Debug)]
pub struct H3mObject {
    location: H3mLocation,
    template_idx: u32,
}

impl H3mObject {
    pub fn without_properties(
        column: u8,
        row: u8,
        underground: bool,
        template_idx: u32,
    ) -> H3mObject {
        H3mObject {
            location: H3mLocation::new(column, row, underground),
            template_idx,
        }
    }
}

fn write_location<W: Write>(location: &H3mLocation, output: &mut W) -> H3mResult<()> {
    output.write_u8(location.column)?;
    output.write_u8(location.row)?;
    write_bool(location.underground, output)?;
    Ok(())
}

fn write_object<W: Write>(object: &H3mObject, output: &mut W) -> H3mResult<()> {
    write_location(&object.location, output)?;
    output.write_u32::<LE>(object.template_idx)?;

    let zero = [0u8; 5];
    output.write_all(&zero)?;

    Ok(())
}

pub fn write_objects<W: Write>(objects: &[H3mObject], output: &mut W) -> H3mResult<()> {
    let count = objects.len();
    let count = u32::try_from(count)?;

    output.write_u32::<LE>(count)?;

    for object in objects {
        write_object(object, output)?;
    }

    Ok(())
}
