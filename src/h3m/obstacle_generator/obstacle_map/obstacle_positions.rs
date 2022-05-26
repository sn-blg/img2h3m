use super::areas_layout::AreasLayout;
use crate::common::position::generic::Position;
use std::collections::HashMap;

fn delta_square(a: usize, b: usize) -> usize {
    usize::pow(if a > b { a - b } else { b - a }, 2)
}

fn distance_square(a: &Position<usize>, b: &Position<usize>) -> usize {
    delta_square(a.row(), b.row()) + delta_square(a.column(), b.column())
}

#[derive(Clone)]
struct Area {
    positions: Vec<Position<usize>>,
}

impl Area {
    fn new() -> Area {
        Area {
            positions: Vec::new(),
        }
    }
}

struct Areas {
    layout: AreasLayout,
    data: Vec<Area>,
}

impl Areas {
    fn new(sparsity: usize, map_size: usize) -> Areas {
        let layout = AreasLayout::new(map_size, sparsity, sparsity);
        Areas {
            layout,
            data: vec![Area::new(); layout.areas_count()],
        }
    }

    fn add(&mut self, position: Position<usize>) {
        let area_index = self.layout.area_index(position);
        self.data[area_index].positions.push(position);
    }
}

pub struct ObstaclePositions {
    map_size: usize,
    data: HashMap<usize, Areas>,
}

impl ObstaclePositions {
    pub fn new(map_size: usize) -> ObstaclePositions {
        ObstaclePositions {
            map_size,
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, template_index: usize, sparsity: usize, position: Position<usize>) {
        self.data
            .entry(template_index)
            .or_insert(Areas::new(sparsity, self.map_size))
            .add(position);
    }

    pub fn min_distance_square(
        &self,
        template_index: usize,
        position: Position<usize>,
    ) -> Option<usize> {
        // todo:
        None
        /*
        self.0
            .get(&template_index)?
            .iter()
            .map(|p| distance_square(p, &position))
            .min()
            */
    }
}
