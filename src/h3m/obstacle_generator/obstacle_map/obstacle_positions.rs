use crate::common::position::generic::Position;
use std::collections::HashMap;

pub struct ObstaclePositions(HashMap<usize, Vec<Position<usize>>>);

impl ObstaclePositions {
    pub fn new() -> ObstaclePositions {
        ObstaclePositions(HashMap::new())
    }

    pub fn add(&mut self, template_index: usize, position: Position<usize>) {
        self.0
            .entry(template_index)
            .or_insert(Vec::new())
            .push(position);
    }

    pub fn min_distance_square(&self, template_index: usize, position: Position<usize>) -> usize {
        // todo!:
        0
    }
}
