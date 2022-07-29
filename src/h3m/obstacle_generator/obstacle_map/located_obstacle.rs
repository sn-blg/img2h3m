use crate::common::position::Position;

#[derive(Clone, Copy)]
pub struct OverlappingObstacle {
    filename: &'static str,
    base_position: Position,
}

impl OverlappingObstacle {
    pub fn new(filename: &'static str, base_position: Position) -> Self {
        OverlappingObstacle {
            filename,
            base_position,
        }
    }

    pub fn filename(&self) -> &'static str {
        self.filename
    }

    pub fn base_position(&self) -> &Position {
        &self.base_position
    }
}

#[derive(Clone)]
pub enum LocatedObstacle {
    Common,
    Overlapping(Vec<OverlappingObstacle>),
}
