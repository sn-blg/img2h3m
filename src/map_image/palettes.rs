use super::SurfaceInfo;
use crate::h3m::Surface;
use delta_e::DE2000;
use image::Rgb;

struct Color {
    surface_info: SurfaceInfo,
    rgb_color: [u8; 3],
}

impl Color {
    fn new(surface: Surface, obstacle: bool, rgb_color: [u8; 3]) -> Color {
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
    pub fn new(_obstacles: bool) -> Palettes {
        let mut palettes = Palettes {
            ground: Vec::new(),
            all: Vec::new(),
        };

        let mut add_surface = |surface: Surface| {
            palettes
                .all
                .push(Color::new(surface, false, surface.rgb_color()));

            if surface.is_ground() {
                palettes
                    .ground
                    .push(Color::new(surface, false, surface.rgb_color()));
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
