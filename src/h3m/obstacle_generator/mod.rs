use crate::h3m::parsers::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::Surface;
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
    map_size: usize,
    objects_data: ObjectsData,
}

impl ObstacleGenerator {
    pub fn new(
        map_size: usize,
        default_object_templates: &DefaultObjectTemplates,
    ) -> ObstacleGenerator {
        let mut obstacle_generator = ObstacleGenerator {
            obstacle_templates: ObstacleTemplates::new(),
            map_size,
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

    pub fn generate(&mut self, underground: bool, surfaces: &[Option<Surface>]) -> H3mResult<()> {
        for (index, &surface) in surfaces.iter().enumerate() {
            if let Some(surface) = surface {
                if surface.obstacle {
                    let column = (index % self.map_size).try_into()?;
                    let row = (index / self.map_size).try_into()?;

                    self.objects_data
                        .objects
                        .push(H3mObject::without_properties(column, row, underground, 2));
                }
            }
        }
        Ok(())
    }

    pub fn object_templates(&self) -> &[H3mObjectTemplate] {
        &self.objects_data.templates
    }

    pub fn objects(&self) -> &[H3mObject] {
        &self.objects_data.objects
    }
}
