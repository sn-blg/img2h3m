use crate::h3m::parser::common::*;
use crate::h3m::result::*;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::io::{Read, Seek, Write};

pub type Mask = [u8; 6];

#[derive(Debug, Clone)]
pub struct H3mObjectTemplate {
    pub filename: String,
    pub shape_mask: Mask,
    pub visit_mask: Mask,
    pub surface_type_mask: u16,
    pub surface_editor_group_mask: u16,
    pub class: u32,
    pub subclass: u32,
    pub group: u8,
    pub is_overlay: bool,
}

fn read_mask<RS: Read + Seek>(input: &mut RS) -> H3mResult<Mask> {
    let mut mask = Mask::default();
    input.read_exact(&mut mask)?;
    Ok(mask)
}

fn write_object_template<W: Write>(
    object_template: &H3mObjectTemplate,
    output: &mut W,
) -> H3mResult<()> {
    write_string(&object_template.filename, output)?;
    output.write_all(&object_template.shape_mask)?;
    output.write_all(&object_template.visit_mask)?;
    output.write_u16::<LE>(object_template.surface_type_mask)?;
    output.write_u16::<LE>(object_template.surface_editor_group_mask)?;
    output.write_u32::<LE>(object_template.class)?;
    output.write_u32::<LE>(object_template.subclass)?;
    output.write_u8(object_template.group)?;
    write_bool(object_template.is_overlay, output)?;

    let zero = [0u8; 16];
    output.write_all(&zero)?;

    Ok(())
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

pub fn write_object_templates<W: Write>(
    object_templates: &[H3mObjectTemplate],
    output: &mut W,
) -> H3mResult<()> {
    let templates_count = u32::try_from(object_templates.len())?;
    output.write_u32::<LE>(templates_count)?;

    for object_template in object_templates {
        write_object_template(object_template, output)?
    }

    Ok(())
}

const DEFAULT_OBJECT_TEMPLATES_COUNT: usize = 2;
pub type DefaultObjectTemplates = [H3mObjectTemplate; DEFAULT_OBJECT_TEMPLATES_COUNT];

pub fn read_default_object_templates<RS: Read + Seek>(
    input: &mut RS,
) -> H3mResult<DefaultObjectTemplates> {
    let templates_count = input.read_u32::<LE>()?;
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
        read_object_template(input)?;
    }

    Ok(default_object_templates)
}

pub fn find_objects_templates_offset(raw_map: &[u8]) -> H3mResult<usize> {
    let signature = [
        0x0Cu8, 0x00, 0x00, 0x00, 0x41, 0x56, 0x57, 0x6D, 0x72, 0x6E, 0x64, 0x30, 0x2E, 0x64, 0x65,
        0x66,
    ];

    let offset = raw_map
        .windows(signature.len())
        .position(|window| window == signature)
        .ok_or_else(|| {
            H3mError::Parameter(ParameterError::new(
                "Invalid input map format. Templates signature not found.",
            ))
        })?
        .checked_sub(4)
        .ok_or_else(|| {
            H3mError::Parameter(ParameterError::new(
                "Invalid input map format. Templates offset is too small.",
            ))
        })?;

    Ok(offset)
}
