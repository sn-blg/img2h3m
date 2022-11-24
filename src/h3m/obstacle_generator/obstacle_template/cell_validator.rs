use super::template_class::TemplateClass;
use super::tile_side::{CornerSide, Side};
use super::ObstacleTemplate;
use crate::common::position::Position;
use crate::h3m::obstacle_generator::obstacle_map::LocatedObstacle;
use crate::h3m::obstacle_generator::obstacle_map::{NeighborhoodSameRelation, ObstacleMapCell};
use crate::h3m::terrain_map::{TerrainVisibleType, Tile};
use crate::h3m::{MapCell, Terrain};

fn same_side(nsr: &NeighborhoodSameRelation, side_list: &[Side]) -> bool {
    let mut top = false;
    let mut bottom = false;
    let mut left = false;
    let mut right = false;

    for side in side_list {
        match side {
            Side::Top => top = true,
            Side::Bottom => bottom = true,
            Side::Left => left = true,
            Side::Right => right = true,
        }
    }

    if (top && !nsr[1])
        || (left && !nsr[3])
        || (right && !nsr[4])
        || (bottom && !nsr[6])
        || (top && left && !nsr[0])
        || (top && right && !nsr[2])
        || (bottom && left && !nsr[5])
        || (bottom && right && !nsr[7])
    {
        return false;
    }

    true
}

#[derive(Clone, Copy)]
pub enum CellValidationResult {
    Valid,
    ValidWithOverlapping,
    Invalid,
}

impl ObstacleTemplate {
    pub fn validate_cell(
        &self,
        obstacle_map_cell: &ObstacleMapCell,
        obstacle_base_position: &Position,
    ) -> CellValidationResult {
        let mut is_overlapping = false;
        let is_valid = self.is_valid_cell(
            obstacle_map_cell,
            obstacle_base_position,
            &mut is_overlapping,
        );

        match (is_valid, is_overlapping) {
            (true, true) => CellValidationResult::ValidWithOverlapping,
            (true, false) => CellValidationResult::Valid,
            (false, _) => CellValidationResult::Invalid,
        }
    }

