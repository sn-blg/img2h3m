use std::error::Error;
use h3m::H3m;
pub use config::Config;

mod config;
mod h3m;
pub mod cli;

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    let h3m = H3m::load()?;
    h3m.save()?;
    Ok(())
}
