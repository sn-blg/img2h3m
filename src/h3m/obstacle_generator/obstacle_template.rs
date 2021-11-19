use crate::h3m::parsers::{H3mObjectTemplate, Mask};
use crate::h3m::result::H3mResult;

#[derive(Debug)]
pub struct Delta {
    row: usize,
    column: usize,
}

impl Delta {
    fn new(row: usize, column: usize) -> Delta {
        Delta { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

fn make_shape(mask: &Mask) -> Vec<Delta> {
    let mut shape = Vec::new();
    for (row, byte) in mask.iter().rev().enumerate() {
        for column in 0..7usize {
            let bit_mask = (1 << (7 - column)) as u8;
            if byte & bit_mask == 0 {
                shape.push(Delta::new(row, column));
            }
        }
    }
    shape
}

pub struct ObstacleTemplate {
    h3m_template: H3mObjectTemplate,
    shape: Vec<Delta>,
    index: u32,
}

impl ObstacleTemplate {
    pub fn new(h3m_template: H3mObjectTemplate) -> ObstacleTemplate {
        let mask = h3m_template.shape_mask;
        ObstacleTemplate {
            h3m_template,
            shape: make_shape(&mask),
            index: 0,
        }
    }

    pub fn h3m_template(&self) -> &H3mObjectTemplate {
        &self.h3m_template
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn set_index_usize(&mut self, index: usize) -> H3mResult<()> {
        self.index = index.try_into()?;
        Ok(())
    }

    pub fn is_valid_terrain(&self, terrain_group: u16) -> bool {
        (terrain_group & self.h3m_template.surface_editor_group_mask) != 0
    }

    pub fn shape(&self) -> &[Delta] {
        &self.shape
    }
}
