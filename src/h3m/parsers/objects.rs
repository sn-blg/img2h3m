use crate::h3m::result::*;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek};

#[derive(Debug)]
struct H3mObjectTemplate {
    filename: String,
    shape_mask: [u8; 6],
    visit_mask: [u8; 6],
    terrain_type_mask: u32,
    class: u32,
    subclass: u32,
    group: u8,
    is_overlay: bool,
}

pub fn print_object_templates<RS: Read + Seek>(input: &mut RS) -> H3mResult<()> {
    let templates_count = input.read_u32::<LE>()?;

    println!("templates_count = {}", templates_count);

    Ok(())
}
