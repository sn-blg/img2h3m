use crate::h3m::parsers::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

#[derive(Debug)]
struct H3mObjectTemplate {
    filename: String,
    shape_mask: [u8; 6],
    visit_mask: [u8; 6],
    surface_type_mask: u16,
    surface_editor_group_mask: u16,
    class: u32,
    subclass: u32,
    group: u8,
    is_overlay: bool,
}

fn read_mask<RS: Read + Seek>(input: &mut RS) -> H3mResult<[u8; 6]> {
    let mut mask = [0u8; 6];
    input.read_exact(&mut mask)?;
    Ok(mask)
}

fn read_object_template<RS: Read + Seek>(input: &mut RS) -> H3mResult<H3mObjectTemplate> {
    let object_template = H3mObjectTemplate {
        filename: read_string(input)?,
        shape_mask: read_mask(input)?,
        visit_mask: read_mask(input)?,
        surface_type_mask: input.read_u16::<LE>()?,
        surface_editor_group_mask: input.read_u16::<LE>()?,
        class: input.read_u32::<LE>()?,
        subclass: input.read_u32::<LE>()?,
        group: input.read_u8()?,
        is_overlay: read_bool(input)?,
    };

    skip_bytes(input, 16)?; // unknown (so far seen zeroes here)

    Ok(object_template)
}

pub fn print_object_templates<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let templates_count = input.read_u32::<LE>()?;

    println!("templates_count = {}", templates_count);

    for _ in 0..templates_count {
        let object_template = read_object_template(input)?;
        println!("object_template = {:?}", object_template);
    }

    Ok(())
}

#[derive(Debug)]
struct H3MLocation {
    column: u8,
    row: u8,
    underground: bool,
}

#[derive(Debug)]
struct H3MObject {
    loc: H3MLocation,
    template_idx: u32,
}

fn read_location<RS: Read + Seek>(input: &mut RS) -> H3mResult<H3MLocation> {
    Ok(H3MLocation {
        column: input.read_u8()?,
        row: input.read_u8()?,
        underground: read_bool(input)?,
    })
}

fn read_object<RS: Read + Seek>(input: &mut RS) -> H3mResult<H3MObject> {
    let object = H3MObject {
        loc: read_location(input)?,
        template_idx: input.read_u32::<LE>()?,
    };

    skip_bytes(input, 5)?; // unknown (so far seen zeroes here)

    Ok(object)
}

pub fn print_objects<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let objects_count = input.read_u32::<LE>()?;

    println!("objects_count = {}", objects_count);

    for _ in 0..objects_count {
        let object = read_object(input)?;
        println!("object = {:?}", object);
    }

    Ok(())
}
