use crate::common::position::generic::Position;

#[derive(Clone, Copy)]
pub struct AreasLayout {
    area_width: usize,
    area_height: usize,

    areas_at_row: usize,
    areas_count: usize,
}

impl AreasLayout {
    pub fn new(map_size: usize, area_width: usize, area_height: usize) -> AreasLayout {
        let ceil_usize = |a: usize, b: usize| (a as f64 / b as f64).ceil() as usize;

        let areas_at_row = ceil_usize(map_size, area_width);
        let areas_count = areas_at_row * ceil_usize(map_size, area_height);

        AreasLayout {
            area_width,
            area_height,
            areas_at_row,
            areas_count,
        }
    }

    pub fn areas_count(&self) -> usize {
        self.areas_count
    }

    pub fn area_index(&self, position: Position<usize>) -> usize {
        let index = (position.row() / self.area_height) * self.areas_at_row
            + (position.column() / self.area_width);

        assert!(index < self.areas_count);
        index
    }
}
