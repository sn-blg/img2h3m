use crate::common::position::generic::Position;
use crate::h3m::terrain_map::MapCell;
use crate::h3m::terrain_map::TerrainVisibleType;

#[derive(Clone, Copy)]
pub struct ObstacleMapCell {
    position: Position<u8>,
    map_cell: Option<MapCell>,
    template_index: Option<usize>,
    terrain_group: u16, // terrain editor group, 0 means no obstacles to place
}

fn calc_terrain_group(map_cell: &Option<MapCell>) -> u16 {
    if let Some(map_cell) = map_cell {
        if map_cell.surface().obstacle {
            let tile = map_cell.tile();

            let terrain = match tile.terrain_visible_type() {
                TerrainVisibleType::Diff(terrain) => terrain,
                TerrainVisibleType::DiffMixed(terrain) => terrain,
                _ => map_cell.surface().terrain,
            };

            return terrain.group();
        }
    }
    0
}

impl ObstacleMapCell {
    pub fn new(position: Position<u8>, map_cell: Option<MapCell>) -> ObstacleMapCell {
        let terrain_group = calc_terrain_group(&map_cell);
        ObstacleMapCell {
            position,
            map_cell,
            template_index: None,
            terrain_group,
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

    pub fn template_index(&self) -> Option<usize> {
        self.template_index
    }

    pub fn terrain_group(&self) -> u16 {
        self.terrain_group
    }
}
