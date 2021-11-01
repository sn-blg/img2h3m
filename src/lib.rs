pub use config::Config;
use h3m::{H3m, Surface};
use std::error::Error;
use std::fs::File;

pub mod cli;
mod config;
mod h3m;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _image_file = File::open(config.image_path)?;

    let input_map_file = File::open(&config.map_path)?;
    let mut h3m = H3m::load(input_map_file)?;

    //println!("map size = {}", h3m.size() as u32);
    //println!("land offset = {}", h3m.land_offset());

    let length = h3m.size() as usize * h3m.size() as usize;
    for i in 0..length {
        h3m.set_land(i, Surface::Wasteland)?;
    }

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
