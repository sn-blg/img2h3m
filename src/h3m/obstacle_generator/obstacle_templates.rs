use crate::h3m::parsers::H3mObjectTemplate;

pub struct ObstacleTemplates {
    template: H3mObjectTemplate,
}

impl ObstacleTemplates {
    pub fn new() -> ObstacleTemplates {
        ObstacleTemplates {
            template: H3mObjectTemplate {
                filename: String::from("AVLHPN03.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 1024,
                class: 155,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        }
    }

    pub fn object_template(&self) -> &H3mObjectTemplate {
        &self.template
    }
}
