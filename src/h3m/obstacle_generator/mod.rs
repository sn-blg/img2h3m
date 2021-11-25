use crate::h3m::parsers::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::Surface;
use map_area::{make_map_areas, MapArea};

use obstacle_template_list::ObstacleTemplateList;
use template_index_set::TemplateIndexSet;

mod common;
mod map_area;
mod obstacle_template;
mod obstacle_template_list;
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
        let map_len = self.map_size * self.map_size;
        if surfaces.len() > map_len {
            return Err(H3mError::Parameter(ParameterError::new(format!(
                "Error surfaces length ({}) greater than map length ({}).",
                surfaces.len(),
                map_len
            ))));
        }
        let map_areas = make_map_areas(self.map_size, surfaces, 36, 36)?;
        for area in map_areas {
            self.generate_in_area(underground, area)?;
        }
        Ok(())
    }

    fn generate_in_area(&mut self, underground: bool, mut map_area: MapArea) -> H3mResult<()> {
        let mut template_index_set = TemplateIndexSet::new(&self.obstacle_template_list);
        while !template_index_set.is_empty() {
            let template_index = template_index_set.random_index();
            let position_index = self.try_position_obstacle(template_index, &map_area);
            match position_index {
                Some(position_index) => {
                    self.add_obstacle(template_index, position_index, underground, &mut map_area)?
                }
                None => template_index_set.remove(template_index),
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

        if obstacle.index() == 0 {
            obstacle.set_index_usize(self.objects_data.templates.len())?;
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
                obstacle.index(),
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
