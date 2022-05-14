use crate::h3m::parser::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::terrain_map::TerrainMap;
use map_area::{make_map_areas, MapArea};
use obstacle_template_list::ObstacleTemplateList;
use rand::rngs::ThreadRng;
use template_index_set::TemplateIndexSet;

mod common;
mod map_area;
mod obstacle_map;
mod obstacle_template;
mod obstacle_template_list;
pub mod template_class;
mod template_index_set;

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
    objects_data: ObjectsData,
    rng: ThreadRng,
}

impl ObstacleGenerator {
    pub fn new(default_object_templates: &DefaultObjectTemplates) -> ObstacleGenerator {
        ObstacleGenerator {
            obstacle_template_list: ObstacleTemplateList::new(),
            objects_data: ObjectsData::new(default_object_templates),
            rng: rand::thread_rng(),
        }
    }

    pub fn generate(&mut self, terrain_map: &TerrainMap) -> H3mResult<()> {
        let map_areas = make_map_areas(terrain_map, terrain_map.size(), 36)?;
        for area in map_areas {
            self.generate_in_area(terrain_map.underground(), area)?;
        }
        Ok(())
    }

    fn generate_in_area(&mut self, underground: bool, mut map_area: MapArea) -> H3mResult<()> {
        let mut template_index_set = TemplateIndexSet::new(&self.obstacle_template_list);
        while !template_index_set.is_empty() {
            let template_index = template_index_set.random_index(&mut self.rng);
            let position_index = self.try_position_obstacle(template_index, &map_area);
            match position_index {
                Some(position_index) => {
                    self.add_obstacle(template_index, position_index, underground, &mut map_area)?
                }
                None => template_index_set.remove_index(template_index),
            }
        }
        Ok(())
    }

    fn try_position_obstacle(&self, template_index: usize, map_area: &MapArea) -> Option<usize> {
        let obstacle = self.obstacle_template_list.template(template_index);
        map_area.try_position_obstacle(obstacle)
    }

    fn add_obstacle(
        &mut self,
        template_index: usize,
        position_index: usize,
        underground: bool,
        map_area: &mut MapArea,
    ) -> H3mResult<()> {
        let obstacle = self.obstacle_template_list.template_mut(template_index);

        if obstacle.h3m_template_index() == 0 {
            obstacle.set_h3m_template_index(self.objects_data.templates.len())?;
            self.objects_data
                .templates
                .push(obstacle.h3m_template().clone());
        }

        let position = map_area.position(position_index);
        self.objects_data
            .objects
            .push(H3mObject::without_properties(
                position.column(),
                position.row(),
                underground,
                obstacle.h3m_template_index(),
            ));

        map_area.add_obstacle(position_index, obstacle);

        Ok(())
    }

    pub fn object_templates(&self) -> &[H3mObjectTemplate] {
        &self.objects_data.templates
    }

    pub fn objects(&self) -> &[H3mObject] {
        &self.objects_data.objects
    }
}
