use super::sparsity::Sparsity;
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
    h3m_template_index: u32,
    shape: Vec<DeltaPos>,
    terrain_group_mask: u16,
    frequency: usize,
    may_located_on_mixed_tiles: bool,
    sparsity: Sparsity, // limit: square of the distance to the same obstacle
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
        let frequency = frequency(template_class, shape.len(), &h3m_template);
        let sparsity = sparsity(template_class, shape.len(), &h3m_template);

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

    pub fn is_valid_tile(&self, tile: &Tile) -> bool {
        if matches!(
            tile.terrain_visible_type(),
            TerrainVisibleType::Mixed | TerrainVisibleType::DiffMixed(_)
        ) {
            self.may_located_on_mixed_tiles
        } else {
            true
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

fn calc_terrain_group_mask(template_class: TemplateClass, h3m_template: &H3mObjectTemplate) -> u16 {
    let mut terrain_group_mask = h3m_template.surface_editor_group_mask;

    if template_class == TemplateClass::Palms {
        terrain_group_mask &= !Terrain::Grass.group();
    }

    if template_class == TemplateClass::OakTrees {
        terrain_group_mask &= !Terrain::Dirt.group();
        terrain_group_mask &= !Terrain::Swamp.group();
    }

    let filename = &h3m_template.filename[..];

    if template_class == TemplateClass::DeadVegetation
        && matches!(
            filename,
            "AVLdead2.def"
                | "AVLdead3.def"
                | "AVLdead4.def"
                | "AVLdead5.def"
                | "AVLdead6.def"
                | "AVLdead7.def"
        )
    {
        terrain_group_mask &= !Terrain::Swamp.group();
    }

    if template_class == TemplateClass::Lake
        && matches!(filename, "AVLlk1g0.def" | "AVLlk2g0.def" | "AVLlk3g0.def")
    {
        terrain_group_mask |= Terrain::Swamp.group();
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
        | TemplateClass::Cactus => true,

        TemplateClass::Reef => !matches!(
            filename,
            "AVLref10.def"
                | "avlrfx02.def"
                | "AVLref60.def"
                | "avlrfx05.def"
                | "ZReef3.def"
                | "ZReef4.def"
                | "ZReef5.def"
                | "avlrfx04.def"
        ),
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

        TemplateClass::Rock => matches!(filename, "AVLrk5d0.def" | "AVLr16u0.def"),
        TemplateClass::YuccaTrees => matches!(filename, "AVLyuc50.def" | "AVLyuc30.def"),

        _ => false,
    }
}

fn sparsity(
    template_class: TemplateClass,
    surface_area: usize,
    h3m_template: &H3mObjectTemplate,
) -> Sparsity {
    let filename = &h3m_template.filename[..];

    let forest_sparsity = |surface_area| {
        if surface_area < 2 {
            2..=9
        } else {
            0..=0
        }
    };

    Sparsity::new(match template_class {
        TemplateClass::Lake
        | TemplateClass::LavaLake
        | TemplateClass::LimestoneLake
        | TemplateClass::TarPit
        | TemplateClass::FrozenLake => 100..=196,

        TemplateClass::IceBlock => 100..=196,

        TemplateClass::Volcano => match filename {
            "AVLvol20.def" => 225..=625,
            _ => 36..=64,
        },

        TemplateClass::Crater => 64..=100,

        TemplateClass::Rock => match filename {
            "AVLrws02.def" => 289..=625,
            _ => 36..=64,
        },

        TemplateClass::Mound | TemplateClass::SandDune => 36..=64,

        TemplateClass::SnowHills => 100..=144,

        TemplateClass::Stump => 64..=100,

        TemplateClass::Mountain => match filename {
            "AVLMTWL7.def" => 225..=625,
            _ => {
                if surface_area <= 4 {
                    25..=36
                } else if surface_area == 5 {
                    16..=25
                } else {
                    0..=0
                }
            }
        },

        TemplateClass::BarchanDunes => 25..=49,

        TemplateClass::Trees => match filename {
            "AVLwlw10.def" => 100..=196,
            "AVLwlw20.def" => 64..=100,
            "AVLwlw30.def" => 16..=36,
            _ => forest_sparsity(surface_area),
        },

        TemplateClass::Cactus | TemplateClass::YuccaTrees => 2..=9,

        TemplateClass::SandPit => 100..=144,

        TemplateClass::OakTrees | TemplateClass::PineTrees | TemplateClass::Spruces => {
            forest_sparsity(surface_area)
        }

        TemplateClass::Palms => match filename {
            "AVLswmp1.def" | "AVLplm20.def" | "AVLplm30.def" | "AVLswmp0.def" => 16..=36,
            _ => forest_sparsity(surface_area),
        },

        TemplateClass::DeadVegetation => match filename {
            "AVLdt3s0.def" => 100..=196,
            "AVLdt1s0.def" | "AVLdt2s0.def" | "swddtree.def" | "AVLswp60.def" | "AVLswp70.def" => {
                100..=144
            }
            _ => forest_sparsity(surface_area),
        },

        TemplateClass::Waterfalls => 144..=225,

        TemplateClass::Reef => 9..=16,

        TemplateClass::Mandrake => 2..=9,
    })
}

fn frequency(
    template_class: TemplateClass,
    surface_area: usize,
    h3m_template: &H3mObjectTemplate,
) -> usize {
    let filename = &h3m_template.filename[..];

    match template_class {
        TemplateClass::LavaLake => 1,

        TemplateClass::FrozenLake | TemplateClass::LimestoneLake | TemplateClass::TarPit => 2,

        TemplateClass::Lake => match filename {
            "AVLlk1r.def" => 1,
            _ => 3,
        },

        TemplateClass::IceBlock => 1,

        TemplateClass::Crater => 1,

        TemplateClass::Waterfalls => 2,

        TemplateClass::SnowHills => 1,

        TemplateClass::BarchanDunes => 4,

        TemplateClass::Palms => match filename {
            "avlspl09.def" | "avlspl10.def" | "avlspl11.def" | "avlspl12.def" | "avlspl13.def"
            | "avlspl14.def" => 1,

            "avlswtr7.def" | "avlswtr1.def" | "avlswtr2.def" | "avlswtr4.def" | "avlswn02.def"
            | "avlswtr5.def" | "avlswn03.def" | "avlswtr6.def" | "avlswn01.def"
            | "avlswtr8.def" | "avlswtr3.def" | "avlswtr9.def" | "avlswtr0.def"
            | "avlswt00.def" => surface_area + (surface_area / 3),

            _ => surface_area,
        },

        TemplateClass::Mountain => match filename {
            "AVLmtsn1.def" | "AVLmtsn2.def" | "AVLmtsn3.def" | "AVLmtsn4.def" | "AVLmtsn5.def"
            | "AVLmtsn6.def" => surface_area * 2,

            "AVLmtsw1.def" | "AVLmtsw2.def" | "AVLmtsw3.def" | "AVLmtsw4.def" | "AVLmtsw5.def"
            | "AVLmtsw6.def" | "mntswp01.def" | "mntswp02.def" | "mntswp03.def"
            | "mntswp04.def" | "mntswp05.def" | "mntswp06.def" => surface_area / 2,

            _ => surface_area,
        },

        TemplateClass::DeadVegetation => {
            if h3m_template.surface_editor_group_mask == Terrain::Snow.group() {
                surface_area
            } else {
                std::cmp::min(surface_area, 2)
            }
        }

        TemplateClass::Trees => match filename {
            "AVLtrRo8.def" | "AVLtrRo9.def" | "AVLtrRo5.def" | "AVLtrRo4.def" | "AVLtrRo1.def"
            | "AVLtrRo0.def" | "AVLtrRo3.def" | "AVLtrRo2.def" => std::cmp::min(surface_area, 3),

            _ => surface_area,
        },

        TemplateClass::Volcano => match filename {
            "AVLvol20.def" | "AVLvol40.def" => 1,
            _ => surface_area,
        },

        TemplateClass::Rock => match filename {
            "AVLrws02.def" => 0,
            _ => surface_area,
        },

        _ => surface_area,
    }
}
