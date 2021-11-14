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

const DEFAULT_OBJECT_TEMPLATES_COUNT: usize = 2;
pub type DefaultObjectTemplates = [H3mObjectTemplate; DEFAULT_OBJECT_TEMPLATES_COUNT];

pub fn read_default_and_skip_other_object_templates<RS: Read + Seek>(
    input: &mut RS,
) -> H3mResult<DefaultObjectTemplates> {
    let templates_count = input.read_u32::<LE>()?;

    println!("templates_count = {}", templates_count);

    let templates_count: usize = templates_count.try_into()?;

    if templates_count < DEFAULT_OBJECT_TEMPLATES_COUNT {
        return Err(H3mError::Parsing(ParsingError::new(
            input.stream_position()?,
            format!(
                "Templates count is {} (less than {}).",
                templates_count, DEFAULT_OBJECT_TEMPLATES_COUNT
            ),
        )));
    }

    let default_object_templates = [read_object_template(input)?, read_object_template(input)?];

    for _ in 0..(templates_count - DEFAULT_OBJECT_TEMPLATES_COUNT) {
        let object_template = read_object_template(input)?;
        println!("object_template = {:?}", object_template);
    }

    Ok(default_object_templates)
}
