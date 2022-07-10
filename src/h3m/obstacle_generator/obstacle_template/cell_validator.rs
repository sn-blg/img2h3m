use super::template_class::TemplateClass;
use super::tile_side::{CornerSide, Side};
use super::ObstacleTemplate;
use crate::h3m::obstacle_generator::obstacle_map::{NeighborhoodSameRelation, ObstacleMapCell};
use crate::h3m::terrain_map::{TerrainVisibleType, Tile, TileType};
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

fn same_left_right_bottom(nsr: &NeighborhoodSameRelation) -> bool {
    nsr[3] && nsr[4] && nsr[5] && nsr[6] && nsr[7]
}

impl ObstacleTemplate {
    pub fn is_valid_cell(&self, obstacle_map_cell: &ObstacleMapCell) -> bool {
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
            Terrain::Snow => self.is_valid_snow_mixed_tile(tile, nsr),
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
        false
        /*
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

            Terrain::Grass => match self.filename() {
                "grsmnt02.def" => return false,
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

        match tile.tile_type() {
            TileType::HalfDiff(Orientation::Horizontal, _) => !tile.vertical_mirroring(),
            TileType::HalfDiff2(Orientation::Horizontal, _, _) => tile.vertical_mirroring(),
            _ => false,
        }
        */
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
                "AVLr01u0.def" | "AVLr04u0.def" => return same_left_right_bottom(nsr),
                "AVLr03u0.def" => {
                    return if tile.mixed_only_with(Terrain::Sand) {
                        true
                    } else {
                        same_left_right_bottom(nsr)
                    }
                }
                "AVLr05u0.def" => {
                    return if tile.mixed_only_with(Terrain::Sand) {
                        same_bottom(nsr)
                    } else {
                        same_right_bottom(nsr)
                    }
                }
                "AVLr07u0.def" => {
                    return if tile.mixed_only_with(Terrain::Sand) {
                        same_right_bottom(nsr)
                    } else {
                        same_left_bottom(nsr)
                    }
                }
                "AVLr11u0.def" | "AVLstg40.def" | "AVLstg50.def" => {
                    return if tile.mixed_only_with(Terrain::Sand) {
                        true
                    } else {
                        same_right_bottom(nsr)
                    }
                }
                "AVLr12u0.def" => {
                    return if tile.mixed_only_with(Terrain::Dirt) {
                        same_right_bottom(nsr)
                    } else {
                        same_left_right_bottom(nsr)
                    }
                }
                "AVLr16u0.def" => {
                    return if tile.mixed_only_with(Terrain::Dirt) {
                        true
                    } else {
                        same_right_bottom(nsr)
                    }
                }
                "AVLstg60.def" => return tile.mixed_only_with(Terrain::Sand),
                _ => (),
            },

            Terrain::Highlands => match self.filename() {
                "AVlrhl03.def" => return false,
                "AVlrhl01.def" | "AVlrhl02.def" => return same_left_right_bottom(nsr),
                _ => (),
            },

            Terrain::Water => return self.may_located_on_mixed_tiles,

            _ => (),
        }

        false
    }

    fn is_valid_snow_mixed_tile(&self, tile: &Tile, nsr: &NeighborhoodSameRelation) -> bool {
        match self.template_class {
            TemplateClass::DeadVegetation => match self.filename() {
                "AVLd3sn0.def" => same_bottom(nsr),
                "AVLd7sn0.def" | "AVLd5sn0.def" | "AVLd9sn0.def" => !tile.is_scrap_on(Side::Bottom),
                "AVLddsn2.def" | "AVLddsn3.def" => !tile.is_scrap_on(Side::Bottom),
                "AVLd2sn0.def" => !tile.is_scrap_on_corner(CornerSide::BottomLeft),
                "AVLddsn4.def" => !tile.is_scrap_on_corner(CornerSide::BottomRight),
                _ => true,
            },

            TemplateClass::Stump => match self.filename() {
                "AVLp2sn0.def" => !tile.is_scrap() && same_bottom(nsr),
                _ => false,
            },

            TemplateClass::PineTrees => match self.filename() {
                "AVLSNTR8.def" => !tile.is_scrap_on(Side::Right) && !tile.is_scrap_on(Side::Bottom),
                "AVLSNTR1.def" => !tile.is_scrap() && same_bottom(nsr),
                "AVLSNTR0.def" | "AVLSNTR9.def" => {
                    !tile.is_scrap_on(Side::Right) && same_bottom(nsr)
                }
                "AVLSNTR2.def" | "AVLSNTR3.def" | "AVLSNTR5.def" | "AVLsntr6.def" => {
                    same_bottom(nsr)
                }
                "AVLsntr7.def" | "AVLSNTR4.def" => !tile.is_scrap_on_corner(CornerSide::BottomLeft),
                _ => false,
            },
            _ => self.may_located_on_mixed_tiles,
        }
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

        if tile.is_scrap() {
            match self.filename() {
                "AVLrk1w0.def" | "AVLref30.def" | "AVLrk4w0.def" | "AVLrk2w0.def"
                | "avlrfx06.def" | "ZReef1.def" => true,
                "avlrfx04.def" | "avlrfx01.def" => tile.is_scrap_on(Side::Right),
                "AVLrk3w0.def" => tile.is_scrap_on(Side::Top),
                "AVLref10.def" | "AVLref60.def" => tile.is_scrap_on(Side::Left),
                "avlrfx02.def" => tile.is_scrap_on_corner(CornerSide::TopLeft),
                "AVLref20.def" => {
                    tile.is_scrap_on_corner(CornerSide::BottomRight)
                        || tile.is_scrap_on_corner(CornerSide::TopLeft)
                }
                "AVLref50.def" | "ZReef2.def" | "AVLref40.def" => {
                    tile.is_scrap_on_corner(CornerSide::TopRight)
                }
                _ => false,
            }
        } else {
            self.may_located_on_mixed_tiles
        }
    }
}
