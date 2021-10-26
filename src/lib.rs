pub use config::Config;
use h3m::H3m;
use std::error::Error;
use std::fs::File;

pub mod cli;
mod config;
mod h3m;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _image_file = File::open(config.image_path)?;

    let input_map_file = File::open(&config.map_path)?;
    let h3m = H3m::load(input_map_file)?;

    let output_map_file = File::create(&config.map_path)?;
    h3m.save(output_map_file)?;
    Ok(())
}
