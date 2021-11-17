use crate::h3m::parsers::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::Surface;
use obstacle_template_list::ObstacleTemplateList;
use rand::Rng;
use std::collections::HashSet;

mod obstacle_template_list;

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
    obstacle_template_list: ObstacleTemplateList,
    map_size: usize,
    objects_data: ObjectsData,
}

impl ObstacleGenerator {
    pub fn new(
        map_size: usize,
        default_object_templates: &DefaultObjectTemplates,
    ) -> ObstacleGenerator {
        ObstacleGenerator {
            obstacle_template_list: ObstacleTemplateList::new(),
            map_size,
            objects_data: ObjectsData::new(default_object_templates),
        }
    }

    pub fn generate(&mut self, underground: bool, surfaces: &[Option<Surface>]) -> H3mResult<()> {
        let mut template_index_set: HashSet<usize> =
            (0..self.obstacle_template_list.len()).collect();

        println!("template_index_set = {:?}", template_index_set);

        let mut surfaces = surfaces.to_vec();
        while !template_index_set.is_empty() {
            let template_index = *template_index_set
                .iter()
                .nth(rand::thread_rng().gen_range(0..template_index_set.len()))
                .unwrap();

            let is_obstacle_added =
                self.try_add_obstacle(template_index, underground, &mut surfaces)?;

            if !is_obstacle_added {
                assert!(template_index_set.remove(&template_index));
                println!("template_index_set = {:?}", template_index_set);
            }
        }

        Ok(())
    }

    fn try_add_obstacle(
        &mut self,
        template_index: usize,
        underground: bool,
        surfaces: &mut [Option<Surface>],
    ) -> H3mResult<bool> {
        let template = self.obstacle_template_list.template(template_index);
        for (index, surface) in surfaces.iter_mut().enumerate() {
            let surface = match surface {
                Some(surface) => surface,
                None => continue,
            };

            if !surface.obstacle {
                continue;
            }

            let is_valid_terrain = (1 << (surface.terrain.code() as u16)
                & template.h3m_template().surface_editor_group_mask)
                != 0;

            if !is_valid_terrain {
                continue;
            }

            let column = (index % self.map_size).try_into()?;
            let row = (index / self.map_size).try_into()?;

            if template.index() == 0 {
                template.set_index(self.objects_data.templates.len());
                self.objects_data
                    .templates
                    .push(template.h3m_template().clone());
            }

            self.objects_data
                .objects
                .push(H3mObject::without_properties(
                    column,
                    row,
                    underground,
                    template.index().try_into()?,
                ));

            surface.obstacle = false;

            return Ok(true);
        }
        Ok(false)
    }

    pub fn object_templates(&self) -> &[H3mObjectTemplate] {
        &self.objects_data.templates
    }

    pub fn objects(&self) -> &[H3mObject] {
        &self.objects_data.objects
    }
}
