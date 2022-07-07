use crate::common::position::DeltaPos;
use crate::h3m::parser::H3mObjectTemplate;
use crate::h3m::result::H3mResult;
pub use factory::ObstacleTemplateCreateParams;
use sparsity::Sparsity;
use template_class::TemplateClass;

mod cell_validator;
mod factory;
mod sparsity;
mod template_class;
mod tile_side;

pub struct ObstacleTemplate {
    h3m_template: H3mObjectTemplate,
    filename: &'static str,
    template_class: TemplateClass,
    h3m_template_index: u32,
    shape: Vec<DeltaPos>,
    terrain_group_mask: u16,
    frequency: usize,
    may_located_on_mixed_tiles: bool,
    sparsity: Sparsity, // limit: square of the distance to the same obstacle
}

impl ObstacleTemplate {
    pub fn h3m_template(&self) -> &H3mObjectTemplate {
        &self.h3m_template
    }

    pub fn filename(&self) -> &'static str {
        self.filename
    }

    pub fn h3m_template_index(&self) -> u32 {
        self.h3m_template_index
    }

    pub fn set_h3m_template_index(&mut self, index: usize) -> H3mResult<()> {
        self.h3m_template_index = index.try_into()?;
        Ok(())
    }

    pub fn shape(&self) -> &[DeltaPos] {
        &self.shape
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn sparsity(&self) -> Sparsity {
        self.sparsity
    }

    pub fn is_valid_terrain(&self, terrain_group: u16) -> bool {
        (terrain_group & self.terrain_group_mask) != 0
    }
}
