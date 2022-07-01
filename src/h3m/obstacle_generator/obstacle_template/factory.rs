use super::sparsity::Sparsity;
use super::template_class::TemplateClass;
use super::ObstacleTemplate;
use crate::common::position::DeltaPos;
use crate::h3m::parser::{H3mObjectTemplate, Mask};
use crate::h3m::Terrain;
use std::cmp::Ordering;

pub struct ObstacleTemplateCreateParams {
    pub filename: &'static str,
    pub shape_mask: [u8; 6],
    pub visit_mask: [u8; 6],
    pub surface_type_mask: u16,
    pub surface_editor_group_mask: u16,
    pub class: u32,
    pub subclass: u32,
    pub group: u8,
    pub is_overlay: bool,
}

impl H3mObjectTemplate {
    fn from_create_params(create_params: &ObstacleTemplateCreateParams) -> H3mObjectTemplate {
        H3mObjectTemplate {
            filename: String::from(create_params.filename),
            shape_mask: create_params.shape_mask,
            visit_mask: create_params.visit_mask,
            surface_type_mask: create_params.surface_type_mask,
            surface_editor_group_mask: create_params.surface_editor_group_mask,
            class: create_params.class,
            subclass: create_params.subclass,
            group: create_params.group,
            is_overlay: create_params.is_overlay,
        }
    }
}

impl ObstacleTemplate {
    pub fn new(create_params: &ObstacleTemplateCreateParams) -> ObstacleTemplate {
        let template_class = template_class(
            create_params.class,
            create_params.subclass,
            create_params.filename,
        );

        let terrain_group_mask = calc_terrain_group_mask(
            template_class,
            create_params.surface_editor_group_mask,
            create_params.filename,
        );

        let may_located_on_mixed_tiles =
            may_located_on_mixed_tiles(template_class, create_params.filename);

        let shape = make_shape(&create_params.shape_mask);

        let frequency = frequency(
            template_class,
            shape.len(),
            create_params.surface_editor_group_mask,
            create_params.filename,
        );

        let sparsity = sparsity(template_class, shape.len(), create_params.filename);

        ObstacleTemplate {
            h3m_template: H3mObjectTemplate::from_create_params(create_params),
            filename: create_params.filename,
            template_class,
            h3m_template_index: 0,
            shape,
            terrain_group_mask,
            frequency,
            may_located_on_mixed_tiles,
            sparsity,
        }
    }
}

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

fn template_class(class: u32, subclass: u32, filename: &'static str) -> TemplateClass {
    TemplateClass::from_code(class, subclass)
        .unwrap_or_else(|| panic!("Сouldn't define a class for the template '{:?}'", filename))
}

fn calc_terrain_group_mask(
    template_class: TemplateClass,
    mut surface_editor_group_mask: u16,
    filename: &'static str,
) -> u16 {
    if template_class == TemplateClass::Palms {
        surface_editor_group_mask &= !Terrain::Grass.group();
    }

    if template_class == TemplateClass::OakTrees {
        surface_editor_group_mask &= !Terrain::Dirt.group();
        surface_editor_group_mask &= !Terrain::Swamp.group();
    }

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
        surface_editor_group_mask &= !Terrain::Swamp.group();
    }

    if template_class == TemplateClass::Lake
        && matches!(filename, "AVLlk1g0.def" | "AVLlk2g0.def" | "AVLlk3g0.def")
    {
        surface_editor_group_mask |= Terrain::Swamp.group();
    }

    surface_editor_group_mask
}

fn may_located_on_mixed_tiles(template_class: TemplateClass, filename: &'static str) -> bool {
    match template_class {
        TemplateClass::OakTrees
        | TemplateClass::PineTrees
        | TemplateClass::Spruces
        | TemplateClass::Cactus => true,

        TemplateClass::Mountain | TemplateClass::Volcano | TemplateClass::Waterfalls => true,

        TemplateClass::Rock => !matches!(filename, "AVLrfx08.def" | "AVLrfx07.def"),
        TemplateClass::Reef => !matches!(filename, "avlrfx05.def" | "ZReef4.def" | "ZReef5.def"),

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

        TemplateClass::YuccaTrees => matches!(filename, "AVLyuc50.def" | "AVLyuc30.def"),
        _ => false,
    }
}

fn sparsity(
    template_class: TemplateClass,
    surface_area: usize,
    filename: &'static str,
) -> Sparsity {
    let forest_sparsity = |surface_area: usize| match surface_area.cmp(&2) {
        Ordering::Less => 2..=9,
        Ordering::Greater => 0..=0,
        Ordering::Equal => 2..=6,
    };

    Sparsity::new(match template_class {
        TemplateClass::Lake
        | TemplateClass::LimestoneLake
        | TemplateClass::TarPit
        | TemplateClass::FrozenLake => 100..=196,

        TemplateClass::LavaLake => 144..=324,

        TemplateClass::IceBlock => 100..=196,

        TemplateClass::Volcano => match filename {
            "AVLvol20.def" => 225..=625,
            _ => 36..=64,
        },

        TemplateClass::Crater => match filename {
            "AVLctrd0.def" | "AVLctds0.def" | "AVLctrg0.def" | "AVLctsn0.def" | "AVLctrs0.def"
            | "AVLctrr0.def" | "AVLctrl0.def" | "AVLcthl0.def" => 289..=625,
            _ => 64..=100,
        },

        TemplateClass::Rock => match filename {
            "AVLrws02.def" => 289..=625,
            "AVLrk3w0.def" | "AVLrk4w0.def" => 14..=16,
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

        TemplateClass::BarchanDunes => match filename {
            "AVLmtdn1.def" | "AVLmtdn2.def" => 100..=196,
            _ => 25..=49,
        },

        TemplateClass::Trees => match filename {
            "AVLwlw10.def" => 100..=196,
            "AVLwlw20.def" => 64..=100,
            "AVLwlw30.def" => 16..=36,
            _ => forest_sparsity(surface_area),
        },

        TemplateClass::Cactus => match filename {
            "AVLwct08.def" => 4..=64,
            _ => 2..=9,
        },

        TemplateClass::YuccaTrees => 2..=9,

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
    surface_editor_group_mask: u16,
    filename: &'static str,
) -> usize {
    match template_class {
        TemplateClass::LavaLake => match filename {
            "AVLllk10.def" | "AVLllk20.def" => 0,
            _ => 1,
        },

        TemplateClass::FrozenLake => 2,

        TemplateClass::LimestoneLake | TemplateClass::TarPit => 1,

        TemplateClass::Lake => match filename {
            "AVLlk1r.def" => 1,
            _ => 3,
        },

        TemplateClass::IceBlock => 1,

        TemplateClass::Crater => 1,

        TemplateClass::Waterfalls => 2,

        TemplateClass::SnowHills => 1,

        TemplateClass::BarchanDunes => match filename {
            "AVLmtdn1.def" | "AVLmtdn2.def" => 3,
            _ => 4,
        },

        TemplateClass::Palms => match filename {
            "avlspl09.def" | "avlspl10.def" | "avlspl11.def" | "avlspl12.def" | "avlspl13.def"
            | "avlspl14.def" => 2,

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
            if surface_editor_group_mask == Terrain::Snow.group() {
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

        TemplateClass::SandPit => 1,

        _ => surface_area,
    }
}
