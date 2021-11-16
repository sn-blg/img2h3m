use crate::h3m::parsers::H3mObjectTemplate;
use crate::h3m::Terrain;
use std::collections::HashMap;

pub struct TemplateData {
    h3m_template: H3mObjectTemplate,
    index: usize,
}

impl TemplateData {
    fn new(h3m_template: H3mObjectTemplate) -> TemplateData {
        TemplateData {
            h3m_template,
            index: 0,
        }
    }

    pub fn h3m_template(&self) -> &H3mObjectTemplate {
        &self.h3m_template
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

pub struct ObstacleTemplates(HashMap<Terrain, TemplateData>);

impl ObstacleTemplates {
    pub fn new() -> ObstacleTemplates {
        ObstacleTemplates(
            h3m_obstacle_templates()
                .into_iter()
                .map(|(terrain, h3m_template)| (terrain, TemplateData::new(h3m_template)))
                .collect(),
        )
    }

    pub fn object_template(&mut self, terrain: Terrain) -> &mut TemplateData {
        self.0.get_mut(&terrain).unwrap()
    }
}

fn h3m_obstacle_templates() -> Vec<(Terrain, H3mObjectTemplate)> {
    vec![
        (
            Terrain::Dirt,
            H3mObjectTemplate {
                filename: String::from("AVLrk5d0.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 1,
                class: 147,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Sand,
            H3mObjectTemplate {
                filename: String::from("AVcact03.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 2,
                class: 116,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Grass,
            H3mObjectTemplate {
                filename: String::from("AVLSPTR1.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 20,
                class: 135,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Snow,
            H3mObjectTemplate {
                filename: String::from("AVLddsn1.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 8,
                class: 119,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Swamp,
            H3mObjectTemplate {
                filename: String::from("TPMTREE3.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 16,
                class: 199,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Rough,
            H3mObjectTemplate {
                filename: String::from("AVLroug2.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 32,
                class: 155,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Subterranean,
            H3mObjectTemplate {
                filename: String::from("AVLr07u0.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 64,
                class: 147,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Lava,
            H3mObjectTemplate {
                filename: String::from("AVLdead8.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 208,
                class: 119,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Highland,
            H3mObjectTemplate {
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
        ),
        (
            Terrain::Wasteland,
            H3mObjectTemplate {
                filename: String::from("AVLtRo02.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 3583,
                surface_editor_group_mask: 2080,
                class: 199,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
        (
            Terrain::Water,
            H3mObjectTemplate {
                filename: String::from("AVLrk3w0.def"),
                shape_mask: [255, 255, 255, 255, 255, 127],
                visit_mask: [0, 0, 0, 0, 0, 0],
                surface_type_mask: 256,
                surface_editor_group_mask: 256,
                class: 147,
                subclass: 0,
                group: 0,
                is_overlay: false,
            },
        ),
    ]
}
