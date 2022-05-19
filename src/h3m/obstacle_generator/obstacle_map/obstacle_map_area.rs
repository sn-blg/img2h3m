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
    let ceil = |a: usize, b: usize| (a as f64 / b as f64).ceil() as usize;

    let areas_at_row = ceil(map_size, area_width);
    let areas_count = areas_at_row * ceil(map_size, area_height);

    let mut areas = vec![ObstacleMapArea::new(); areas_count];

    for map_index in 0..(map_size * map_size) {
        let row = map_index / map_size;
        let column = map_index % map_size;

        let area_index = (row / area_height) * areas_at_row + (column / area_width);

        areas[area_index].0.push(map_index);
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
