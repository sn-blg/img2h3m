pub use config::Config;
use h3m::H3m;
use image::io::Reader as ImageReader;
use map_image::MapImage;
use std::error::Error;
use std::fs::File;

pub mod cli;
mod config;
mod h3m;
mod map_image;

fn make_map_image(
    map_size: usize,
    obstacles: bool,
    image_path: impl Into<String>,
) -> Result<MapImage, Box<dyn Error>> {
    let mut map_image = MapImage::new(map_size, obstacles);
    let img = ImageReader::open(image_path.into())?.decode()?.into_rgb8();

    for (row_id, row) in img.rows().take(map_size).enumerate() {
        for (column_id, pixel) in row.take(map_size).enumerate() {
            map_image.set_pixel(row_id, column_id, *pixel);
        }
    }
    Ok(map_image)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_map_file = File::open(&config.map_path)?;
    let mut h3m = H3m::load(input_map_file)?;

    let mut map_image = make_map_image(h3m.map_size(), config.obstacles, &config.image_path)?;

    if config.fix {
        map_image.fix();
    }

    let surfaces = map_image.surfaces();
    h3m.set_surfaces(config.underground, &surfaces)?;

    if config.obstacles {
        let obstacles = map_image.obstacles();
        h3m.set_obstacles(config.underground, &obstacles)?;
    }

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
