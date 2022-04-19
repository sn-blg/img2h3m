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
    Cactus,
    SandDune,
    BarchanDunes,
    Palms, // !
    SandPit,
    YuccaTrees, //
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
            (116, 0) => Some(TemplateClass::Cactus),
            (148, 0) => Some(TemplateClass::SandDune),
            (140, 6) => Some(TemplateClass::BarchanDunes),
            (140, 2) => Some(TemplateClass::Palms),
            (149, 0) => Some(TemplateClass::SandPit),
            (204, 0) | (160, 0) => Some(TemplateClass::YuccaTrees),
            _ => None,
        }
    }
}
