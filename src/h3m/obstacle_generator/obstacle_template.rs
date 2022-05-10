use super::template_class::TemplateClass;
use crate::common::position::DeltaPos;
use crate::h3m::parser::{H3mObjectTemplate, Mask};
use crate::h3m::result::H3mResult;
use crate::h3m::terrain_map::{TerrainVisibleType, Tile};
use crate::h3m::Terrain;

fn make_shape(mask: &Mask) -> Vec<DeltaPos> {
    let mut shape = Vec::new();
    for (row, byte) in mask.iter().rev().enumerate() {
        for column in 0..7usize {
            let bit_mask = (1 << (7 - column)) as u8;
            if byte & bit_mask == 0 {
                shape.push(DeltaPos::new(row, column));
            }
        }
    }
    shape
}

pub struct ObstacleTemplate {
    h3m_template: H3mObjectTemplate,
    shape: Vec<DeltaPos>,
    index: u32,
    terrain_group_mask: u16,
    frequency: usize,
    may_located_on_mixed_tiles: bool,
}

fn template_class(h3m_template: &H3mObjectTemplate) -> TemplateClass {
    TemplateClass::from_code(h3m_template.class, h3m_template.subclass).unwrap_or_else(|| {
        panic!(
            "Сouldn't define a class for the template '{:?}'",
            h3m_template
        )
    })
}

impl ObstacleTemplate {
    pub fn new(h3m_template: H3mObjectTemplate) -> ObstacleTemplate {
        let mask = h3m_template.shape_mask;
        let template_class = template_class(&h3m_template);
        let terrain_group_mask = calc_terrain_group_mask(template_class, &h3m_template);
        let may_located_on_mixed_tiles = may_located_on_mixed_tiles(template_class, &h3m_template);
        let shape = make_shape(&mask);
        let frequency = std::cmp::min(shape.len(), 10);
        ObstacleTemplate {
            h3m_template,
            shape,
            index: 0,
            terrain_group_mask,
            frequency,
            may_located_on_mixed_tiles,
        }
    }

    pub fn h3m_template(&self) -> &H3mObjectTemplate {
        &self.h3m_template
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn set_index_usize(&mut self, index: usize) -> H3mResult<()> {
        self.index = index.try_into()?;
        Ok(())
    }

    pub fn is_valid_terrain(&self, terrain_group: u16) -> bool {
        (terrain_group & self.terrain_group_mask) != 0
    }

    pub fn is_valid_tile(&self, tile: &Option<Tile>) -> bool {
        if let Some(tile) = tile {
            if matches!(
                tile.terrain_visible_type(),
                TerrainVisibleType::Mixed | TerrainVisibleType::DiffMixed(_)
            ) {
                return self.may_located_on_mixed_tiles;
            }
        }
        true
    }

    pub fn shape(&self) -> &[DeltaPos] {
        &self.shape
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }
}

fn calc_terrain_group_mask(template_class: TemplateClass, h3m_template: &H3mObjectTemplate) -> u16 {
    let mut terrain_group_mask = h3m_template.surface_editor_group_mask;

    if template_class == TemplateClass::Palms {
        terrain_group_mask &= !Terrain::Grass.group();
    }

    terrain_group_mask
}

fn may_located_on_mixed_tiles(
    template_class: TemplateClass,
    h3m_template: &H3mObjectTemplate,
) -> bool {
    let filename = &h3m_template.filename[..];

    match template_class {
        TemplateClass::OakTrees
        | TemplateClass::PineTrees
        | TemplateClass::Spruces
        | TemplateClass::Cactus
        | TemplateClass::Reef => true,

        TemplateClass::Stump => !matches!(filename, "AVLp1sn0.def"),
        TemplateClass::DeadVegetation => !matches!(
            filename,
            "AVLdt1s0.def" | "AVLdt2s0.def" | "AVLdt3s0.def" | "swddtree.def" | "AVLswp60.def"
        ),
        TemplateClass::Trees => {
            !matches!(filename, "AVLwlw20.def" | "AVLwlw10.def" | "AVLwlw30.def")
        }
        TemplateClass::Palms => !matches!(
            filename,
            "AVLplm20.def"
                | "AVLplm30.def"
                | "AVLplm50.def"
                | "AVLswmp0.def"
                | "AVLswmp1.def"
                | "AVLswmp2.def"
                | "AVLswmp3.def"
                | "AVLswmp4.def"
                | "AVLswmp5.def"
                | "AVLswmp6.def"
                | "AVLswmp7.def"
                | "avlswtr1.def"
                | "avlswtr2.def"
                | "avlswtr4.def"
                | "avlswn02.def"
                | "avlswtr3.def"
                | "avlswtr5.def"
                | "avlswtr8.def"
        ),

        TemplateClass::Rock => matches!(filename, "AVLrk5d0.def" | "AVLr03u0.def" | "AVLr16u0.def"), // ????
        TemplateClass::YuccaTrees => matches!(filename, "AVLyuc50.def" | "AVLyuc30.def"),

        _ => false,
    }
}
