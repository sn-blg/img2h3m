use super::SurfaceInfo;
use crate::h3m::Surface;
use delta_e::DE2000;
use image::Rgb;

type RgbColor = [u8; 3];

fn surface_rgb_color(surface: Surface) -> RgbColor {
    match surface {
        Surface::Dirt => [0x52, 0x39, 0x08],
        Surface::Sand => [0xDE, 0xCE, 0x8C],
        Surface::Grass => [0x00, 0x42, 0x00],
        Surface::Snow => [0xB5, 0xC6, 0xC6],
        Surface::Swamp => [0x4A, 0x84, 0x6B],
        Surface::Rough => [0x84, 0x73, 0x31],
        Surface::Subterranean => [0x84, 0x31, 0x00],
        Surface::Lava => [0x4A, 0x4A, 0x4A],
        Surface::Highland => [0x29, 0x73, 0x18],
        Surface::Wasteland => [0xBD, 0x5A, 0x08],
        Surface::Water => [0x08, 0x52, 0x94],
        Surface::Rock => [0x00, 0x00, 0x00],
    }
}

fn surface_obstacle_rgb_color(surface: Surface) -> Option<RgbColor> {
    match surface {
        Surface::Dirt => Some([0x39, 0x29, 0x08]),
        Surface::Sand => Some([0xA5, 0x9C, 0x6B]),
        Surface::Grass => Some([0x00, 0x31, 0x00]),
        Surface::Snow => Some([0x8C, 0x9C, 0x9C]),
        Surface::Swamp => Some([0x21, 0x5A, 0x42]),
        Surface::Rough => Some([0x63, 0x52, 0x21]),
        Surface::Subterranean => Some([0x5A, 0x08, 0x00]),
        Surface::Lava => Some([0x29, 0x29, 0x29]),
        Surface::Highland => Some([0x21, 0x52, 0x10]),
        Surface::Wasteland => Some([0x9C, 0x42, 0x08]),
        Surface::Water => Some([0x00, 0x29, 0x6B]),
        Surface::Rock => None,
    }
}

struct Color {
    surface_info: SurfaceInfo,
    rgb_color: RgbColor,
}

impl Color {
    fn new(surface: Surface, obstacle: bool, rgb_color: RgbColor) -> Color {
        Color {
            surface_info: SurfaceInfo { surface, obstacle },
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

        let mut add_surface = |surface: Surface| {
            let color = surface_rgb_color(surface);

            palettes.all.push(Color::new(surface, false, color));

            if surface.is_ground() {
                palettes.ground.push(Color::new(surface, false, color));
            }

            let obstacle_color = if obstacles {
                surface_obstacle_rgb_color(surface)
            } else {
                None
            };

            if let Some(obstacle_color) = obstacle_color {
                palettes.all.push(Color::new(surface, true, obstacle_color));

                if surface.is_ground() {
                    palettes
                        .ground
                        .push(Color::new(surface, true, obstacle_color));
                }
            }
        };

        add_surface(Surface::Dirt);
        add_surface(Surface::Sand);
        add_surface(Surface::Grass);
        add_surface(Surface::Snow);
        add_surface(Surface::Swamp);
        add_surface(Surface::Rough);
        add_surface(Surface::Subterranean);
        add_surface(Surface::Lava);
        add_surface(Surface::Highland);
        add_surface(Surface::Wasteland);
        add_surface(Surface::Water);
        add_surface(Surface::Rock);

        palettes
    }

    pub fn nearest_surface(&self, pixel: &Rgb<u8>, ground_only: bool) -> SurfaceInfo {
        let input_color = &pixel.0;

        let palette = if ground_only { &self.ground } else { &self.all };

        *palette
            .iter()
            .map(|color| {
                (
                    &color.surface_info,
                    DE2000::from_rgb(&color.rgb_color, input_color),
                )
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}
