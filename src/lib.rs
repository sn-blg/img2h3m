pub use config::Config;
use h3m::{H3m, Surface};
use image::io::Reader as ImageReader;
use image::Rgb;
use palettes::Palettes;
use std::error::Error;
use std::fs::File;

pub mod cli;
mod config;
mod h3m;
mod palettes;

#[derive(Clone, Copy)]
struct MapPixel {
    original_color: Rgb<u8>,
    surface: Surface,
}

impl MapPixel {
    fn new(original_color: Rgb<u8>, surface: Surface) -> MapPixel {
        MapPixel {
            surface,
            original_color,
        }
    }
}

struct Map {
    size: usize,
    pixels: Vec<Option<MapPixel>>,
    palettes: Palettes,
}

impl Map {
    fn new(size: usize) -> Map {
        Map {
            size,
            pixels: vec![None; size * size],
            palettes: Palettes::new(),
        }
    }

    fn set_pixel(&mut self, row: usize, column: usize, pixel: Rgb<u8>) {
        let ground_only = false;
        let surface = self.palettes.nearest_surface(&pixel, ground_only);
        let index = row * self.size + column;

        self.pixels[index] = Some(MapPixel::new(pixel, surface));
    }

    fn fix(&mut self) {
        let ground_only = true;
        for pixel in self.pixels.iter_mut().flatten() {
            if !pixel.surface.is_ground() {
                pixel.surface = self
                    .palettes
                    .nearest_surface(&pixel.original_color, ground_only);
            }
        }
    }

    fn into_surfaces(self) -> Vec<Option<Surface>> {
        self.pixels
            .into_iter()
            .map(|p| p.map(|p| p.surface))
            .collect()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_map_file = File::open(&config.map_path)?;
    let mut h3m = H3m::load(input_map_file)?;

    let img = ImageReader::open(&config.image_path)?.decode()?.into_rgb8();
    let map_size = h3m.map_size();
    let mut map = Map::new(map_size);

    for (row_id, row) in img.rows().take(map_size).enumerate() {
        for (column_id, pixel) in row.take(map_size).enumerate() {
            map.set_pixel(row_id, column_id, *pixel);
        }
    }

    if config.fix {
        map.fix();
    }

    for (index, surface) in map.into_surfaces().into_iter().enumerate() {
        if let Some(surface) = surface {
            h3m.set_surface_by_index(index, config.underground, surface)?;
        }
    }

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
