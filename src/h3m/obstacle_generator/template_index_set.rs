use super::obstacle_template_list::ObstacleTemplateList;
use crate::common::index_multiset::IndexMultiset;
use rand::rngs::ThreadRng;

#[derive(Clone)]
pub struct TemplateIndexSet(IndexMultiset<usize>);

impl TemplateIndexSet {
    pub fn new(
        generalized_terrain_group: u16,
        obstacle_template_list: &ObstacleTemplateList,
    ) -> TemplateIndexSet {
        let mut index_set = IndexMultiset::new();
        for (index, obstacle) in obstacle_template_list.iter().enumerate() {
            if obstacle.is_valid_terrain(generalized_terrain_group) {
                index_set.add_index(index, obstacle.frequency());
            }
        }
        TemplateIndexSet(index_set)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn random_index(&self, rng: &mut ThreadRng) -> usize {
        self.0.random_index(rng).unwrap()
    }

    pub fn remove_index(&mut self, index: usize) {
        self.0.remove_index(index).unwrap();
    }
}
