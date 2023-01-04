use super::multi_sparsity::MultiSparsity;
use super::overlap_map::OverlapMap;
use super::sparsity::Sparsity;
use super::template_class::TemplateClass;
use super::ObstacleTemplate;
use crate::common::position::DeltaPos;
use crate::h3m::parser::{H3mObjectTemplate, Mask};
use crate::h3m::Terrain;
use std::ops::RangeInclusive;

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

        let multi_sparsity = multi_sparsity(create_params.filename);

        let may_be_overlapped = may_be_overlapped(template_class);

        let overlap_map = OverlapMap::new(create_params.filename);

        let overlap_obstacle_sparsity_penalty =
            overlap_obstacle_sparsity_penalty(template_class, create_params.filename);

        ObstacleTemplate {
            h3m_template: H3mObjectTemplate::from_create_params(create_params),
            filename: create_params.filename,
            template_class,
            h3m_template_index: 0,
            shape,
            terrain_group_mask,
            frequency,
            may_located_on_mixed_tiles,
            may_be_overlapped,
            sparsity,
            multi_sparsity,
            overlap_map,
            overlap_obstacle_sparsity_penalty,
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
        | TemplateClass::Cactus
        | TemplateClass::Rock => true,

        TemplateClass::Mountain | TemplateClass::Volcano | TemplateClass::Waterfalls => true,

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
    let forest_sparsity = |surface_area: usize| match surface_area {
        0..=1 => 2..=9,
        2 => 2..=6,
        3..=4 => 0..=4,
        _ => 0..=0,
    };

    Sparsity::new(match template_class {
        TemplateClass::Lake
        | TemplateClass::LimestoneLake
        | TemplateClass::TarPit
        | TemplateClass::FrozenLake => 100..=196, // todo: от размера?

        TemplateClass::LavaLake => 144..=324,

        TemplateClass::IceBlock => 100..=196,

        TemplateClass::Volcano => match filename {
            "AVLvol20.def" => 225..=625,
            _ => 36..=64,
        },

        TemplateClass::Crater => match filename {
            "AVLctrd0.def" | "AVLctds0.def" | "AVLctrg0.def" | "AVLctsn0.def" | "AVLctrs0.def"
            | "AVLctrr0.def" | "AVLctrl0.def" | "AVLcthl0.def" => 289..=625,
            _ => 100..=196,
        },

        TemplateClass::Rock => match filename {
            "AVLrws02.def" => 289..=625,
            "AvLRD02.def" => 81..=144,
            "AVLrk1w0.def" | "avlrfx06.def" | "AVLrk3w0.def" | "AVLrk2w0.def" | "AVLrk4w0.def" => {
                14..=25
            }
            _ => 25..=36,
        },

        TemplateClass::Mound | TemplateClass::SandDune => 36..=64,

        TemplateClass::SnowHills => 100..=144,

        TemplateClass::Stump => 64..=100,

        TemplateClass::Mountain => match filename {
            "AVLMHS00.def" | "AVLMHS01.def" | "AVLMHS02.def" | "AVLMHS03.def" | "AVLMHS04.def"
            | "AVLMHS05.def" => 64..=81,

            "AVLMTWL7.def" => 225..=625,
            "AVLmtsb0.def" => 36..=64,
            "AVLMTWL1.def" => 25..=36,
            "AVLmtsw1.def" | "AVLmtsw2.def" => 25..=36,
            "avlmtrf4.def" => 25..=36,
            "AVLmtds3.def" | "AVLmtds4.def" => 36..=49,
            _ => {
                if surface_area <= 4 {
                    25..=36
                } else if surface_area == 5 {
                    16..=25
                } else {
                    2..=9
                }
            }
        },

        TemplateClass::BarchanDunes => match filename {
            "AVLmtdn1.def" | "AVLmtdn2.def" => 144..=225,
            "AVLmtdn5.def" => 64..=100,
            "AVLmtdn4.def" => 36..=64,
            _ => 25..=49,
        },

        TemplateClass::Trees => match filename {
            "AVLwlw10.def" => 100..=196,
            "AVLwlw20.def" => 64..=100,
            "AVLwlw30.def" => 16..=36,
            _ => forest_sparsity(surface_area),
        },

        TemplateClass::Cactus => match filename {
            "AVLwct08.def" | "AVLca130.def" => 36..=100,
            _ => 2..=9,
        },

        TemplateClass::YuccaTrees => 2..=9,

        TemplateClass::SandPit => 100..=144,

        TemplateClass::OakTrees | TemplateClass::PineTrees | TemplateClass::Spruces => {
            forest_sparsity(surface_area)
        }

        TemplateClass::Palms => match filename {
            "AVLswmp1.def" | "AVLplm20.def" | "AVLplm30.def" | "AVLswmp0.def" | "avlswn03.def"
            | "avlswn02.def" => 16..=36,
            _ => forest_sparsity(surface_area),
        },

        TemplateClass::DeadVegetation => match filename {
            "AVLdt3s0.def" => 100..=196,
            "AVLdt1s0.def" | "AVLdt2s0.def" | "swddtree.def" | "AVLswp60.def" | "AVLswp70.def" => {
                100..=144
            }
            _ => forest_sparsity(surface_area),
        },

        TemplateClass::Waterfalls => 225..=625,

        TemplateClass::Reef => match filename {
            "ZReef5.def" => 16..=144,
            _ => 9..=16,
        },

        TemplateClass::Mandrake => 2..=9,
    })
}