    fn is_valid_cell(
        &self,
        obstacle_map_cell: &ObstacleMapCell,
        obstacle_base_position: &Position,
        if_overlapping: &mut bool,
    ) -> bool {
        *if_overlapping = false;

        if !self.is_valid_terrain(obstacle_map_cell.terrain_group()) {
            return false;
        }

        match obstacle_map_cell.located_obstacle() {
            Some(LocatedObstacle::Common) => return false,
            Some(LocatedObstacle::Overlapping(ref vec)) => {
                assert!(!vec.is_empty());
                for overlapping_obstacle in vec {
                    if !self.overlap_map.may_overlap(
                        overlapping_obstacle.filename(),
                        obstacle_base_position.sub_position(overlapping_obstacle.base_position()),
                    ) {
                        return false;
                    }
                }
                *if_overlapping = true;
            }
            None => (),
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
            TemplateClass::Cactus => {
                if self.filename() == "AVLwct08.def" {
                    tile.mixed_only_with(Terrain::Sand)
                        || same_side(nsr, &[Side::Right, Side::Left])
                } else {
                    true
                }
            }
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
        let tile = map_cell.tile();

        match map_cell.surface().terrain {
            Terrain::Dirt => match self.filename() {
                "avlmtdr7.def" => return true,
                "avlmtdr3.def" => {
                    return same_side(nsr, &[Side::Bottom]);
                }
                "avlmtdr4.def" => {
                    return same_side(nsr, &[Side::Left, Side::Bottom]);
                }
                "avlmtdr1.def" => {
                    return same_side(nsr, &[Side::Right, Side::Bottom]);
                }
                _ => (),
            },

            Terrain::Grass => {
                if self.filename() == "grsmnt02.def" {
                    return false;
                }
            }

            Terrain::Snow => {
                if self.filename() == "AVLmtsn2.def" {
                    return same_side(nsr, &[Side::Right, Side::Bottom]);
                }
            }

            Terrain::Swamp => {
                if self.filename() == "AVLmtsw3.def" {
                    return false;
                }
            }

            Terrain::Rough => match self.filename() {
                "avlmtrf4.def" | "avlmtrf6.def" => {
                    return same_side(nsr, &[Side::Right, Side::Bottom]);
                }
                "avlmtrf2.def" | "avlmtrf1.def" | "avlmtrf5.def" => {
                    return same_side(nsr, &[Side::Left, Side::Bottom]);
                }
                _ => (),
            },

            Terrain::Subterranean => {
                if tile.mixed_only_with(Terrain::Sand) {
                    return true;
                } else {
                    match self.filename() {
                        "AVLmtsb5.def" | "AVLmtsb4.def" => return true,
                        "AVLmtsb0.def" => {
                            return same_side(nsr, &[Side::Right, Side::Bottom]);
                        }
                        "AVLmtsb2.def" => {
                            return same_side(nsr, &[Side::Left, Side::Bottom]);
                        }
                        _ => (),
                    }
                }
            }

            Terrain::Lava => match self.filename() {
                "AVLmtvo5.def" | "AVLvol20.def" | "AVLvol10.def" | "AVLvol30.def"
                | "AVLmtvo4.def" => return false,
                "AVLmtvo3.def" | "AVLvol60.def" => {
                    return same_side(nsr, &[Side::Top, Side::Left, Side::Bottom])
                }
                "AVLmtvo6.def" => {
                    return tile.mixed_only_with(Terrain::Dirt)
                        && same_side(nsr, &[Side::Top, Side::Right, Side::Bottom])
                }
                "AVLvol40.def" => {
                    return tile.mixed_only_with(Terrain::Dirt)
                        && same_side(nsr, &[Side::Left, Side::Right, Side::Bottom])
                }
                _ => (),
            },

            Terrain::Wasteland => match self.filename() {
                "AVLMTWL3.def" | "AVLMTWL7.def" => return false,
                "AVLMTWL5.def" | "AVLMTWL6.def" => {
                    return same_side(nsr, &[Side::Top, Side::Left, Side::Bottom])
                        || same_side(nsr, &[Side::Top, Side::Right, Side::Bottom])
                }
                _ => (),
            },

            _ => (),
        }

        same_side(nsr, &[Side::Left, Side::Right, Side::Bottom])
    }

    fn is_valid_rock_mixed_tile(&self, map_cell: &MapCell, nsr: &NeighborhoodSameRelation) -> bool {
        let tile = map_cell.tile();

        match map_cell.surface().terrain {
            Terrain::Dirt => match self.filename() {
                "AVLrk5d0.def" => return true,
                "AvLRD01.def" => {
                    return same_side(nsr, &[Side::Right, Side::Bottom]);
                }
                "AvLRD02.def" => {
                    return same_side(nsr, &[Side::Left]);
                }
                _ => (),
            },

            Terrain::Subterranean => match self.filename() {
                "AVLr01u0.def" | "AVLr04u0.def" | "AVLr05u0.def" => {
                    return same_side(nsr, &[Side::Left, Side::Right, Side::Bottom])
                }
                "AVLr03u0.def" => {
                    return if tile.mixed_only_with(Terrain::Sand) {
                        same_side(nsr, &[Side::Left, Side::Right])
                    } else {
                        same_side(nsr, &[Side::Left, Side::Right, Side::Bottom])
                    }
                }
                "AVLr07u0.def" => {
                    return if tile.mixed_only_with(Terrain::Sand) {
                        same_side(nsr, &[Side::Right, Side::Bottom])
                    } else {
                        same_side(nsr, &[Side::Left, Side::Bottom])
                    }
                }
                "AVLr11u0.def" | "AVLstg40.def" | "AVLstg50.def" => {
                    return tile.mixed_only_with(Terrain::Sand)
                        || same_side(nsr, &[Side::Right, Side::Bottom]);
                }
                "AVLr12u0.def" => {
                    return if tile.mixed_only_with(Terrain::Dirt) {
                        same_side(nsr, &[Side::Right, Side::Bottom])
                    } else {
                        same_side(nsr, &[Side::Left, Side::Right, Side::Bottom])
                    }
                }
                "AVLr16u0.def" => {
                    return tile.mixed_only_with(Terrain::Dirt)
                        || same_side(nsr, &[Side::Right, Side::Bottom]);
                }
                "AVLstg60.def" => return tile.mixed_only_with(Terrain::Sand),
                _ => (),
            },

            Terrain::Highlands => match self.filename() {
                "AVlrhl03.def" => return false,
                "AVlrhl01.def" | "AVlrhl02.def" => {
                    return same_side(nsr, &[Side::Left, Side::Right, Side::Bottom])
                }
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
                "AVLd3sn0.def" => same_side(nsr, &[Side::Bottom]),
                "AVLd7sn0.def" | "AVLd5sn0.def" | "AVLd9sn0.def" => !tile.is_scrap_on(Side::Bottom),
                "AVLddsn2.def" | "AVLddsn3.def" => !tile.is_scrap_on(Side::Bottom),
                "AVLd2sn0.def" => !tile.is_scrap_on_corner(CornerSide::BottomLeft),
                "AVLddsn4.def" => !tile.is_scrap_on_corner(CornerSide::BottomRight),
                _ => true,
            },

            TemplateClass::Stump => match self.filename() {
                "AVLp2sn0.def" => !tile.is_scrap() && same_side(nsr, &[Side::Bottom]),
                _ => false,
            },

            TemplateClass::PineTrees => match self.filename() {
                "AVLSNTR8.def" => !tile.is_scrap_on(Side::Right) && !tile.is_scrap_on(Side::Bottom),
                "AVLSNTR1.def" => !tile.is_scrap() && same_side(nsr, &[Side::Bottom]),
                "AVLSNTR0.def" | "AVLSNTR9.def" => {
                    !tile.is_scrap_on(Side::Right) && same_side(nsr, &[Side::Bottom])
                }
                "AVLSNTR2.def" | "AVLSNTR3.def" | "AVLSNTR5.def" | "AVLsntr6.def" => {
                    same_side(nsr, &[Side::Bottom])
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
            "AVLrfx07.def" => !same_side(nsr, &[Side::Top, Side::Right]),
            "AVLrfx08.def" => !same_side(nsr, &[Side::Top, Side::Left]),
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
                "AVLref20.def" | "avlrfx02.def" => {
                    tile.is_scrap_on_corner(CornerSide::BottomRight)
                        || tile.is_scrap_on_corner(CornerSide::TopLeft)
                }
                "AVLref50.def" | "ZReef2.def" => tile.is_scrap_on_corner(CornerSide::TopRight),
                "avlrfx03.def" | "AVLref40.def" => {
                    tile.is_scrap_on_corner(CornerSide::BottomLeft)
                        || tile.is_scrap_on_corner(CornerSide::TopRight)
                }
                "ZReef3.def" | "AVLrfx07.def" => tile.is_scrap_on_corner(CornerSide::BottomLeft),
                "AVLrfx08.def" => tile.is_scrap_on_corner(CornerSide::BottomRight),
                _ => false,
            }
        } else {
            self.may_located_on_mixed_tiles
        }
    }
}
