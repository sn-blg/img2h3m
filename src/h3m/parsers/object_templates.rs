use crate::h3m::parsers::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

type Mask = [u8; 6];

#[derive(Debug)]
pub struct H3mObjectTemplate {
    filename: String,
    shape_mask: Mask,
    visit_mask: Mask,
    surface_type_mask: u16,
    surface_editor_group_mask: u16,
    class: u32,
    subclass: u32,
    group: u8,
    is_overlay: bool,
}

impl H3mObjectTemplate {
    pub fn _class(&self) -> u32 {
        self.class
    }
}

fn read_mask<RS: Read + Seek>(input: &mut RS) -> H3mResult<Mask> {
    let mut mask = Mask::default();
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

pub fn read_object_templates<RS: Read + Seek>(input: &mut RS) -> H3mResult<Vec<H3mObjectTemplate>> {
    let templates_count = input.read_u32::<LE>()?;

    println!("templates_count = {}", templates_count);

    let mut object_templates = Vec::with_capacity(templates_count.try_into()?);

    for _ in 0..templates_count {
        let object_template = read_object_template(input)?;
        object_templates.push(object_template);
    }

    println!("object_templates = {:#?}", object_templates);

    Ok(object_templates)
}
