use crate::h3m::obstacle_generator::obstacle_map::Position;

#[derive(Clone, Copy)]
pub struct OverlappingObstacle {
    filename: &'static str,
    base_position: Position<usize>,
}

impl OverlappingObstacle {
    pub fn new(filename: &'static str, base_position: Position<usize>) -> Self {
        OverlappingObstacle {
            filename,
            base_position,
        }
    }
}

#[derive(Clone)]
pub enum LocatedObstacle {
    Common,
    Overlapping(Vec<OverlappingObstacle>),
}
