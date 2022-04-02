use crate::common::RgbColor;
use crate::h3m::{Surface, Terrain};
use delta_e::DE2000;
use image::Rgb;
use strum::IntoEnumIterator;

fn terrain_rgb_color(terrain: Terrain) -> RgbColor {
    match terrain {
        Terrain::Dirt => [0x52, 0x39, 0x08],
        Terrain::Sand => [0xDE, 0xCE, 0x8C],
        Terrain::Grass => [0x00, 0x42, 0x00],
        Terrain::Snow => [0xB5, 0xC6, 0xC6],
        Terrain::Swamp => [0x4A, 0x84, 0x6B],
        Terrain::Rough => [0x84, 0x73, 0x31],
        Terrain::Subterranean => [0x84, 0x31, 0x00],
        Terrain::Lava => [0x4A, 0x4A, 0x4A],
        Terrain::Highland => [0x29, 0x73, 0x18],
        Terrain::Wasteland => [0xBD, 0x5A, 0x08],
        Terrain::Water => [0x08, 0x52, 0x94],
        Terrain::Rock => [0x00, 0x00, 0x00],
    }
}

fn obstacle_rgb_color(terrain: Terrain) -> Option<RgbColor> {
    match terrain {
        Terrain::Dirt => Some([0x39, 0x29, 0x08]),
        Terrain::Sand => Some([0xA5, 0x9C, 0x6B]),
        Terrain::Grass => Some([0x00, 0x31, 0x00]),
        Terrain::Snow => Some([0x8C, 0x9C, 0x9C]),
        Terrain::Swamp => Some([0x21, 0x5A, 0x42]),
        Terrain::Rough => Some([0x63, 0x52, 0x21]),
        Terrain::Subterranean => Some([0x5A, 0x08, 0x00]),
        Terrain::Lava => Some([0x29, 0x29, 0x29]),
        Terrain::Highland => Some([0x21, 0x52, 0x10]),
        Terrain::Wasteland => Some([0x9C, 0x42, 0x08]),
        Terrain::Water => Some([0x00, 0x29, 0x6B]),
        Terrain::Rock => None,
    }
}

struct Color {
    surface: Surface,
    rgb_color: RgbColor,
}

impl Color {
    fn new(terrain: Terrain, obstacle: bool, rgb_color: RgbColor) -> Color {
        Color {
            surface: Surface { terrain, obstacle },
            rgb_color,
        }
    }
}

type Palette = Vec<Color>;

pub struct Palettes {
    ground: Palette,
    all: Palette,
}

impl Palettes {
    pub fn new(obstacles: bool) -> Palettes {
        let mut palettes = Palettes {
            ground: Vec::new(),
            all: Vec::new(),
        };

        let mut add_terrain = |terrain: Terrain| {
            let color = terrain_rgb_color(terrain);

            palettes.all.push(Color::new(terrain, false, color));

            if terrain.is_ground() {
                palettes.ground.push(Color::new(terrain, false, color));
            }

            let obstacle_color = if obstacles {
                obstacle_rgb_color(terrain)
            } else {
                None
            };

            if let Some(obstacle_color) = obstacle_color {
                palettes.all.push(Color::new(terrain, true, obstacle_color));

                if terrain.is_ground() {
                    palettes
                        .ground
                        .push(Color::new(terrain, true, obstacle_color));
                }
            }
        };

        for terrain in Terrain::iter() {
            add_terrain(terrain);
        }

        palettes
    }

    pub fn nearest_surface(&self, pixel: &Rgb<u8>, ground_only: bool) -> Surface {
        let input_color = &pixel.0;

        let palette = if ground_only { &self.ground } else { &self.all };

        *palette
            .iter()
            .map(|color| {
                (
                    &color.surface,
                    DE2000::from_rgb(&color.rgb_color, input_color),
                )
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}
