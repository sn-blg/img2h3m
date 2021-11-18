use crate::h3m::parsers::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::Surface;
use obstacle_cell::{obstacle_cells, ObstacleCell};
use obstacle_template_list::ObstacleTemplateList;
use rand::Rng;
use std::collections::HashSet;

mod obstacle_cell;
mod obstacle_template_list;

#[derive(Debug)]
struct TemplateIndexSet(HashSet<usize>);

impl TemplateIndexSet {
    fn new(len: usize) -> TemplateIndexSet {
        TemplateIndexSet((0..len).collect())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn random_index(&self) -> usize {
        *self
            .0
            .iter()
            .nth(rand::thread_rng().gen_range(0..self.0.len()))
            .unwrap()
    }

    fn remove(&mut self, index: usize) {
        assert!(self.0.remove(&index));
        println!("template_index_set = {:?}", self);
    }
}

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
        let mut obstacle_cells = obstacle_cells(self.map_size, surfaces)?;
        self.generate_impl(underground, &mut obstacle_cells)
    }

    fn generate_impl(
        &mut self,
        underground: bool,
        obstacle_cells: &mut [ObstacleCell],
    ) -> H3mResult<()> {
        let mut template_index_set = TemplateIndexSet::new(self.obstacle_template_list.len());
        while !template_index_set.is_empty() {
            let template_index = template_index_set.random_index();
            let position = self.try_position_obstacle(template_index, obstacle_cells);
            match position {
                Some(position) => {
                    self.add_obstacle(template_index, position, underground, obstacle_cells)?
                }
                None => template_index_set.remove(template_index),
            }
        }
        Ok(())
    }

    fn try_position_obstacle(
        &self,
        template_index: usize,
        obstacle_cells: &[ObstacleCell],
    ) -> Option<ObstacleCell> {
        let obstacle = self.obstacle_template_list.template(template_index);
        for cell in obstacle_cells {
            //fn is_obstacle_template_fit() -> bool {}
            let is_valid_terrain =
                cell.group() & obstacle.h3m_template().surface_editor_group_mask != 0;

            if is_valid_terrain {
                return Some(*cell);
            }
        }
        None
    }

    fn add_obstacle(
        &mut self,
        template_index: usize,
        position: ObstacleCell,
        underground: bool,
        obstacle_cells: &mut [ObstacleCell],
    ) -> H3mResult<()> {
        let obstacle = self.obstacle_template_list.template_mut(template_index);

        if obstacle.index() == 0 {
            obstacle.set_index(self.objects_data.templates.len());
            self.objects_data
                .templates
                .push(obstacle.h3m_template().clone());
        }

        self.objects_data
            .objects
            .push(H3mObject::without_properties(
                position.column(),
                position.row(),
                underground,
                obstacle.index().try_into()?,
            ));

        obstacle_cells[position.index()].reset_group();

        Ok(())
    }

    pub fn object_templates(&self) -> &[H3mObjectTemplate] {
        &self.objects_data.templates
    }

    pub fn objects(&self) -> &[H3mObject] {
        &self.objects_data.objects
    }
}
