use crate::h3m::obstacle_generator::obstacle_template::ObstacleTemplate;
use crate::h3m::parsers;

pub struct ObstacleTemplateList(Vec<ObstacleTemplate>);

impl ObstacleTemplateList {
    pub fn new() -> ObstacleTemplateList {
        ObstacleTemplateList(
            h3m_obstacle_templates()
                .into_iter()
                .map(ObstacleTemplate::new)
                .collect(),
        )
    }

    pub fn template(&self, index: usize) -> &ObstacleTemplate {
        &self.0[index]
    }

    pub fn template_mut(&mut self, index: usize) -> &mut ObstacleTemplate {
        &mut self.0[index]
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

fn h3m_obstacle_templates() -> Vec<parsers::H3mObjectTemplate> {
    struct H3mObjectTemplate {
        pub filename: &'static str,
        pub shape_mask: [u8; 6],
        pub visit_mask: [u8; 6],
        pub surface_type_mask: u16,
        pub surface_editor_group_mask: u16,
        pub class: u32,
        pub subclass: u32,
        pub group: u8,
        pub is_overlay: bool,
    }

    [
        H3mObjectTemplate {
            filename: "avlswt00.def",
            shape_mask: [255, 255, 255, 255, 255, 127],
            visit_mask: [0, 0, 0, 0, 0, 0],
            surface_type_mask: 3583,
            surface_editor_group_mask: 16,
            class: 140,
            subclass: 2,
            group: 0,
            is_overlay: false,
        },
        H3mObjectTemplate {
            filename: "AVLman20.def",
            shape_mask: [255, 255, 255, 255, 255, 63],
            visit_mask: [0, 0, 0, 0, 0, 0],
            surface_type_mask: 3583,
            surface_editor_group_mask: 16,
            class: 131,
            subclass: 0,
            group: 0,
            is_overlay: false,
        },
    ]
    .into_iter()
    .map(|t| parsers::H3mObjectTemplate {
        filename: String::from(t.filename),
        shape_mask: t.shape_mask,
        visit_mask: t.visit_mask,
        surface_type_mask: t.surface_type_mask,
        surface_editor_group_mask: t.surface_editor_group_mask,
        class: t.class,
        subclass: t.subclass,
        group: t.group,
        is_overlay: t.is_overlay,
    })
    .collect()
}
