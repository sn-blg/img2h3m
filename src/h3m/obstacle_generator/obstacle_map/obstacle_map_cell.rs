use crate::common::position::generic::Position;
use crate::h3m::terrain_map::MapCell;
use crate::h3m::terrain_map::TerrainVisibleType;
use crate::h3m::Terrain;

pub type NeighborhoodSameRelation = [bool; 8];

#[derive(Clone, Copy)]
pub struct ObstacleMapCell {
    position: Position<u8>,
    map_cell: Option<MapCell>,
    template_index: Option<usize>,
    terrain_group: u16, // terrain editor group, 0 means no obstacles to place
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
            template_index: None,
            terrain_group,
            neighborhood_same_relation,
        }
    }

    pub fn set_template(&mut self, template_index: usize) {
        assert!(self.template_index.is_none());
        assert!(self.terrain_group != 0);
        assert!(self.map_cell.is_some());

        self.template_index = Some(template_index);
        self.terrain_group = 0;
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

    pub fn neighborhood_same_relation(&self) -> &NeighborhoodSameRelation {
        &self.neighborhood_same_relation
    }
}
