use crate::h3m::enums::*;
use crate::h3m::result::*;
use header::*;
use std::io::Cursor;

mod common;
mod header;

pub struct H3mInfo {
    pub map_size: Size,
    pub has_underground: bool,
}

pub fn parse(raw_map: &[u8]) -> H3mResult<H3mInfo> {
    let mut raw_map = Cursor::new(raw_map);

    let header_info = read_header(&mut raw_map)?;

    Ok(H3mInfo {
        map_size: header_info.map_size,
        has_underground: header_info.has_underground,
    })
}
