use crate::common::position::generic::Position;
use std::collections::HashMap;

pub struct ObstaclePositions(HashMap<usize, Vec<Position<usize>>>);

fn delta_square(a: usize, b: usize) -> usize {
    usize::pow(if a > b { a - b } else { b - a }, 2)
}

fn distance_square(a: &Position<usize>, b: &Position<usize>) -> usize {
    delta_square(a.row(), b.row()) + delta_square(a.column(), b.column())
}

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

    pub fn min_distance_square(
        &self,
        template_index: usize,
        position: Position<usize>,
    ) -> Option<usize> {
        self.0
            .get(&template_index)?
            .iter()
            .map(|p| distance_square(p, &position))
            .min()
    }
}
