use super::areas_layout::SquareAreasLayout;
use crate::common::position::generic::{Position, SignedDeltaPos};
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
        if max_sparsity > 0 {
            let max_sparsity_square = usize::pow(max_sparsity, 2);

            let areas = self
                .data
                .entry(template_index)
                .or_insert(Areas::new(max_sparsity_square, self.map_size));

            assert!(areas.layout.area_side() == max_sparsity_square);

            areas.add_position(position);
        }
    }

    pub fn verify_position(
        &self,
        template_index: usize,
        sparsity: usize,
        position: Position<usize>,
    ) -> bool {
        if sparsity == 0 {
            true
        } else {
            if let Some(areas) = self.data.get(&template_index) {
                self.verify_in_areas(usize::pow(sparsity, 2), position, areas)
            } else {
                true
            }
        }
    }

    fn verify_in_areas(
        &self,
        sparsity_square: usize,
        position: Position<usize>,
        areas: &Areas,
    ) -> bool {
        assert!(sparsity_square > 0);
        assert!(sparsity_square <= areas.layout.area_side());

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
                if !self.verify_in_area(sparsity_square, position, &areas.data[area_index]) {
                    return false;
                }
            }
        }
        true
    }

    fn verify_in_area(
        &self,
        sparsity_square: usize,
        position: Position<usize>,
        area: &Area,
    ) -> bool {
        for area_position in area.positions.iter() {
            let distance_square = distance_square(area_position, &position);
            if distance_square < sparsity_square {
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
        let sparsity = 1;

        let map_len = map_size.pow(2);

        for index in 0..map_len {
            let position = Position::from_index(map_size, index);

            assert!(sparsity_validator.verify_position(template_index, sparsity, position));
            sparsity_validator.add_position(template_index, max_sparsity, position);
        }

        let data = &sparsity_validator.data;
        assert_eq!(data.keys().len(), 1);
        assert!(data.contains_key(&template_index));

        let areas = data.get(&template_index).unwrap();

        assert_eq!(areas.layout.area_side(), max_sparsity);
        assert_eq!(areas.layout.areas_at_row(), map_size);
        assert_eq!(areas.layout.areas_at_column(), map_size);
        assert_eq!(areas.layout.areas_count(), map_len);

        assert_eq!(areas.data.len(), map_len);
        for (index, area) in areas.data.iter().enumerate() {
            assert_eq!(area.positions.len(), 1);

            let position = area.positions[0];
            assert_eq!(index, position.index(map_size));
        }
    }
}
