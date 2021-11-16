use crate::h3m::parsers::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use obstacle_templates::ObstacleTemplates;

mod obstacle_templates;

struct ObjectsData {
    templates: Vec<H3mObjectTemplate>,
    objects: Vec<H3mObject>,
}

impl ObjectsData {
    fn new(default_object_templates: &DefaultObjectTemplates) -> ObjectsData {
        ObjectsData {
            templates: vec![
                default_object_templates[0].clone(),
                default_object_templates[1].clone(),
            ],
            objects: Vec::new(),
        }
    }
}

pub struct ObstacleGenerator {
    obstacle_templates: ObstacleTemplates,
    objects_data: ObjectsData,
}

impl ObstacleGenerator {
    pub fn new(default_object_templates: &DefaultObjectTemplates) -> ObstacleGenerator {
        let mut obstacle_generator = ObstacleGenerator {
            obstacle_templates: ObstacleTemplates::new(),
            objects_data: ObjectsData::new(default_object_templates),
        };

        obstacle_generator.objects_data.templates.push(
            obstacle_generator
                .obstacle_templates
                .object_template()
                .clone(),
        );

        obstacle_generator
    }

    pub fn generate(&mut self, column: u8, row: u8, underground: bool) {
        self.objects_data
            .objects
            .push(H3mObject::without_properties(column, row, underground, 2));
    }

    pub fn object_templates(&self) -> &[H3mObjectTemplate] {
        &self.objects_data.templates
    }

    pub fn objects(&self) -> &[H3mObject] {
        &self.objects_data.objects
    }
}
