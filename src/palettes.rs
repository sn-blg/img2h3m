use crate::h3m::Surface;
use delta_e::DE2000;
use image::Rgb;

type Palette = Vec<(Surface, [u8; 3])>;

pub struct Palettes {
    ground: Palette,
    all: Palette,
}

impl Palettes {
    pub fn new() -> Palettes {
        let mut palettes = Palettes {
            ground: Vec::new(),
            all: Vec::new(),
        };

        let mut add_surface = |surface: Surface| {
            let color = surface.rgb_color();
            palettes.all.push((surface, [color.0, color.1, color.2]));

            if surface.is_ground() {
                palettes.ground.push((surface, [color.0, color.1, color.2]));
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

    pub fn nearest_surface(&self, pixel: &Rgb<u8>, ground_only: bool) -> Surface {
        let input_color = &pixel.0;

        let palette = if ground_only { &self.ground } else { &self.all };

        *palette
            .iter()
            .map(|(surface, color)| (surface, DE2000::from_rgb(color, input_color)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}
