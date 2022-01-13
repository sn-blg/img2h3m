use super::obstacle_template_list::ObstacleTemplateList;
use crate::common::index_multiset::IndexMultiset;

pub struct TemplateIndexSet(IndexMultiset<usize>);

impl TemplateIndexSet {
    pub fn new(obstacle_template_list: &ObstacleTemplateList) -> TemplateIndexSet {
        let mut index_set = IndexMultiset::new();
        for (index, obstacle) in obstacle_template_list.iter().enumerate() {
            index_set.add_index(index, obstacle.frequency());
        }
        TemplateIndexSet(index_set)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn random_index(&self) -> usize {
        self.0.random_index().unwrap()
    }

    pub fn remove_index(&mut self, index: usize) {
        self.0.remove_index(index).unwrap();
    }
}
