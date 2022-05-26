use crate::common::position::generic::Position;

#[derive(Clone, Copy)]
pub struct AreasLayout {
    area_width: usize,
    area_height: usize,

    areas_at_row: usize,
    areas_at_column: usize,

    areas_count: usize,
}

impl AreasLayout {
    pub fn new(map_size: usize, area_width: usize, area_height: usize) -> AreasLayout {
        let ceil_usize = |a: usize, b: usize| (a as f64 / b as f64).ceil() as usize;

        let areas_at_row = ceil_usize(map_size, area_width);
        let areas_at_column = ceil_usize(map_size, area_height);

        let areas_count = areas_at_row * areas_at_column;

        AreasLayout {
            area_width,
            area_height,
            areas_at_row,
            areas_at_column,
            areas_count,
        }
    }

    pub fn area_width(&self) -> usize {
        self.area_width
    }

    pub fn area_height(&self) -> usize {
        self.area_height
    }

    pub fn areas_at_row(&self) -> usize {
        self.areas_at_row
    }

    pub fn areas_at_column(&self) -> usize {
        self.areas_at_column
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

#[derive(Clone, Copy)]
pub struct SquareAreasLayout(AreasLayout);

impl SquareAreasLayout {
    pub fn new(map_size: usize, area_side: usize) -> SquareAreasLayout {
        SquareAreasLayout(AreasLayout::new(map_size, area_side, area_side))
    }

    pub fn area_side(&self) -> usize {
        assert!(self.0.area_width() == self.0.area_height());
        self.0.area_width()
    }

    pub fn areas_at_row(&self) -> usize {
        self.0.areas_at_row()
    }

    pub fn areas_at_column(&self) -> usize {
        self.0.areas_at_column()
    }

    pub fn areas_count(&self) -> usize {
        self.0.areas_count()
    }

    pub fn area_index(&self, position: Position<usize>) -> usize {
        self.0.area_index(position)
    }
}
