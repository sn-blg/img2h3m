use super::template_class::TemplateClass;
use crate::common::position::DeltaPos;
use crate::h3m::parser::{H3mObjectTemplate, Mask};
use crate::h3m::result::H3mResult;
use crate::h3m::Terrain;

fn make_shape(mask: &Mask) -> Vec<DeltaPos> {
    let mut shape = Vec::new();
    for (row, byte) in mask.iter().rev().enumerate() {
        for column in 0..7usize {
            let bit_mask = (1 << (7 - column)) as u8;
            if byte & bit_mask == 0 {
                shape.push(DeltaPos::new(row, column));
            }
        }
    }
    shape
}

pub struct ObstacleTemplate {
    h3m_template: H3mObjectTemplate,
    shape: Vec<DeltaPos>,
    index: u32,
    terrain_group_mask: u16,
    frequency: usize,
}

fn template_class(h3m_template: &H3mObjectTemplate) -> TemplateClass {
    TemplateClass::from_code(h3m_template.class, h3m_template.subclass).unwrap_or_else(|| {
        panic!(
            "Сouldn't define a class for the template '{:?}'",
            h3m_template
        )
    })
}

impl ObstacleTemplate {
    pub fn new(h3m_template: H3mObjectTemplate) -> ObstacleTemplate {
        let mask = h3m_template.shape_mask;
        let template_class = template_class(&h3m_template);
        let terrain_group_mask =
            ObstacleTemplate::fix_terrain_group_mask(template_class, &h3m_template);
        ObstacleTemplate {
            h3m_template,
            shape: make_shape(&mask),
            index: 0,
            terrain_group_mask,
            frequency: 100,
        }
    }

    fn fix_terrain_group_mask(
        template_class: TemplateClass,
        h3m_template: &H3mObjectTemplate,
    ) -> u16 {
        let mut terrain_group_mask = h3m_template.surface_editor_group_mask;

        if template_class == TemplateClass::Palms {
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

    pub fn shape(&self) -> &[DeltaPos] {
        &self.shape
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }
}
