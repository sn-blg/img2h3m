use super::obstacle_template_list::ObstacleTemplateList;
use hashbag::HashBag;
use rand::Rng;

pub struct TemplateIndexSet(HashBag<usize>);

impl TemplateIndexSet {
    pub fn new(obstacle_template_list: &ObstacleTemplateList) -> TemplateIndexSet {
        let mut index_set = HashBag::new();
        for (index, obstacle) in obstacle_template_list.iter().enumerate() {
            index_set.insert_many(index, obstacle.frequency());
        }
        TemplateIndexSet(index_set)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn random_index(&self) -> usize {
        *self
            .0
            .iter()
            .nth(rand::thread_rng().gen_range(0..self.0.len()))
            .unwrap()
    }

    pub fn remove(&mut self, index: usize) {
        self.0.take_all(&index).unwrap();
    }
}
