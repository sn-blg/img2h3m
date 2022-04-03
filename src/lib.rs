use common::RgbColor;
pub use config::Config;
use h3m::H3m;
use image::io::Reader as ImageReader;
use image::Rgb;
use map_image::MapImage;
use std::error::Error;
use std::fs::File;

mod common;
mod config;
mod h3m;
mod map_image;

impl MapImage {
    fn from_image(
        image_path: impl Into<String>,
        map_size: usize,
        one_tile_water: bool,
        obstacles: bool,
        transparent_color: Option<RgbColor>,
    ) -> Result<MapImage, Box<dyn Error>> {
        let mut map_image = MapImage::new(map_size, one_tile_water, obstacles);
        let img = ImageReader::open(image_path.into())?.decode()?.into_rgb8();
        let is_transparent_color = |pixel: &Rgb<u8>| {
            if transparent_color.is_none() {
                false
            } else {
                Some(pixel.0) == transparent_color
            }
        };

        for (row_id, row) in img.rows().take(map_size).enumerate() {
            for (column_id, pixel) in row.take(map_size).enumerate() {
                if is_transparent_color(pixel) {
                    continue;
                }
                map_image.set_pixel(row_id, column_id, *pixel);
            }
        }
        Ok(map_image)
    }
}

impl H3m {
    fn set_image(
        &mut self,
        image_path: impl Into<String>,
        underground: bool,
        one_tile_water: bool,
        obstacles: bool,
        transparent_color: Option<RgbColor>,
    ) -> Result<(), Box<dyn Error>> {
        let mut map_image = MapImage::from_image(
            image_path,
            self.map_size(),
            one_tile_water,
            obstacles,
            transparent_color,
        )?;
        map_image.fix();
        let surfaces = map_image.surfaces();
        self.set_surfaces(one_tile_water, obstacles, underground, &surfaces)?;

        Ok(())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_map_file = File::open(&config.map_path)?;
    let mut h3m = H3m::load(input_map_file)?;

    if let Some(land_image_path) = config.land_image_path {
        h3m.set_image(
            land_image_path,
            false,
            config.one_tile_water,
            config.obstacles,
            config.transparent_color,
        )?;
    }

    if let Some(underground_image_path) = config.underground_image_path {
        h3m.set_image(
            underground_image_path,
            true,
            config.one_tile_water,
            config.obstacles,
            config.transparent_color,
        )?;
    }

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
