use super::obstacle_template_list::ObstacleTemplateList;
use std::collections::HashMap;

pub struct FilenameToTemplateIndexMap(HashMap<&'static str, usize>);

impl FilenameToTemplateIndexMap {
    pub fn new(obstacle_template_list: &ObstacleTemplateList) -> FilenameToTemplateIndexMap {
        let mut map = HashMap::new();
        for (index, obstacle) in obstacle_template_list.iter().enumerate() {
            map.insert(obstacle.filename(), index);
        }
        FilenameToTemplateIndexMap(map)
    }

    pub fn template_index(&self, filename: &'static str) -> Option<usize> {
        self.0.get(filename).cloned()
    }
}
