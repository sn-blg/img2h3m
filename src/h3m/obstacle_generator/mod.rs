use crate::h3m::parser::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::terrain_map::TerrainMap;
use filename_to_template_index_map::FilenameToTemplateIndexMap;
use obstacle_map::{ObstacleMap, ObstacleMapArea};
use obstacle_template_list::ObstacleTemplateList;
use rand::rngs::ThreadRng;
use template_index_set::TemplateIndexSet;

mod common;
mod filename_to_template_index_map;
mod obstacle_map;
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
        let mut obstacle_map = ObstacleMap::new(terrain_map)?;

        let filename_to_template_index_map =
            FilenameToTemplateIndexMap::new(&self.obstacle_template_list);

        let map_size = terrain_map.size();
        let areas = obstacle_map::make_areas(map_size, 36, 36);

        for sparsity_penalty in [0, 1, 2, 4, 8, 16, 32] {
            obstacle_map.set_sparsity_penalty(sparsity_penalty);

            let template_index_set = TemplateIndexSet::new(
                obstacle_map.generalized_terrain_group(),
                &self.obstacle_template_list,
            );

            for area in areas.iter().rev() {
                self.generate_in_area(
                    terrain_map.underground(),
                    &filename_to_template_index_map,
                    template_index_set.clone(),
                    area,
                    &mut obstacle_map,
                )?;
            }

            if obstacle_map.first_position_to_place_obstacle().is_none() {
                break;
            }
        }

        if let Some(position) = obstacle_map.first_position_to_place_obstacle() {
            return Err(H3mError::Internal(InternalError::new(format!(
                "failed to place obstacle in position (row: {}, column: {}).",
                position.row(),
                position.column()
            ))));
        }

        Ok(())
    }

    fn generate_in_area(
        &mut self,
        underground: bool,
        filename_to_template_index_map: &FilenameToTemplateIndexMap,
        mut template_index_set: TemplateIndexSet,
        area: &ObstacleMapArea,
        obstacle_map: &mut ObstacleMap,
    ) -> H3mResult<()> {
        while !template_index_set.is_empty() {
            let template_index = template_index_set.random_index(&mut self.rng);
            let position_index = self.try_position_obstacle(
                template_index,
                filename_to_template_index_map,
                area,
                obstacle_map,
            );
            match position_index {
                Some(position_index) => {
                    self.add_obstacle(template_index, position_index, underground, obstacle_map)?
                }
                None => template_index_set.remove_index(template_index),
            }
        }
        Ok(())
    }

    fn try_position_obstacle(
        &mut self,
        template_index: usize,
        filename_to_template_index_map: &FilenameToTemplateIndexMap,
        area: &ObstacleMapArea,
        obstacle_map: &ObstacleMap,
    ) -> Option<usize> {
        let obstacle = self.obstacle_template_list.template(template_index);
        obstacle_map.try_position_obstacle(
            area,
            template_index,
            filename_to_template_index_map,
            obstacle,
            &mut self.rng,
        )
    }

    fn add_obstacle(
        &mut self,
        template_index: usize,
        position_index: usize,
        underground: bool,
        obstacle_map: &mut ObstacleMap,
    ) -> H3mResult<()> {
        let obstacle = self.obstacle_template_list.template_mut(template_index);

        if obstacle.h3m_template_index() == 0 {
            obstacle.set_h3m_template_index(self.objects_data.templates.len())?;
            self.objects_data
                .templates
                .push(obstacle.h3m_template().clone());
        }

        let position = obstacle_map.position(position_index);
        self.objects_data
            .objects
            .push(H3mObject::without_properties(
                position.column(),
                position.row(),
                underground,
                obstacle.h3m_template_index(),
            ));

        obstacle_map.add_obstacle(position_index, template_index, obstacle);

        Ok(())
    }

    pub fn object_templates(&self) -> &[H3mObjectTemplate] {
        &self.objects_data.templates
    }

    pub fn objects(&self) -> &[H3mObject] {
        &self.objects_data.objects
    }
}
