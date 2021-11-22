use crate::h3m::parsers::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::Surface;
use hashbag::HashBag;
use obstacle_cell::{obstacle_cells, ObstacleCell};
use obstacle_template::Delta;
use obstacle_template_list::ObstacleTemplateList;
use rand::Rng;

mod common;
mod obstacle_cell;
mod obstacle_template;
mod obstacle_template_list;

#[derive(Debug)]
struct TemplateIndexSet(HashBag<usize>);

impl TemplateIndexSet {
    fn new(obstacle_template_list: &ObstacleTemplateList) -> TemplateIndexSet {
        let mut index_set = HashBag::new();
        for (index, obstacle) in obstacle_template_list.iter().enumerate() {
            index_set.insert_many(index, obstacle.frequency());
        }
        TemplateIndexSet(index_set)
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
        self.0.take_all(&index).unwrap();
        //println!("template_index_set = {:?}", self);
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
        let mut template_index_set = TemplateIndexSet::new(&self.obstacle_template_list);
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
        let is_valid_neighbour = |cell: &ObstacleCell, delta: &Delta| {
            let row = (cell.row() as usize).checked_sub(delta.row());
            let column = (cell.column() as usize).checked_sub(delta.column());
            match (row, column) {
                (Some(row), Some(column)) => {
                    let neighbour = obstacle_cells[row * self.map_size + column];
                    obstacle.is_valid_terrain(neighbour.terrain_group())
                }
                _ => false,
            }
        };

        'cell_traversal: for position in obstacle_cells {
            for delta in obstacle.shape() {
                if !is_valid_neighbour(position, delta) {
                    continue 'cell_traversal;
                }
            }
            return Some(*position);
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
            obstacle.set_index_usize(self.objects_data.templates.len())?;
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
                obstacle.index(),
            ));

        for delta in obstacle.shape() {
            let row = (position.row() as usize) - delta.row();
            let column = (position.column() as usize) - delta.column();
            obstacle_cells[row * self.map_size + column].reset_terrain_group();
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
