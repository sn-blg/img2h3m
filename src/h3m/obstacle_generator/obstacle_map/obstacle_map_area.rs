use super::areas_layout::AreasLayout;
use crate::common::position::Position;

#[derive(Clone)]
pub struct ObstacleMapArea(Vec<usize>);

impl ObstacleMapArea {
    fn new() -> ObstacleMapArea {
        ObstacleMapArea(Vec::new())
    }

    pub fn indexes(&self) -> &Vec<usize> {
        &self.0
    }
}

pub fn make_areas(map_size: usize, area_width: usize, area_height: usize) -> Vec<ObstacleMapArea> {
    let areas_layout = AreasLayout::new(map_size, area_width, area_height);
    let mut areas = vec![ObstacleMapArea::new(); areas_layout.areas_count()];

    for cell_index in 0..(map_size * map_size) {
        let cell_position = Position::from_index(map_size, cell_index);
        let area_index = areas_layout.area_index(cell_position);
        areas[area_index].0.push(cell_index);
    }
    areas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_obstacle_map_areas_test() {
        let areas = make_areas(5, 3, 2);
        assert_eq!(areas.len(), 6);

        assert_eq!(areas[0].0, [0, 1, 2, 5, 6, 7]);
        assert_eq!(areas[1].0, [3, 4, 8, 9]);
        assert_eq!(areas[2].0, [10, 11, 12, 15, 16, 17]);
        assert_eq!(areas[3].0, [13, 14, 18, 19]);
        assert_eq!(areas[4].0, [20, 21, 22]);
        assert_eq!(areas[5].0, [23, 24]);
    }
}
