use crate::h3m::result::*;
use crate::h3m::Surface;

#[derive(Clone, Copy)]
pub struct ObstacleCell {
    index: usize,
    row: u8,
    column: u8,
    group: u16, // surface editor group, 0 means no obstacles to place
}

impl ObstacleCell {
    fn try_new(index: usize, map_size: usize) -> H3mResult<ObstacleCell> {
        Ok(ObstacleCell {
            index,
            row: (index / map_size).try_into()?,
            column: (index % map_size).try_into()?,
            group: 0,
        })
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn column(&self) -> u8 {
        self.column
    }

    pub fn group(&self) -> u16 {
        self.group
    }

    pub fn reset_group(&mut self) {
        self.group = 0;
    }
}

pub fn obstacle_cells(
    map_size: usize,
    surfaces: &[Option<Surface>],
) -> H3mResult<Vec<ObstacleCell>> {
    let mut obstacle_cells = Vec::with_capacity(surfaces.len());

    for (index, surface) in surfaces.iter().enumerate() {
        let mut cell = ObstacleCell::try_new(index, map_size)?;

        if let Some(surface) = surface {
            if surface.obstacle {
                cell.group = 1 << (surface.terrain.code() as u16);
            }
        }

        obstacle_cells.push(cell)
    }

    Ok(obstacle_cells)
}
