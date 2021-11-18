use crate::h3m::parsers::H3mObjectTemplate;
use crate::h3m::result::H3mResult;

pub struct ObstacleTemplate {
    h3m_template: H3mObjectTemplate,
    index: u32,
}

impl ObstacleTemplate {
    pub fn new(h3m_template: H3mObjectTemplate) -> ObstacleTemplate {
        ObstacleTemplate {
            h3m_template,
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
}
