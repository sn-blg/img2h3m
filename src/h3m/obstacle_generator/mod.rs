use crate::h3m::parsers::{DefaultObjectTemplates, H3mObject, H3mObjectTemplate};
use crate::h3m::result::*;
use crate::h3m::Surface;
use obstacle_template_list::{ObstacleTemplate, ObstacleTemplateList};
use rand::Rng;
use std::collections::HashSet;

mod obstacle_template_list;

#[derive(Clone, Copy)]
struct ObstacleCell {
    index: usize,
    row: u8,
    column: u8,
    editor_group: u16, // 0 means no obstacles to place
}

impl ObstacleCell {
    fn try_new(index: usize, map_size: usize) -> H3mResult<ObstacleCell> {
        Ok(ObstacleCell {
            index,
            row: (index / map_size).try_into()?,
            column: (index % map_size).try_into()?,
            editor_group: 0,
        })
    }

    fn index(&self) -> usize {
        self.index
    }

    fn row(&self) -> u8 {
        self.row
    }

    fn column(&self) -> u8 {
        self.column
    }

    fn editor_group(&self) -> u16 {
        self.editor_group
    }

    fn set_editor_group(&mut self, editor_group: u16) {
        self.editor_group = editor_group;
    }

    fn reset_editor_group(&mut self) {
        self.editor_group = 0;
    }
}

fn obstacle_cells(map_size: usize, surfaces: &[Option<Surface>]) -> H3mResult<Vec<ObstacleCell>> {
    let mut obstacle_cells = Vec::with_capacity(surfaces.len());

    for (index, surface) in surfaces.iter().enumerate() {
        let mut cell = ObstacleCell::try_new(index, map_size)?;

        if let Some(surface) = surface {
            if surface.obstacle {
                cell.set_editor_group(1 << (surface.terrain.code() as u16));
            }
        }

        obstacle_cells.push(cell)
    }

    Ok(obstacle_cells)
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
        let mut template_index_set: HashSet<usize> =
            (0..self.obstacle_template_list.len()).collect();

        while !template_index_set.is_empty() {
            let template_index = *template_index_set
                .iter()
                .nth(rand::thread_rng().gen_range(0..template_index_set.len()))
                .unwrap();

            let obstacle = self.obstacle_template_list.template(template_index);
            let position = ObstacleGenerator::try_position_obstacle(obstacle, obstacle_cells);
            match position {
                Some(position) => {
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
                    obstacle_cells[position.index()].reset_editor_group();
                }
                None => {
                    assert!(template_index_set.remove(&template_index));
                    println!("template_index_set = {:?}", template_index_set);
                }
            }
        }
        Ok(())
    }

    fn try_position_obstacle(
        obstacle: &ObstacleTemplate,
        obstacle_cells: &[ObstacleCell],
    ) -> Option<ObstacleCell> {
        for cell in obstacle_cells {
            let is_valid_terrain =
                cell.editor_group() & obstacle.h3m_template().surface_editor_group_mask != 0;

            if is_valid_terrain {
                return Some(*cell);
            }
        }
        None
    }

    //fn is_obstacle_template_fit() -> bool {}

    pub fn object_templates(&self) -> &[H3mObjectTemplate] {
        &self.objects_data.templates
    }

    pub fn objects(&self) -> &[H3mObject] {
        &self.objects_data.objects
    }
}