fn update_multi_sparsity(
    filename: &'static str,
    multi_sparsity: &mut MultiSparsity,
    first_array: &[&'static str],
    second_array: &[&'static str],
    sparsity: RangeInclusive<usize>,
) {
    if first_array.contains(&filename) {
        for neighbor_name in second_array {
            multi_sparsity.add(neighbor_name, sparsity.clone());
        }
    } else if second_array.contains(&filename) {
        for neighbor_name in first_array {
            multi_sparsity.add(neighbor_name, sparsity.clone());
        }
    }
}

fn multi_sparsity(filename: &'static str) -> MultiSparsity {
    let mut multi_sparsity = MultiSparsity::new();

    let tar_pits = [
        "AVLwloi1.def",
        "AVLwloi2.def",
        "AVLwloi3.def",
        "AVLwloi4.def",
        "AVLwloi5.def",
    ];
    let limestone_lake = ["AVLwll00.def", "AVLwll01.def", "AVLwll02.def"];
    let wasteland_trees = [
        "AVLtRo00.def",
        "AVLtRo01.def",
        "AVLtRo02.def",
        "AVLtRo03.def",
        "AVLtRo04.def",
        "AVLtRo05.def",
        "AVLtRo06.def",
        "AVLtRo07.def",
        "AVLtRo08.def",
        "AVLtRo09.def",
        "AVLtRo10.def",
        "AVLtRo11.def",
        "AVLtRo12.def",
        "AVLtRo13.def",
        "AVLtrRo0.def",
        "AVLtrRo1.def",
        "AVLtrRo2.def",
        "AVLtrRo3.def",
        "AVLtrRo4.def",
        "AVLtrRo5.def",
        "AVLtrRo6.def",
        "AVLtrRo7.def",
        "AVLtrRo8.def",
        "AVLtrRo9.def",
    ];

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &tar_pits,
        &limestone_lake,
        2..=4,
    );

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &tar_pits,
        &wasteland_trees,
        8..=12,
    );

    let cactus = [
        "AVLca060.def",
        "AVLca090.def",
        "AVLca100.def",
        "AVLca130.def",
        "AVLwct01.def",
        "AVLwct02.def",
        "AVLwct03.def",
        "AVLwct04.def",
        "AVLwct05.def",
        "AVLwct06.def",
        "AVLwct07.def",
        "AVLwct08.def",
        "AVLwct09.def",
        "AVLwct10.def",
    ];

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLwloi5.def", "AVLwloi2.def"],
        &cactus,
        2..=3,
    );

    let big_barchan_dunes = ["AVLmtdn1.def", "AVLmtdn2.def"];
    let barchan_dunes = ["AVLmtdn3.def", "AVLmtdn4.def"];
    let little_barchan_dunes = ["AVLmtdn5.def", "AVLmtdn6.def"];

    let sand_mountain = [
        "AVLmtds1.def",
        "AVLmtds2.def",
        "AVLmtds3.def",
        "AVLmtds4.def",
        "AVLmtds5.def",
        "AVLmtds6.def",
    ];

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &big_barchan_dunes,
        &sand_mountain,
        2..=4,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &barchan_dunes,
        &sand_mountain,
        2..=2,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &little_barchan_dunes,
        &sand_mountain,
        2..=2,
    );

    let big_sand_palms = [
        "avlspl09.def",
        "avlspl10.def",
        "avlspl11.def",
        "avlspl12.def",
        "avlspl13.def",
        "avlspl14.def",
    ];

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &big_barchan_dunes,
        &big_sand_palms,
        2..=2,
    );

    let little_sand_palms = [
        "avlspl01.def",
        "avlspl02.def",
        "avlspl03.def",
        "avlspl04.def",
        "avlspl05.def",
        "avlspl06.def",
        "avlspl07.def",
        "avlspl08.def",
    ];

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &big_barchan_dunes,
        &little_sand_palms,
        2..=2,
    );

    let water_rock = ["AVLrfx07.def"];

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &big_barchan_dunes,
        &water_rock,
        2..=2,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &barchan_dunes,
        &water_rock,
        2..=2,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &little_barchan_dunes,
        &water_rock,
        2..=2,
    );

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLca120.def"],
        &["AVLca020.def"],
        9..=16,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLyuc50.def"],
        &["AVLyuc30.def"],
        9..=16,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLyuc40.def"],
        &["AVLyuc10.def"],
        9..=16,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLyuc20.def"],
        &["AVLyuc40.def"],
        9..=16,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLyuc20.def"],
        &["AVLyuc10.def"],
        9..=16,
    );

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLwtf00.def"],
        &["AVLwtf01.def"],
        9..=25,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLwtf00.def"],
        &["AVLwtf02.def"],
        9..=25,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLwtf01.def"],
        &["AVLwtf02.def"],
        9..=25,
    );

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLlk1s0.def"],
        &["AVLswp60.def"],
        100..=144,
    );

    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLMHS00.def"],
        &["avlhll03.def"],
        2..=2,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLMHS01.def"],
        &["avlhll03.def"],
        2..=2,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["avlhll01.def"],
        &["avlhll03.def"],
        2..=2,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLMHS03.def"],
        &["avlhll03.def"],
        2..=2,
    );
    update_multi_sparsity(
        filename,
        &mut multi_sparsity,
        &["AVLMHS05.def"],
        &["avlhll03.def"],
        2..=2,
    );

    multi_sparsity
}

