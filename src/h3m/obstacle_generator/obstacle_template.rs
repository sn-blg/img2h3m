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
    frequency: usize,
}

impl ObstacleTemplate {
    pub fn new(h3m_template: H3mObjectTemplate) -> ObstacleTemplate {
        let mask = h3m_template.shape_mask;
        let terrain_group_mask = ObstacleTemplate::calc_terrain_group_mask(&h3m_template);
        let frequency = ObstacleTemplate::calc_frequency(&h3m_template);
        ObstacleTemplate {
            h3m_template,
            shape: make_shape(&mask),
            index: 0,
            terrain_group_mask,
            frequency,
        }
    }

    fn calc_terrain_group_mask(h3m_template: &H3mObjectTemplate) -> u16 {
        let mut terrain_group_mask = h3m_template.surface_editor_group_mask;

        if is_palm_tree(h3m_template) {
            terrain_group_mask &= !Terrain::Grass.group();
        }

        terrain_group_mask
    }

    fn calc_frequency(h3m_template: &H3mObjectTemplate) -> usize {
        if is_lake(h3m_template) || is_crater(h3m_template) || is_uniq_volcano(h3m_template) {
            10
        } else if is_ice_block(h3m_template) {
            5
        } else if is_uniq_mountain(h3m_template) {
            1
        } else {
            100
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
        (terrain_group & self.terrain_group_mask) != 0
    }

    pub fn shape(&self) -> &[Delta] {
        &self.shape
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }
}

fn is_palm_tree(h3m_template: &H3mObjectTemplate) -> bool {
    h3m_template.class == 140 && h3m_template.subclass == 2
}

fn is_lake(h3m_template: &H3mObjectTemplate) -> bool {
    if h3m_template.subclass == 0 {
        h3m_template.class == 177
            || h3m_template.class == 128
            || h3m_template.class == 154
            || h3m_template.class == 126
            || h3m_template.class == 121
    } else if h3m_template.subclass == 8 {
        h3m_template.class == 140
    } else {
        false
    }
}

fn is_ice_block(h3m_template: &H3mObjectTemplate) -> bool {
    h3m_template.class == 140 && h3m_template.subclass == 3
}

fn is_crater(h3m_template: &H3mObjectTemplate) -> bool {
    h3m_template.class == 118 && h3m_template.subclass == 0
}

fn is_uniq_mountain(h3m_template: &H3mObjectTemplate) -> bool {
    h3m_template.filename == "AVLMTWL7.def" || h3m_template.filename == "AVLrws02.def"
}

fn is_uniq_volcano(h3m_template: &H3mObjectTemplate) -> bool {
    h3m_template.filename == "AVLvol20.def" || h3m_template.filename == "AVLvol40.def"
}
