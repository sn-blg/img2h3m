pub use config::Config;
use h3m::H3m;
use image::io::Reader as ImageReader;
use map_image::MapImage;
use std::error::Error;
use std::fs::File;

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

fn set_image(
    h3m: &mut H3m,
    image_path: impl Into<String>,
    underground: bool,
    fix: bool,
    obstacles: bool,
) -> Result<(), Box<dyn Error>> {
    let mut map_image = make_map_image(h3m.map_size(), obstacles, image_path)?;

    if fix {
        map_image.fix();
    }

    let surfaces = map_image.surfaces();
    h3m.set_surfaces(underground, &surfaces)?;

    if obstacles {
        let obstacles = map_image.obstacles();
        h3m.set_obstacles(underground, &obstacles)?;
    }

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_map_file = File::open(&config.map_path)?;
    let mut h3m = H3m::load(input_map_file)?;

    if let Some(land_image_path) = config.land_image_path {
        set_image(
            &mut h3m,
            land_image_path,
            false,
            config.fix,
            config.obstacles,
        )?;
    }

    if let Some(underground_image_path) = config.underground_image_path {
        set_image(
            &mut h3m,
            underground_image_path,
            true,
            config.fix,
            config.obstacles,
        )?;
    }

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
