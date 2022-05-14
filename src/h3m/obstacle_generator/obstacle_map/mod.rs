use obstacle_map_cell::ObstacleMapCell;

mod obstacle_map_cell;

pub struct ObstacleMap {
    size: usize,
    cells: Vec<ObstacleMapCell>,
}
