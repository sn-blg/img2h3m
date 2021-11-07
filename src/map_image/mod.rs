use crate::h3m::Surface;
use image::Rgb;
use palettes::Palettes;

mod palettes;

#[derive(Clone, Copy)]
struct MapPixel {
    original_color: Rgb<u8>,
    surface: Surface,
}

pub struct MapImage {
    size: usize,
    pixels: Vec<Option<MapPixel>>,
    palettes: Palettes,
}

impl MapImage {
    pub fn new(size: usize) -> MapImage {
        MapImage {
            size,
            pixels: vec![None; size * size],
            palettes: Palettes::new(),
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
        while self.fix_impl() {}
    }

    pub fn surfaces(&self) -> Vec<Option<Surface>> {
        self.pixels.iter().map(|p| p.map(|p| p.surface)).collect()
    }

    fn calc_index(&self, row: usize, column: usize) -> usize {
        row * self.size + column
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
            ],
        ) {
            return true;
        }

        if is_all_some_and_not_same(
            pixel,
            &[
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
}
