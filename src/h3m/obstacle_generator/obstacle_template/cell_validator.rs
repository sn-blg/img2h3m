use super::template_class::TemplateClass;
use super::ObstacleTemplate;
use crate::common::position::DeltaPos;
use crate::h3m::obstacle_generator::obstacle_map::{NeighborhoodSameRelation, ObstacleMapCell};
use crate::h3m::terrain_map::{Orientation, TerrainVisibleType, Tile, TileType};
use crate::h3m::{MapCell, Terrain};

fn same_bottom(nsr: &NeighborhoodSameRelation) -> bool {
    nsr[6]
}

fn same_left(nsr: &NeighborhoodSameRelation) -> bool {
    nsr[3]
}

fn same_left_bottom(nsr: &NeighborhoodSameRelation) -> bool {
    nsr[3] && nsr[5] && nsr[6]
}

fn same_right_bottom(nsr: &NeighborhoodSameRelation) -> bool {
    nsr[4] && nsr[6] && nsr[7]
}

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

        let nsr = obstacle_map_cell.neighborhood_same_relation();

        if !match self.template_class {
            TemplateClass::Mountain | TemplateClass::Volcano | TemplateClass::Waterfalls => {
                self.is_valid_mountain_mixed_tile(map_cell, nsr)
            }
            TemplateClass::Rock => self.is_valid_rock_mixed_tile(map_cell, nsr),
            _ => true,
        } {
            return false;
        }

        if !match map_cell.surface().terrain {
            Terrain::Snow => self.may_located_on_mixed_tiles,
            Terrain::Water => self.is_valid_water_mixed_tile(tile, nsr),
            _ => true,
        } {
            return false;
        }

        true
    }

    fn is_valid_mountain_mixed_tile(
        &self,
        map_cell: &MapCell,
        nsr: &NeighborhoodSameRelation,
    ) -> bool {
        let tile = map_cell.tile();

        match map_cell.surface().terrain {
            Terrain::Dirt => match self.filename() {
                "avlmtdr7.def" => return true,
                "avlmtdr3.def" => {
                    return same_bottom(nsr);
                }
                "avlmtdr4.def" => {
                    return same_left_bottom(nsr);
                }
                "avlmtdr1.def" => {
                    return same_right_bottom(nsr);
                }
                _ => (),
            },

            Terrain::Snow => match self.filename() {
                "AVLmtsn2.def" => {
                    return same_right_bottom(nsr);
                }
                _ => (),
            },

            Terrain::Swamp => match self.filename() {
                "AVLmtsw3.def" => return false,
                _ => (),
            },

            Terrain::Rough => match self.filename() {
                "avlmtrf4.def" | "avlmtrf6.def" => {
                    return same_bottom(nsr);
                }
                "avlmtrf2.def" | "avlmtrf1.def" | "avlmtrf5.def" => {
                    return same_left_bottom(nsr);
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
                            return same_right_bottom(nsr);
                        }
                        "AVLmtsb2.def" => {
                            return same_left_bottom(nsr);
                        }
                        _ => (),
                    }
                }
            }

            Terrain::Lava => match self.filename() {
                "AVLmtvo5.def" | "AVLvol20.def" | "AVLvol10.def" | "AVLvol30.def" => return false,
                "AVLmtvo1.def" | "AVLmtvo2.def" => {
                    return same_right_bottom(nsr);
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

    fn is_valid_rock_mixed_tile(&self, map_cell: &MapCell, nsr: &NeighborhoodSameRelation) -> bool {
        let tile = map_cell.tile();

        match map_cell.surface().terrain {
            Terrain::Dirt => match self.filename() {
                "AVLrk5d0.def" => return true,
                "AvLRD01.def" => {
                    return same_right_bottom(nsr);
                }
                "AvLRD02.def" => {
                    return same_left(nsr);
                }
                _ => (),
            },

            Terrain::Subterranean => match self.filename() {
                "AVLr16u0.def" => {
                    return match tile.tile_type() {
                        TileType::HalfDiff(Orientation::Horizontal, _)
                        | TileType::HalfDiff2(Orientation::Horizontal, _, _) => true,
                        _ => false,
                    };
                }
                "AVLr03u0.def" => {
                    return match tile.tile_type() {
                        TileType::HalfDiff(_, Terrain::Sand) => true,
                        TileType::HalfDiff(Orientation::Horizontal, _)
                        | TileType::HalfDiff2(Orientation::Horizontal, _, _) => {
                            !tile.vertical_mirroring()
                        }
                        _ => false,
                    };
                }
                "AVLr04u0.def" | "AVLr01u0.def" => {
                    match tile.tile_type() {
                        TileType::HalfDiff(Orientation::Horizontal, _)
                        | TileType::HalfDiff2(Orientation::Horizontal, _, _) => {
                            !tile.vertical_mirroring()
                        }
                        _ => false,
                    };
                }
                "AVLr05u0.def" => return same_left_bottom(nsr),
                "AVLr07u0.def" => {
                    return match tile.tile_type() {
                        TileType::HalfDiff(_, Terrain::Sand) => same_right_bottom(nsr),
                        _ => same_left_bottom(nsr) && same_right_bottom(nsr),
                    };
                }
                "AVLr11u0.def" | "AVLstg40.def" => {
                    return match tile.tile_type() {
                        TileType::HalfDiff(_, Terrain::Sand) => true,
                        _ => same_right_bottom(nsr),
                    };
                }
                "AVLr12u0.def" => {
                    return match tile.tile_type() {
                        TileType::HalfDiff(_, Terrain::Dirt) => same_right_bottom(nsr),
                        _ => same_left_bottom(nsr) && same_right_bottom(nsr),
                    };
                }
                "AVLstg50.def" => {
                    return match tile.tile_type() {
                        TileType::HalfDiff(_, Terrain::Sand) => true,
                        _ => same_right_bottom(nsr),
                    };
                }
                "AVLstg60.def" => {
                    return match tile.tile_type() {
                        TileType::HalfDiff(_, Terrain::Sand) => true,
                        _ => false,
                    };
                }
                _ => (),
            },

            Terrain::Water => return self.may_located_on_mixed_tiles,
            _ => (),
        }

        false
    }

    fn is_valid_water_mixed_tile(&self, tile: &Tile, nsr: &NeighborhoodSameRelation) -> bool {
        if match self.filename() {
            "avlrfx04.def" | "ZReef2.def" => !nsr[3],
            "AVLref40.def" => !nsr[0],
            "ZReef3.def" => !nsr[1],
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