fn frequency(
    template_class: TemplateClass,
    surface_area: usize,
    surface_editor_group_mask: u16,
    filename: &'static str,
) -> usize {
    let surface_area_coeff = surface_area * 10;

    match template_class {
        TemplateClass::LavaLake => match filename {
            "AVLllk10.def" | "AVLllk20.def" => 0,
            _ => 10,
        },

        TemplateClass::FrozenLake => 20,

        TemplateClass::LimestoneLake => 10,
        TemplateClass::TarPit => 15,

        TemplateClass::Lake => match filename {
            "AVLlk1r.def" => 10,
            "avlhll00.def" | "avlhll01.def" | "avlhll02.def" => 35,
            _ => 30,
        },

        TemplateClass::IceBlock => 10,

        TemplateClass::Crater => 10,

        TemplateClass::Waterfalls => 20,

        TemplateClass::SnowHills => match filename {
            "avlxsn02.def" => 0,
            _ => 10,
        },

        TemplateClass::BarchanDunes => match filename {
            "AVLmtdn1.def" | "AVLmtdn2.def" => 10,
            _ => 30,
        },

        TemplateClass::Palms => {
            if surface_editor_group_mask == Terrain::Sand.group() {
                3
            } else {
                match filename {
                    "AVLswmp6.def" | "AVLswmp7.def" => 40,
                    "AVLswmp2.def" | "AVLswmp3.def" | "AVLswmp4.def" | "AVLswmp5.def" => 30,

                    "avlswtr7.def" | "avlswtr1.def" | "avlswtr2.def" | "avlswtr4.def"
                    | "avlswn02.def" | "avlswtr5.def" | "avlswn03.def" | "avlswtr6.def"
                    | "avlswn01.def" | "avlswtr8.def" | "avlswtr3.def" | "avlswtr9.def"
                    | "avlswtr0.def" | "avlswt00.def" => {
                        surface_area_coeff + (surface_area_coeff / 3)
                    }

                    _ => surface_area_coeff,
                }
            }
        }

        TemplateClass::Mountain => match filename {
            "avlmtrf4.def" => 40,
            "AVLmtds1.def" => 60,

            "AVLmtsn1.def" | "AVLmtsn2.def" | "AVLmtsn3.def" | "AVLmtsn4.def" | "AVLmtsn5.def"
            | "AVLmtsn6.def" => surface_area_coeff * 2,

            "AVLMHS00.def" | "AVLMHS01.def" | "AVLMHS02.def" | "AVLMHS03.def" | "AVLMHS04.def"
            | "AVLMHS05.def" => std::cmp::min(surface_area_coeff, 37),

            "AVLmtsw1.def" | "AVLmtsw2.def" | "AVLmtsw3.def" | "AVLmtsw4.def" | "AVLmtsw5.def"
            | "AVLmtsw6.def" | "mntswp01.def" | "mntswp02.def" | "mntswp03.def"
            | "mntswp04.def" | "mntswp05.def" | "mntswp06.def" => surface_area_coeff / 2,

            _ => std::cmp::min(surface_area_coeff, 100),
        },

        TemplateClass::DeadVegetation => {
            if surface_editor_group_mask == Terrain::Snow.group() {
                surface_area_coeff
            } else {
                std::cmp::min(surface_area_coeff, 20)
            }
        }

        TemplateClass::Trees => match filename {
            "AVLtrRo4.def" => 20,
            "AVLtrRo8.def" | "AVLtrRo9.def" | "AVLtrRo5.def" | "AVLtrRo1.def" | "AVLtrRo0.def"
            | "AVLtrRo3.def" | "AVLtrRo2.def" => std::cmp::min(surface_area_coeff, 27),

            _ => surface_area_coeff,
        },

        TemplateClass::Volcano => match filename {
            "AVLvol20.def" | "AVLvol40.def" => 10,
            _ => surface_area_coeff,
        },

        TemplateClass::Rock => match filename {
            "AVLrws02.def" => 0,
            "AvLRD02.def" => 10,
            "AVLrk5d0.def" => 3,
            "AvLRD01.def" => 5,
            _ => surface_area_coeff,
        },

        TemplateClass::SandPit => 10,

        TemplateClass::YuccaTrees => 5,

        TemplateClass::SandDune => 30,

        TemplateClass::Cactus => match filename {
            "AVcact03.def" | "AVcact02.def" | "AVcact01.def" | "AVLca040.def" => 4,
            _ => 3,
        },

        _ => surface_area_coeff,
    }
}

fn may_be_overlapped(template_class: TemplateClass) -> bool {
    matches!(
        template_class,
        TemplateClass::Mountain
            | TemplateClass::PineTrees
            | TemplateClass::Palms
            | TemplateClass::Trees
            | TemplateClass::Waterfalls
    )
}

fn overlap_obstacle_sparsity_penalty(
    template_class: TemplateClass,
    filename: &'static str,
) -> usize {
    match template_class {
        TemplateClass::Mountain => match filename {
            "AVLMHS00.def" | "AVLMHS01.def" | "AVLMHS02.def" | "AVLMHS03.def" | "AVLMHS04.def"
            | "AVLMHS05.def" => 60,
            "AVLwtf00.def" | "AVLwtf01.def" | "AVLwtf02.def" => 81,

            "AVLmtds3.def" | "AVLmtds4.def" => 12,
            "AVLmtds6.def" => 6,
            "AVLmtsw1.def" | "AVLmtsw2.def" => 25,
            "avlmtrf4.def" => 12,
            _ => 0,
        },

        _ => 0,
    }
}
