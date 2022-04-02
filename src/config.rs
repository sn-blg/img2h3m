use crate::common::RgbColor;

pub struct Config {
    pub land_image_path: Option<String>,
    pub underground_image_path: Option<String>,
    pub map_path: String,
    pub obstacles: bool,
    pub one_tile_water: bool,
    pub transparent_color: Option<RgbColor>,
}
