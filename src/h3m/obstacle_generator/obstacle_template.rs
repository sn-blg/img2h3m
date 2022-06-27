use super::obstacle_map::NeighborhoodSameRelation;
use super::sparsity::Sparsity;
use crate::common::position::DeltaPos;
use crate::h3m::parser::H3mObjectTemplate;
use crate::h3m::result::H3mResult;
use crate::h3m::terrain_map::{TerrainVisibleType, Tile, TileType};

pub struct ObstacleTemplate {
    h3m_template: H3mObjectTemplate,
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
        shape: Vec<DeltaPos>,
        terrain_group_mask: u16,
        frequency: usize,
        may_located_on_mixed_tiles: bool,
        sparsity: Sparsity,
    ) -> ObstacleTemplate {
        ObstacleTemplate {
            h3m_template,
            shape,
            h3m_template_index: 0,
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

    pub fn is_valid_terrain(&self, terrain_group: u16) -> bool {
        (terrain_group & self.terrain_group_mask) != 0
    }

    pub fn is_valid_tile(
        &self,
        tile: &Tile,
        delta_pos: &DeltaPos,
        neighborhood_same_relation: &NeighborhoodSameRelation,
    ) -> bool {
        if matches!(
            tile.terrain_visible_type(),
            TerrainVisibleType::Mixed | TerrainVisibleType::DiffMixed(_)
        ) {
            self.is_valid_mixed_tile(tile, delta_pos, neighborhood_same_relation)
        } else {
            true
        }
    }

    fn is_valid_mixed_tile(
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

    pub fn shape(&self) -> &[DeltaPos] {
        &self.shape
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn sparsity(&self) -> Sparsity {
        self.sparsity
    }
}
