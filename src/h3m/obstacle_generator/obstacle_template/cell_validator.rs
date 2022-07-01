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

        if self.may_located_on_mixed_tiles {
            return true;
        }

        let neighborhood_same_relation = obstacle_map_cell.neighborhood_same_relation();

        if match self.template_class {
            TemplateClass::Mountain | TemplateClass::Volcano | TemplateClass::Waterfalls => {
                self.is_valid_mountain_mixed_tile(map_cell, neighborhood_same_relation)
            }
            _ => false,
        } {
            return true;
        }

        if match map_cell.surface().terrain {
            Terrain::Snow => self.may_located_on_mixed_tiles,
            Terrain::Water => {
                self.is_valid_water_mixed_tile(tile, delta_pos, neighborhood_same_relation)
            }
            _ => false,
        } {
            return true;
        }

        false
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
            self.may_located_on_mixed_tiles
        }
    }

    fn is_valid_water_mixed_tile(
        &self,
        tile: &Tile,
        delta_pos: &DeltaPos,
        neighborhood_same_relation: &NeighborhoodSameRelation,
    ) -> bool {
        assert!(matches!(
            tile.terrain_visible_type(),
            TerrainVisibleType::Mixed | TerrainVisibleType::DiffMixed(_)
        ));

        if self.filename() == "avlrfx04.def" && !neighborhood_same_relation[3] {
            return false;
        }

        if let TileType::WideObliqueAngle(_) = tile.tile_type() {
            match self.filename() {
                "AVLrk3w0.def" => !tile.vertical_mirroring(),
                "AVLrk4w0.def" => !(tile.vertical_mirroring() && tile.horizontal_mirroring()),
                "avlrfx01.def" => tile.horizontal_mirroring(),
                _ => self.may_located_on_mixed_tiles,
            }
        } else {
            self.may_located_on_mixed_tiles
        }
    }
}
