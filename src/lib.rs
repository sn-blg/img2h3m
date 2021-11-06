pub use config::Config;
use h3m::H3m;
use image::io::Reader as ImageReader;
use palettes::Palettes;
use std::error::Error;
use std::fs::File;

pub mod cli;
mod config;
mod h3m;
mod palettes;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_map_file = File::open(&config.map_path)?;
    let mut h3m = H3m::load(input_map_file)?;

    let img = ImageReader::open(&config.image_path)?.decode()?.into_rgb8();
    let map_size = h3m.map_size();
    let palettes = Palettes::new();
    let ground_only = config.fix;

    for (row_id, row) in img.rows().take(map_size).enumerate() {
        for (column_id, pixel) in row.take(map_size).enumerate() {
            let surface = palettes.nearest_surface(pixel, ground_only);
            h3m.set_surface(row_id, column_id, config.underground, surface)?;
        }
    }

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
