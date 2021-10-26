use std::error::Error;

pub use config::Config;

pub mod cli;
mod config;

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
