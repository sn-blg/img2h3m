use super::areas_layout::SquareAreasLayout;
use crate::common::position::generic::{Position, SignedDeltaPos};
use std::collections::HashMap;

fn delta_square(a: usize, b: usize) -> usize {
    (if a > b { a - b } else { b - a }).pow(2)
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
    layout: SquareAreasLayout,
    data: Vec<Area>,
}

impl Areas {
    fn new(area_side: usize, map_size: usize) -> Areas {
        let layout = SquareAreasLayout::new(map_size, area_side);
        Areas {
            layout,
            data: vec![Area::new(); layout.areas_count()],
        }
    }

    fn add_position(&mut self, position: Position<usize>) {
        let area_index = self.layout.area_index(position);
        self.data[area_index].positions.push(position);
    }
}

pub struct SparsityValidator {
    map_size: usize,
    data: HashMap<usize, Areas>,
}

impl SparsityValidator {
    pub fn new(map_size: usize) -> SparsityValidator {
        SparsityValidator {
            map_size,
            data: HashMap::new(),
        }
    }

    pub fn add_position(
        &mut self,
        template_index: usize,
        max_sparsity: usize,
        position: Position<usize>,
    ) {
        if max_sparsity < 2 {
            return;
        }

        let area_side = (max_sparsity as f64).sqrt().ceil() as usize;
        assert!(area_side >= 2);

        let areas = self
            .data
            .entry(template_index)
            .or_insert(Areas::new(area_side, self.map_size));

        assert!(areas.layout.area_side() == area_side);

        areas.add_position(position);
    }

    pub fn verify_position(
        &self,
        template_index: usize,
        sparsity: usize,
        position: Position<usize>,
    ) -> bool {
        if sparsity < 2 {
            true
        } else {
            if let Some(areas) = self.data.get(&template_index) {
                self.verify_in_areas(sparsity, position, areas)
            } else {
                true
            }
        }
    }

    fn verify_in_areas(&self, sparsity: usize, position: Position<usize>, areas: &Areas) -> bool {
        assert!(sparsity >= 2);
        assert!(sparsity <= areas.layout.area_side().pow(2));

        let areas_at_row = areas.layout.areas_at_row();
        let areas_at_column = areas.layout.areas_at_column();

        let central_area_index = areas.layout.area_index(position);
        let central_area_position = Position::from_index(areas_at_row, central_area_index);

        for (delta_row, delta_column) in [
            (0, 0),
            (-1, 0),
            (0, -1),
            (0, 1),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ] {
            let area_position = central_area_position.checked_apply(
                areas_at_row,
                areas_at_column,
                &SignedDeltaPos::new(delta_row, delta_column),
            );
            if let Some(area_position) = area_position {
                let area_index = area_position.index(areas_at_row);
                if !self.verify_in_area(sparsity, position, &areas.data[area_index]) {
                    return false;
                }
            }
        }
        true
    }

    fn verify_in_area(&self, sparsity: usize, position: Position<usize>, area: &Area) -> bool {
        for area_position in area.positions.iter() {
            let distance_square = distance_square(area_position, &position);
            if distance_square < sparsity {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_size_sparsity_test() {
        let map_size = 5;

        let mut sparsity_validator = SparsityValidator::new(map_size);

        let template_index = 42;
        let max_sparsity = 1;
        let sparsity = max_sparsity;

        let map_len = map_size.pow(2);

        for index in 0..map_len {
            let position = Position::from_index(map_size, index);

            assert!(sparsity_validator.verify_position(template_index, sparsity, position));
            sparsity_validator.add_position(template_index, max_sparsity, position);
        }

        let data = &sparsity_validator.data;
        assert_eq!(data.keys().len(), 0);
    }

    #[test]
    fn sparsity_test() {
        let map_size = 5;

        let mut sparsity_validator = SparsityValidator::new(map_size);

        let template_index = 42;
        let max_sparsity = 2;
        let sparsity = max_sparsity;

        let map_len = map_size.pow(2);

        for index in 0..map_len {
            let position = Position::from_index(map_size, index);

            let is_valid = sparsity_validator.verify_position(template_index, sparsity, position);
            assert_eq!(is_valid, index % 2 == 0);

            if is_valid {
                sparsity_validator.add_position(template_index, max_sparsity, position);
            }
        }

        let data = &sparsity_validator.data;
        assert_eq!(data.keys().len(), 1);
        assert!(data.contains_key(&template_index));

        let areas = data.get(&template_index).unwrap();

        assert_eq!(areas.layout.area_side(), 2);
        assert_eq!(areas.layout.areas_at_row(), 3);
        assert_eq!(areas.layout.areas_at_column(), 3);
        assert_eq!(areas.layout.areas_count(), 9);

        assert_eq!(areas.data.len(), 9);

        assert_eq!(areas.data[0].positions, [Position::new(0, 0), Position::new(1, 1)]);
        assert_eq!(areas.data[1].positions, [Position::new(0, 2), Position::new(1, 3)]);
        assert_eq!(areas.data[2].positions, [Position::new(0, 4)]);
        assert_eq!(areas.data[3].positions, [Position::new(2, 0), Position::new(3, 1)]);
        assert_eq!(areas.data[4].positions, [Position::new(2, 2), Position::new(3, 3)]);
        assert_eq!(areas.data[5].positions, [Position::new(2, 4)]);
        assert_eq!(areas.data[6].positions, [Position::new(4, 0)]);
        assert_eq!(areas.data[7].positions, [Position::new(4, 2)]);
        assert_eq!(areas.data[8].positions, [Position::new(4, 4)]);
    }
}
