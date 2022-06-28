use super::obstacle_map::NeighborhoodSameRelation;
use super::obstacle_map::ObstacleMapCell;
use super::sparsity::Sparsity;
use super::template_class::TemplateClass;
use crate::common::position::DeltaPos;
use crate::h3m::parser::H3mObjectTemplate;
use crate::h3m::result::H3mResult;
use crate::h3m::terrain_map::{TerrainVisibleType, Tile, TileType};
use crate::h3m::Terrain;

pub struct ObstacleTemplate {
    h3m_template: H3mObjectTemplate,
    template_class: TemplateClass,
    h3m_template_index: u32,
    shape: Vec<DeltaPos>,
    terrain_group_mask: u16,
    frequency: usize,
    may_located_on_mixed_tiles: bool,
    sparsity: Sparsity, // limit: square of the distance to the same obstacle
}

impl ObstacleTemplate {
    pub fn new(
        h3m_template: H3mObjectTemplate,
        template_class: TemplateClass,
        shape: Vec<DeltaPos>,
        terrain_group_mask: u16,
        frequency: usize,
        may_located_on_mixed_tiles: bool,
        sparsity: Sparsity,
    ) -> ObstacleTemplate {
        ObstacleTemplate {
            h3m_template,
            template_class,
            h3m_template_index: 0,
            shape,
            terrain_group_mask,
            frequency,
            may_located_on_mixed_tiles,
            sparsity,
        }
    }

    pub fn h3m_template(&self) -> &H3mObjectTemplate {
        &self.h3m_template
    }

    pub fn h3m_template_index(&self) -> u32 {
        self.h3m_template_index
    }

    pub fn set_h3m_template_index(&mut self, index: usize) -> H3mResult<()> {
        self.h3m_template_index = index.try_into()?;
        Ok(())
    }

    pub fn shape(&self) -> &[DeltaPos] {
        &self.shape
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn sparsity(&self) -> Sparsity {
        self.sparsity
    }

    pub fn is_valid_terrain(&self, terrain_group: u16) -> bool {
        (terrain_group & self.terrain_group_mask) != 0
    }

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

        //if self.template_class == TemplateClass::Mountain {
        //    return neighborhood_same_relation[3] && neighborhood_same_relation[6];
        //}

        match map_cell.surface().terrain {
            Terrain::Snow => self.may_located_on_mixed_tiles,
            Terrain::Water => {
                self.is_valid_water_mixed_tile(tile, delta_pos, neighborhood_same_relation)
            }
            _ => self.may_located_on_mixed_tiles,
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

        let filename = &self.h3m_template.filename[..];

        if filename == "avlrfx04.def" && !neighborhood_same_relation[3] {
            return false;
        }

        if let TileType::WideObliqueAngle(_) = tile.tile_type() {
            match filename {
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
