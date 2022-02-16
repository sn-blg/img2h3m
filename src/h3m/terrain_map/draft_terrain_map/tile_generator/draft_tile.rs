use super::common::NEIGHBORHOOD_SIZE;
use super::tiles_table::TilesGroupInfo;
use super::Neighborhood;
pub use crate::h3m::terrain_map::tile::TerrainVisibleType;
use crate::h3m::terrain_map::tile::Tile;

#[derive(Clone, Copy, PartialEq)]
pub enum TileComposition {
    Main,
    Fallback,
}

type NeighborhoodGroups = [Option<usize>; NEIGHBORHOOD_SIZE];

fn neighborhood_groups(neighborhood: &Neighborhood) -> NeighborhoodGroups {
    neighborhood.map(|cell| Some(cell?.tile?.group_number()))
}

#[derive(Clone, Copy, PartialEq)]
pub struct DraftTile {
    composition: TileComposition,
    name: &'static str,
    terrain_visible_type: TerrainVisibleType,
    code: u8,
    vertical_mirroring: bool,
    horizontal_mirroring: bool,
    group_number: usize,
    neighborhood_groups: NeighborhoodGroups,
}

impl DraftTile {
    pub fn new(
        tiles_group_info: &TilesGroupInfo,
        composition: TileComposition,
        code: u8,
        vertical_mirroring: bool,
        horizontal_mirroring: bool,
        neighborhood: &Neighborhood,
    ) -> DraftTile {
        DraftTile {
            composition,
            name: tiles_group_info.name(),
            terrain_visible_type: tiles_group_info.terrain_visible_type(),
            code,
            vertical_mirroring,
            horizontal_mirroring,
            group_number: tiles_group_info.group_number(),
            neighborhood_groups: neighborhood_groups(neighborhood),
        }
    }

    pub fn composition(&self) -> TileComposition {
        self.composition
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn terrain_visible_type(&self) -> TerrainVisibleType {
        self.terrain_visible_type
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn vertical_mirroring(&self) -> bool {
        self.vertical_mirroring
    }

    pub fn horizontal_mirroring(&self) -> bool {
        self.horizontal_mirroring
    }

    pub fn set_vertical_mirroring(&mut self, value: bool) {
        self.vertical_mirroring = value;
    }

    pub fn set_horizontal_mirroring(&mut self, value: bool) {
        self.horizontal_mirroring = value;
    }

    pub fn group_number(&self) -> usize {
        self.group_number
    }

    pub fn to_tile(self) -> Tile {
        Tile::new(
            self.terrain_visible_type,
            self.code,
            self.vertical_mirroring,
            self.horizontal_mirroring,
        )
    }

    pub fn is_neighborhood_changed(&self, new_neighborhood: &Neighborhood) -> bool {
        self.neighborhood_groups
            .iter()
            .zip(neighborhood_groups(new_neighborhood))
            .any(|(&old_group, new_group)| old_group != new_group)
    }
}

impl Default for DraftTile {
    fn default() -> Self {
        DraftTile {
            composition: TileComposition::Main,
            name: "",
            terrain_visible_type: TerrainVisibleType::None,
            code: 0,
            vertical_mirroring: false,
            horizontal_mirroring: false,
            group_number: 0,
            neighborhood_groups: [None; NEIGHBORHOOD_SIZE],
        }
    }
}
