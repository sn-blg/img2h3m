pub use config::Config;
use delta_e::DE2000;
use h3m::{H3m, Surface};
use image::io::Reader as ImageReader;
use image::Rgb;
use std::error::Error;
use std::fs::File;

pub mod cli;
mod config;
mod h3m;

type Palette = Vec<(Surface, [u8; 3])>;

struct Palettes {
    safe: Palette,
    all: Palette,
}

impl Palettes {
    fn new() -> Palettes {
        let mut palettes = Palettes {
            safe: Vec::new(),
            all: Vec::new(),
        };

        let mut add_surface = |surface: Surface| {
            let color = surface.rgb_color();
            palettes.all.push((surface, [color.0, color.1, color.2]));

            if !surface.is_special() {
                palettes.safe.push((surface, [color.0, color.1, color.2]));
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

    fn nearest_surface(&self, pixel: &Rgb<u8>, safe: bool) -> Surface {
        let input_color = &pixel.0;

        let palette = if safe { &self.safe } else { &self.all };

        *palette
            .iter()
            .map(|(surface, color)| (surface, DE2000::from_rgb(color, input_color)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_map_file = File::open(&config.map_path)?;
    let mut h3m = H3m::load(input_map_file)?;

    let img = ImageReader::open(&config.image_path)?.decode()?.into_rgb8();
    let map_size = h3m.map_size();
    let palettes = Palettes::new();
    let safe = config.fix;

    for (row_id, row) in img.rows().take(map_size).enumerate() {
        for (column_id, pixel) in row.take(map_size).enumerate() {
            let surface = palettes.nearest_surface(pixel, safe);
            h3m.set_surface(row_id, column_id, config.underground, surface)?;
        }
    }

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
