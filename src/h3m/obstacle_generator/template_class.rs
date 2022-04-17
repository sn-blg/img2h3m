

#[derive(Clone, Copy, PartialEq)]
pub enum TemplateClass {
    OakTrees,
    PineTrees,
    Trees,
    Crater,
    Lake,
    Mound,
    Mountain,
    Rock,
    Stump,
}

impl TemplateClass {
    pub fn from_code(class: u32, subclass: u32) -> Option<Self> {
        match (class, subclass) {
            (135, 0) => Some(TemplateClass::OakTrees),
            (137, 0) => Some(TemplateClass::PineTrees),
            (155, 0) | (199, 0) => Some(TemplateClass::Trees),
            (118, 0) => Some(TemplateClass::Crater),
            (126, 0) | (177, 0) => Some(TemplateClass::Lake),
            (133, 0) => Some(TemplateClass::Mound),
            (134, 0) => Some(TemplateClass::Mountain),
            (147, 0) => Some(TemplateClass::Rock),
            (153, 0) => Some(TemplateClass::Stump),
            _ => None,
        }
    }
}
