

#[derive(Clone, Copy, PartialEq)]
pub enum TemplateClass {
    OakTrees,
    Crater,
}

impl TemplateClass {
    pub fn from_code(class: u32, subclass: u32) -> Option<Self> {
        match (class, subclass) {
            (135, 0) => Some(TemplateClass::OakTrees),
            (118, 0) => Some(TemplateClass::Crater),
            _ => None,
        }
    }
}
