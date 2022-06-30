use super::template_class::TemplateClass;
use super::ObstacleTemplate;
use crate::common::position::DeltaPos;
use crate::h3m::obstacle_generator::obstacle_map::{NeighborhoodSameRelation, ObstacleMapCell};
use crate::h3m::terrain_map::{Orientation, TerrainVisibleType, Tile, TileType};
use crate::h3m::Terrain;

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

        let neighborhood_same_relation = obstacle_map_cell.neighborhood_same_relation();

        if self.template_class == TemplateClass::Mountain {
            return self.is_valid_mountain_mixed_tile(tile, delta_pos, neighborhood_same_relation);
        }

        match map_cell.surface().terrain {
            Terrain::Snow => self.may_located_on_mixed_tiles,
            Terrain::Water => {
                self.is_valid_water_mixed_tile(tile, delta_pos, neighborhood_same_relation)
            }
            _ => self.may_located_on_mixed_tiles,
        }
    }

    fn is_valid_mountain_mixed_tile(
        &self,
        tile: &Tile,
        delta_pos: &DeltaPos,
        neighborhood_same_relation: &NeighborhoodSameRelation,
    ) -> bool {
        match self.filename() {
            "avlmtdr7.def" => true,
            "avlmtdr3.def" => neighborhood_same_relation[6],
            "avlmtdr4.def" => {
                neighborhood_same_relation[3]
                    && neighborhood_same_relation[5]
                    && neighborhood_same_relation[6]
            }
            "avlmtdr1.def" | "AVLmtsn2.def" | "AVLmtsw1.def" | "AVLmtsw4.def" => {
                neighborhood_same_relation[4]
                    && neighborhood_same_relation[6]
                    && neighborhood_same_relation[7]
            }
            _ => {
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
