use super::located_obstacle::{LocatedObstacle, OverlappingObstacle};
use crate::common::position::generic::Position;
use crate::h3m::obstacle_generator::obstacle_template::ObstacleTemplate;
use crate::h3m::terrain_map::MapCell;
use crate::h3m::terrain_map::TerrainVisibleType;
use crate::h3m::Terrain;

pub type NeighborhoodSameRelation = [bool; 8];

#[derive(Clone)]
pub struct ObstacleMapCell {
    position: Position<u8>,
    map_cell: Option<MapCell>,
    terrain_group: u16, // terrain editor group, 0 means no obstacles to place
    located_obstacle: Option<LocatedObstacle>,
    neighborhood_same_relation: NeighborhoodSameRelation,
}

pub fn calc_terrain(map_cell: &MapCell) -> Terrain {
    match map_cell.tile().terrain_visible_type() {
        TerrainVisibleType::Diff(terrain) => terrain,
        TerrainVisibleType::DiffMixed(terrain, _) => terrain,
        _ => map_cell.surface().terrain,
    }
}

fn calc_terrain_group(map_cell: &Option<MapCell>) -> u16 {
    if let Some(map_cell) = map_cell {
        if map_cell.surface().obstacle {
            return calc_terrain(map_cell).group();
        }
    }
    0
}

impl ObstacleMapCell {
    pub fn new(
        row: u8,
        column: u8,
        map_cell: Option<MapCell>,
        neighborhood_same_relation: NeighborhoodSameRelation,
    ) -> ObstacleMapCell {
        let terrain_group = calc_terrain_group(&map_cell);
        ObstacleMapCell {
            position: Position::new(row, column),
            map_cell,
            terrain_group,
            located_obstacle: None,
            neighborhood_same_relation,
        }
    }

    pub fn set_obstacle(&mut self, obstacle: &ObstacleTemplate, base_position: Position<usize>) {
        assert!(self.terrain_group != 0);
        assert!(self.map_cell.is_some());

        if obstacle.may_overlap() {
            let overlapping_obstacle = OverlappingObstacle::new(obstacle.filename(), base_position);
            match self.located_obstacle {
                Some(LocatedObstacle::Common) => panic!(
                    "Invalid located obstacle state in position (row: {}, column: {}).",
                    self.position.row(),
                    self.position.column()
                ),
                Some(LocatedObstacle::Overlapping(ref mut vec)) => vec.push(overlapping_obstacle),
                None => {
                    self.located_obstacle =
                        Some(LocatedObstacle::Overlapping(vec![overlapping_obstacle]));
                }
            }
        } else {
            self.located_obstacle = Some(LocatedObstacle::Common);
        }
    }

    pub fn position(&self) -> Position<u8> {
        self.position
    }

    pub fn map_cell(&self) -> &Option<MapCell> {
        &self.map_cell
    }

    pub fn terrain_group(&self) -> u16 {
        self.terrain_group
    }

    pub fn located_obstacle(&self) -> &Option<LocatedObstacle> {
        &self.located_obstacle
    }

    pub fn neighborhood_same_relation(&self) -> &NeighborhoodSameRelation {
        &self.neighborhood_same_relation
    }

    pub fn need_place_obstacle(&self) -> bool {
        (self.terrain_group != 0) && (self.located_obstacle.is_none())
    }
}
