use super::template_class::TemplateClass;
use super::ObstacleTemplate;
use crate::common::position::DeltaPos;
use crate::h3m::obstacle_generator::obstacle_map::{NeighborhoodSameRelation, ObstacleMapCell};
use crate::h3m::terrain_map::{Orientation, TerrainVisibleType, Tile, TileType};
use crate::h3m::{MapCell, Terrain};

impl ObstacleTemplate {
    pub fn is_valid_cell(&self, obstacle_map_cell: &ObstacleMapCell, delta_pos: &DeltaPos) -> bool {
        if !self.is_valid_terrain(obstacle_map_cell.terrain_group()) {
            return false;
        }

        let map_cell = match obstacle_map_cell.map_cell() {
            Some(map_cell) => map_cell,
            None => return false,
        };

        let tile = map_cell.tile();

        if matches!(
            tile.terrain_visible_type(),
            TerrainVisibleType::Same | TerrainVisibleType::Diff(_)
        ) {
            return true;
        }

        if !self.may_located_on_mixed_tiles {
            return false;
        }

        let neighborhood_same_relation = obstacle_map_cell.neighborhood_same_relation();

        if !match self.template_class {
            TemplateClass::Mountain | TemplateClass::Volcano | TemplateClass::Waterfalls => {
                self.is_valid_mountain_mixed_tile(map_cell, neighborhood_same_relation)
            }
            TemplateClass::Rock => {
                self.is_valid_rock_mixed_tile(map_cell, neighborhood_same_relation)
            }
            _ => true,
        } {
            return false;
        }

        if !match map_cell.surface().terrain {
            Terrain::Snow => self.may_located_on_mixed_tiles,
            Terrain::Water => self.is_valid_water_mixed_tile(tile, neighborhood_same_relation),
            _ => true,
        } {
            return false;
        }

        true
    }

    fn is_valid_mountain_mixed_tile(
        &self,
        map_cell: &MapCell,
        neighborhood_same_relation: &NeighborhoodSameRelation,
    ) -> bool {
        let same_bottom = || neighborhood_same_relation[6];
        let same_left_bottom = || {
            neighborhood_same_relation[3]
                && neighborhood_same_relation[5]
                && neighborhood_same_relation[6]
        };
        let same_right_bottom = || {
            neighborhood_same_relation[4]
                && neighborhood_same_relation[6]
                && neighborhood_same_relation[7]
        };

        let tile = map_cell.tile();

        match map_cell.surface().terrain {
            Terrain::Dirt => match self.filename() {
                "avlmtdr7.def" => return true,
                "avlmtdr3.def" => {
                    return same_bottom();
                }
                "avlmtdr4.def" => {
                    return same_left_bottom();
                }
                "avlmtdr1.def" => {
                    return same_right_bottom();
                }
                _ => (),
            },

            Terrain::Snow => match self.filename() {
                "AVLmtsn2.def" => {
                    return same_right_bottom();
                }
                _ => (),
            },

            Terrain::Swamp => match self.filename() {
                "AVLmtsw1.def" | "AVLmtsw4.def" => {
                    return same_right_bottom();
                }
                _ => (),
            },

            Terrain::Rough => match self.filename() {
                "avlmtrf4.def" | "avlmtrf6.def" => {
                    return same_bottom();
                }
                "avlmtrf2.def" | "avlmtrf1.def" | "avlmtrf5.def" => {
                    return same_left_bottom();
                }
                _ => (),
            },

            Terrain::Subterranean => {
                if matches!(tile.tile_type(), TileType::HalfDiff(_, Terrain::Sand)) {
                    return true;
                } else {
                    match self.filename() {
                        "AVLmtsb5.def" | "AVLmtsb4.def" => return true,
                        "AVLmtsb0.def" => {
                            return same_right_bottom();
                        }
                        "AVLmtsb2.def" => {
                            return same_left_bottom();
                        }
                        _ => (),
                    }
                }
            }

            Terrain::Lava => match self.filename() {
                "AVLmtvo5.def" | "AVLvol20.def" | "AVLvol10.def" | "AVLvol30.def" => return false,
                "AVLmtvo1.def" | "AVLmtvo2.def" => {
                    return same_right_bottom();
                }
                "AVLmtvo3.def" | "AVLvol60.def" => {
                    if matches!(
                        tile.tile_type(),
                        TileType::HalfDiff(Orientation::Vertical, _)
                    ) {
                        if tile.horizontal_mirroring() {
                            return true;
                        }
                    }
                }
                "AVLmtvo6.def" => {
                    if matches!(
                        tile.tile_type(),
                        TileType::HalfDiff(Orientation::Vertical, _)
                    ) {
                        if !tile.horizontal_mirroring() {
                            return true;
                        }
                    }
                }
                _ => (),
            },

            Terrain::Wasteland => match self.filename() {
                "AVLMTWL3.def" | "AVLMTWL7.def" => return false,
                "AVLMTWL5.def" | "AVLMTWL6.def" => {
                    if matches!(
                        tile.tile_type(),
                        TileType::HalfDiff(Orientation::Vertical, _)
                    ) {
                        return true;
                    }
                }
                _ => (),
            },

            _ => (),
        }

        if matches!(
            tile.tile_type(),
            TileType::HalfDiff(Orientation::Horizontal, _)
                | TileType::HalfDiff2(Orientation::Horizontal, _, _)
        ) {
            !tile.vertical_mirroring()
        } else {
            false
        }
    }

    fn is_valid_rock_mixed_tile(
        &self,
        map_cell: &MapCell,
        neighborhood_same_relation: &NeighborhoodSameRelation,
    ) -> bool {
        match map_cell.surface().terrain {
            Terrain::Water => self.may_located_on_mixed_tiles,
            _ => matches!(self.filename(), "AVLrk5d0.def" | "AVLr16u0.def"),
        }
    }

    fn is_valid_water_mixed_tile(
        &self,
        tile: &Tile,
        neighborhood_same_relation: &NeighborhoodSameRelation,
    ) -> bool {
        if match self.filename() {
            "avlrfx04.def" | "ZReef2.def" => !neighborhood_same_relation[3],
            "AVLref40.def" => !neighborhood_same_relation[0],
            "ZReef3.def" => !neighborhood_same_relation[1],
            _ => false,
        } {
            return false;
        }

        if let TileType::WideObliqueAngle(_) = tile.tile_type() {
            match self.filename() {
                "AVLref30.def" | "AVLrk4w0.def" | "AVLrk2w0.def" | "avlrfx06.def"
                | "ZReef1.def" => true,
                "AVLrk1w0.def" => tile.vertical_mirroring(),
                "avlrfx04.def" | "avlrfx01.def" => tile.horizontal_mirroring(),
                "AVLrk3w0.def" => !tile.vertical_mirroring(),
                "AVLref10.def" | "AVLref60.def" => !tile.horizontal_mirroring(),
                "AVLref20.def" => tile.vertical_mirroring() && tile.horizontal_mirroring(),
                "AVLref50.def" | "ZReef2.def" | "AVLref40.def" => {
                    tile.horizontal_mirroring() && !tile.vertical_mirroring()
                }
                _ => false,
            }
        } else {
            self.may_located_on_mixed_tiles
        }
    }
}
