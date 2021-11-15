use libflate::gzip::{Decoder, Encoder};
use object_templates_set::ObjectTemplatesSet;
use parsers::*;
use rand::Rng;
use result::*;
use std::io::{self, Read, Write};
pub use surface::Surface;

mod object_templates_set;
mod parsers;
pub mod result;
mod surface;

struct ObjectsData {
    object_templates_set: ObjectTemplatesSet,
    object_templates: Vec<H3mObjectTemplate>,
    objects: Vec<H3MObject>,
}

impl ObjectsData {
    fn new(default_object_templates: &DefaultObjectTemplates) -> ObjectsData {
        let mut objects_data = ObjectsData {
            object_templates_set: ObjectTemplatesSet::new(),
            object_templates: vec![
                default_object_templates[0].clone(),
                default_object_templates[1].clone(),
            ],
            objects: Vec::new(),
        };

        objects_data
            .object_templates
            .push(objects_data.object_templates_set.object_template().clone());

        objects_data
    }

    fn put_some_object(&mut self, column: u8, row: u8, underground: bool) {
        self.objects
            .push(H3MObject::without_properties(column, row, underground, 2));
    }
}

pub struct H3m {
    info: H3mInfo,
    raw_map: Vec<u8>,
    objects_data: Option<ObjectsData>,
}

impl H3m {
    pub fn load<R: io::Read>(input: R) -> H3mResult<H3m> {
        let mut decoder = Decoder::new(input)?;
        let mut raw_map = Vec::new();
        decoder.read_to_end(&mut raw_map)?;

        Ok(H3m {
            info: parse(&raw_map)?,
            raw_map,
            objects_data: None,
        })
    }

    pub fn save<W: io::Write>(&self, output: W) -> H3mResult<()> {
        let mut encoder = Encoder::new(output)?;

        if let Some(objects_data) = &self.objects_data {
            encoder.write_all(&self.raw_map[..self.info.objects_templates_offset])?;
            write_object_templates(&objects_data.object_templates, &mut encoder)?;
            write_objects(&objects_data.objects, &mut encoder)?;
            encoder.write_all(&[0u8; 124])?;
        } else {
            encoder.write_all(&self.raw_map)?;
        }

        encoder.finish().into_result()?;
        Ok(())
    }

    pub fn map_size(&self) -> usize {
        self.info.map_size
    }

    pub fn set_surfaces(
        &mut self,
        underground: bool,
        surfaces: &[Option<Surface>],
    ) -> H3mResult<()> {
        for (index, surface) in surfaces.iter().enumerate() {
            if let Some(surface) = surface {
                self.set_surface_by_index(index, underground, *surface)?;
            }
        }
        Ok(())
    }

    pub fn set_obstacles(&mut self, underground: bool, obstacles: &[bool]) -> H3mResult<()> {
        if self.objects_data.is_none() {
            self.objects_data = Some(ObjectsData::new(&self.info.default_object_templates));
        }

        for (index, &obstacle) in obstacles.iter().enumerate() {
            if obstacle {
                let column = (index % self.map_size()).try_into()?;
                let row = (index / self.map_size()).try_into()?;
                self.objects_data
                    .as_mut()
                    .unwrap()
                    .put_some_object(column, row, underground);
            }
        }

        Ok(())
    }

    fn set_surface_by_index(
        &mut self,
        index: usize,
        underground: bool,
        surface: Surface,
    ) -> H3mResult<()> {
        let land_length = self.map_size() * self.map_size();

        if index >= land_length {
            return Err(H3mError::Parameter(ParameterError::new(format!(
                "Invalid surface index: {}, land length: {}.",
                index, land_length
            ))));
        }

        let surface_cell_offset = if underground {
            self.info.underground_offset.ok_or_else(|| {
                H3mError::Parameter(ParameterError::new(
                    "Can't set underground surface, input map has not underground.",
                ))
            })?
        } else {
            self.info.land_offset
        } + index * SURFACE_CELL_SIZE;

        let surface_picture_type = self.surface_picture_type(surface);

        let surface_cell =
            &mut self.raw_map[surface_cell_offset..surface_cell_offset + SURFACE_CELL_SIZE];

        surface_cell[0] = surface.code();
        surface_cell[1] = surface_picture_type;

        Ok(())
    }

    fn surface_picture_type(&self, surface: Surface) -> u8 {
        let range = match surface {
            Surface::Dirt => 21..=44,
            Surface::Sand => 0..=23,
            Surface::Grass => 49..=72,
            Surface::Snow => 49..=72,
            Surface::Swamp => 49..=72,
            Surface::Rough => 49..=72,
            Surface::Subterranean => 49..=72,
            Surface::Lava => 49..=72,
            Surface::Highland => 77..=117,
            Surface::Wasteland => 77..=117,
            Surface::Water => 21..=32,
            Surface::Rock => 0..=7,
        };
        rand::thread_rng().gen_range(range)
    }
}
