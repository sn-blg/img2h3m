use crate::h3m::result::*;
use crate::h3m::Surface;
use draft_terrain_map::DraftTerrainMap;
pub use map_cell::MapCell;

mod draft_terrain_map;
mod map_cell;
mod tile;

pub struct TerrainMap {
    size: usize,
    underground: bool,
    has_obstacles: bool,
    cells: Vec<Option<MapCell>>,
}

impl TerrainMap {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn underground(&self) -> bool {
        self.underground
    }

    pub fn has_obstacles(&self) -> bool {
        self.has_obstacles
    }

    pub fn cells(&self) -> &[Option<MapCell>] {
        &self.cells
    }

    pub fn generate(
        size: usize,
        one_tile_water: bool,
        underground: bool,
        surfaces: &[Option<Surface>],
    ) -> H3mResult<TerrainMap> {
        let map_len = size * size;
        if surfaces.len() != map_len {
            return Err(H3mError::Parameter(ParameterError::new(format!(
                "surfaces length ({}) not equal map length ({}).",
                surfaces.len(),
                map_len
            ))));
        }

        let mut draft_terrain_map = DraftTerrainMap::new(size, surfaces);
        draft_terrain_map.set_tile_codes(one_tile_water);

        Ok(TerrainMap {
            size,
            underground,
            has_obstacles: surfaces
                .iter()
                .map(|s| if let Some(s) = s { s.obstacle } else { false })
                .any(|obstacle| obstacle),
            cells: draft_terrain_map.into_map_cells(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h3m::Terrain;
    use strum::IntoEnumIterator;

    impl Default for Surface {
        fn default() -> Self {
            Surface {
                terrain: Terrain::Dirt,
                obstacle: false,
            }
        }
    }

    struct Surfaces(Vec<Option<Surface>>);
    impl Surfaces {
        fn update_element(&mut self, index: usize, overflow: &mut bool) {
            let terrain = self.0[index].unwrap().terrain;
            let mut iter = Terrain::iter().skip_while(|&t| t != terrain);
            assert_eq!(iter.next(), Some(terrain));
            let next_terrain = {
                let next_terrain = iter.next();
                if next_terrain == Some(Terrain::Rock) {
                    iter.next()
                } else {
                    next_terrain
                }
            };
            let next_terrain = {
                if let Some(next_terrain) = next_terrain {
                    *overflow = false;
                    next_terrain
                } else {
                    *overflow = true;
                    Terrain::iter().next().unwrap()
                }
            };
            self.0[index] = Some(Surface {
                terrain: next_terrain,
                obstacle: false,
            });
        }

        fn next(&mut self, overflow: &mut bool) {
            let len = self.0.len();
            for index in 0..len {
                self.update_element(index, overflow);
                if !*overflow {
                    break;
                }
            }
        }

        fn print(&self) {
            let len = self.0.len();
            print!("[  ");
            for index in 0..len {
                let terrain = self.0[index].unwrap().terrain;
                print!("{:?}  ", terrain);
            }
            println!("  ]");
        }
    }

    #[test]
    fn generate_map_for_various_combinations_of_surfaces() {
        let one_tile_water = true;
        let underground = false;

        let size = 3;
        let mut surfaces = Surfaces(vec![Some(Surface::default()); size * size]);
        let mut overflow = false;

        while !overflow {
            surfaces.print();
            assert!(TerrainMap::generate(size, one_tile_water, underground, &surfaces.0).is_ok());
            surfaces.next(&mut overflow);
        }
        println!("---");
        surfaces.print();
    }
}
