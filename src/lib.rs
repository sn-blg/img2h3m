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

    fn calc_index(&self, row: usize, column: usize) -> usize {
        row * self.size + column
    }

    fn set_pixel(&mut self, row: usize, column: usize, pixel: Rgb<u8>) {
        let ground_only = false;
        let surface = self.palettes.nearest_surface(&pixel, ground_only);
        let index = self.calc_index(row, column);

        self.pixels[index] = Some(MapPixel {
            surface,
            original_color: pixel,
        });
    }

    fn try_get_pixel(&self, row: i64, column: i64) -> Option<MapPixel> {
        if (row < 0) || (column < 0) {
            return None;
        }

        let row = row as usize;
        let column = column as usize;

        if (row >= self.size) || (column >= self.size) {
            return None;
        }

        let index = self.calc_index(row, column);
        self.pixels[index]
    }

    fn is_problem_pixel(&self, row: usize, column: usize, pixel: &MapPixel) -> bool {
        fn test_pixel_neighbors(
            base: &MapPixel,
            same: bool,
            neighbors: &[Option<MapPixel>],
        ) -> bool {
            for neighbour in neighbors {
                if let Some(neighbour) = neighbour {
                    let is_same_neighbour = neighbour.surface == base.surface;
                    if (same && !is_same_neighbour) || (!same && is_same_neighbour) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }

        fn is_all_some_and_not_same(base: &MapPixel, neighbors: &[Option<MapPixel>]) -> bool {
            test_pixel_neighbors(base, false, neighbors)
        }

        fn is_all_some_and_same(base: &MapPixel, neighbors: &[Option<MapPixel>]) -> bool {
            test_pixel_neighbors(base, true, neighbors)
        }

        if pixel.surface.is_ground() {
            return false;
        }

        let row = row as i64;
        let column = column as i64;

        if is_all_some_and_not_same(
            pixel,
            &[
                self.try_get_pixel(row - 1, column),
                self.try_get_pixel(row + 1, column),
            ],
        ) {
            return true;
        }

        if is_all_some_and_not_same(
            pixel,
            &[
                self.try_get_pixel(row, column - 1),
                self.try_get_pixel(row, column + 1),
            ],
        ) {
            return true;
        }

        ////////////
        if is_all_some_and_not_same(
            pixel,
            &[
                self.try_get_pixel(row - 1, column - 1),
                self.try_get_pixel(row - 1, column),
                self.try_get_pixel(row, column - 1),
                self.try_get_pixel(row + 1, column + 1),
            ],
        ) {
            return true;
        }

        if is_all_some_and_not_same(
            pixel,
            &[
                self.try_get_pixel(row - 1, column - 1),
                self.try_get_pixel(row + 1, column),
                self.try_get_pixel(row, column + 1),
                self.try_get_pixel(row + 1, column + 1),
            ],
        ) {
            return true;
        }

        if is_all_some_and_not_same(
            pixel,
            &[
                self.try_get_pixel(row + 1, column - 1),
                self.try_get_pixel(row, column - 1),
                self.try_get_pixel(row + 1, column),
                self.try_get_pixel(row - 1, column + 1),
            ],
        ) {
            return true;
        }

        if is_all_some_and_not_same(
            pixel,
            &[
                self.try_get_pixel(row + 1, column - 1),
                self.try_get_pixel(row - 1, column),
                self.try_get_pixel(row, column + 1),
                self.try_get_pixel(row - 1, column + 1),
            ],
        ) {
            return true;
        }
        ///////////////

        if is_all_some_and_same(
            pixel,
            &[
                self.try_get_pixel(row, column - 1),
                self.try_get_pixel(row, column + 1),
                self.try_get_pixel(row - 1, column),
                self.try_get_pixel(row + 1, column),
            ],
        ) {
            if is_all_some_and_not_same(
                pixel,
                &[
                    self.try_get_pixel(row - 1, column - 1),
                    self.try_get_pixel(row + 1, column + 1),
                ],
            ) {
                return true;
            }

            if is_all_some_and_not_same(
                pixel,
                &[
                    self.try_get_pixel(row - 1, column + 1),
                    self.try_get_pixel(row + 1, column - 1),
                ],
            ) {
                return true;
            }
        }

        false
    }

    fn fix_problem_pixel(&mut self, index: usize) {
        let ground_only = true;
        let pixel = &mut self.pixels[index];
        if let Some(pixel) = pixel {
            pixel.surface = self
                .palettes
                .nearest_surface(&pixel.original_color, ground_only);
        }
    }

    fn fix_impl(&mut self) -> bool {
        let mut has_problem = false;
        for row in 0..self.size {
            for column in 0..self.size {
                let index = self.calc_index(row, column);
                let pixel = self.pixels[index];

                if let Some(pixel) = pixel {
                    if self.is_problem_pixel(row, column, &pixel) {
                        has_problem = true;
                        self.fix_problem_pixel(index);
                    }
                }
            }
        }
        has_problem
    }

    fn fix(&mut self) {
        while self.fix_impl() {}
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
