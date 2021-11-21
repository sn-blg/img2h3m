use crate::h3m::parsers::{H3mObjectTemplate, Mask};
use crate::h3m::result::H3mResult;
use crate::h3m::Terrain;

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
    terrain_group_mask: u16,
}

impl ObstacleTemplate {
    pub fn new(h3m_template: H3mObjectTemplate) -> ObstacleTemplate {
        let mask = h3m_template.shape_mask;
        let terrain_group_mask = ObstacleTemplate::terrain_group_mask(&h3m_template);
        ObstacleTemplate {
            h3m_template,
            shape: make_shape(&mask),
            index: 0,
            terrain_group_mask,
        }
    }

    fn terrain_group_mask(h3m_template: &H3mObjectTemplate) -> u16 {
        let mut terrain_group_mask = h3m_template.surface_editor_group_mask;

        if is_palm_tree(h3m_template) {
            terrain_group_mask &= !Terrain::Grass.group();
        }

        terrain_group_mask
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
        (terrain_group & self.terrain_group_mask) != 0
    }

    pub fn shape(&self) -> &[Delta] {
        &self.shape
    }
}

fn is_palm_tree(h3m_template: &H3mObjectTemplate) -> bool {
    h3m_template.class == 140 && h3m_template.subclass == 2
}
