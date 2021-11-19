use crate::h3m::result::*;
use crate::h3m::Surface;

#[derive(Clone, Copy)]
pub struct ObstacleCell {
    row: u8,
    column: u8,
    terrain_group: u16, // surface editor group, 0 means no obstacles to place
}

impl ObstacleCell {
    fn try_new(index: usize, map_size: usize) -> H3mResult<ObstacleCell> {
        Ok(ObstacleCell {
            row: (index / map_size).try_into()?,
            column: (index % map_size).try_into()?,
            terrain_group: 0,
        })
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn column(&self) -> u8 {
        self.column
    }

    pub fn terrain_group(&self) -> u16 {
        self.terrain_group
    }

    pub fn reset_terrain_group(&mut self) {
        self.terrain_group = 0;
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
                cell.terrain_group = 1 << (surface.terrain.code() as u16);
            }
        }

        obstacle_cells.push(cell)
    }

    Ok(obstacle_cells)
}
