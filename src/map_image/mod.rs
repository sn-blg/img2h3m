use crate::h3m::Surface;
use image::Rgb;
use palettes::Palettes;
use terrain_check::TerrainCheck;

mod palettes;
mod terrain_check;

#[derive(Clone, Copy)]
struct MapPixel {
    original_color: Rgb<u8>,
    surface: Surface,
}

pub struct MapImage {
    size: usize,
    pixels: Vec<Option<MapPixel>>,
    palettes: Palettes,
    terrain_check: TerrainCheck,
}

impl MapImage {
    pub fn new(size: usize, obstacles: bool) -> MapImage {
        MapImage {
            size,
            pixels: vec![None; size * size],
            palettes: Palettes::new(obstacles),
            terrain_check: TerrainCheck::new(),
        }
    }

    pub fn set_pixel(&mut self, row: usize, column: usize, pixel: Rgb<u8>) {
        let ground_only = false;
        let surface = self.palettes.nearest_surface(&pixel, ground_only);
        let index = self.calc_index(row, column);

        self.pixels[index] = Some(MapPixel {
            surface,
            original_color: pixel,
        });
    }

    pub fn fix(&mut self) {
        while self.fix_iteration() {}
    }

    pub fn surfaces(&self) -> Vec<Option<Surface>> {
        self.pixels.iter().map(|p| p.map(|p| p.surface)).collect()
    }

    fn calc_index(&self, row: usize, column: usize) -> usize {
        row * self.size + column
    }

    fn fix_problem_surface(&mut self, index: usize) {
        let ground_only = true;
        let pixel = &mut self.pixels[index];
        if let Some(pixel) = pixel {
            pixel.surface = self
                .palettes
                .nearest_surface(&pixel.original_color, ground_only);
        }
    }

    fn fix_iteration(&mut self) -> bool {
        let terrain_getter = |row: usize, column: usize| {
            if (row >= self.size) || (column >= self.size) {
                None
            } else {
                let index = self.calc_index(row, column);
                self.pixels[index].map(|p| p.surface.terrain)
            }
        };

        let mut problem_surface_indexes = Vec::new();

        for row in 0..self.size {
            for column in 0..self.size {
                let is_problem_surface =
                    self.terrain_check.has_problem(row, column, terrain_getter);

                if is_problem_surface {
                    let index = self.calc_index(row, column);
                    problem_surface_indexes.push(index);
                }
            }
        }

        let has_problems = !problem_surface_indexes.is_empty();

        for index in problem_surface_indexes {
            self.fix_problem_surface(index);
        }

        has_problems
    }
}
