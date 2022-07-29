use crate::common::position::{Position, SignedDeltaPos};
use crate::h3m::Terrain;

#[derive(Clone, Copy, PartialEq)]
enum TerrainRelation {
    Same, // Some neighborhood == test terrain
    Diff, // Some neighborhood != test terrain
    Any,  // any neighborhood, including None
}

const PROBLEM_PATTERN_SIZE: usize = 9;
const TEST_TERRAIN_INDEX: usize = 4;

type ProblemPattern = [TerrainRelation; PROBLEM_PATTERN_SIZE];
type Neighborhood = [Option<Terrain>; PROBLEM_PATTERN_SIZE];

fn is_terrain_relation_matched(
    test_terrain: Terrain,
    neighbour: &Option<Terrain>,
    relation: TerrainRelation,
) -> bool {
    use TerrainRelation::*;

    if relation == Any {
        return true;
    }

    if let Some(neighbour) = neighbour {
        if *neighbour == test_terrain {
            relation == Same
        } else {
            relation == Diff
        }
    } else {
        false
    }
}

fn is_problem_pattern_matched(
    neighborhood: &Neighborhood,
    problem_pattern: &ProblemPattern,
) -> bool {
    let test_terrain = neighborhood[TEST_TERRAIN_INDEX].unwrap();
    for (neighbour, &relation) in neighborhood.iter().zip(problem_pattern) {
        if !is_terrain_relation_matched(test_terrain, neighbour, relation) {
            return false;
        }
    }
    true
}

#[rustfmt::skip]
fn rotate_problem_pattern(problem_pattern: &ProblemPattern) -> ProblemPattern {
    [
        problem_pattern[6], problem_pattern[3],  problem_pattern[0],
        problem_pattern[7], problem_pattern[4],  problem_pattern[1],
        problem_pattern[8], problem_pattern[5],  problem_pattern[2],
    ]
}

fn add_problem_pattern(
    mut problem_pattern: ProblemPattern,
    rotation_count: usize,
    problem_patterns: &mut Vec<ProblemPattern>,
) {
    problem_patterns.push(problem_pattern);
    for _ in 0..rotation_count {
        problem_pattern = rotate_problem_pattern(&problem_pattern);
        problem_patterns.push(problem_pattern);
    }
}

#[rustfmt::skip]
fn problem_patterns() -> Vec<ProblemPattern> {
    use TerrainRelation::*;

    let mut problem_patterns = Vec::new();

    add_problem_pattern([
        Any,  Any,  Any,
        Diff, Same, Diff,
        Any,  Any,  Any,
    ], 1, &mut problem_patterns);

    add_problem_pattern([
        Diff, Same, Any,
        Same, Same, Same,
        Any,  Same, Diff,
    ], 1, &mut problem_patterns);

    add_problem_pattern([
        Diff, Any,  Any,
        Any,  Same, Diff,
        Any,  Diff, Any,
    ], 3, &mut problem_patterns);

    problem_patterns
}

pub struct TerrainCheck {
    size: usize,
    one_tile_water: bool,
    problem_patterns: Vec<ProblemPattern>,
}

impl TerrainCheck {
    pub fn new(size: usize, one_tile_water: bool) -> TerrainCheck {
        TerrainCheck {
            size,
            one_tile_water,
            problem_patterns: problem_patterns(),
        }
    }

    #[rustfmt::skip]
    pub fn has_problem<F>(&self, row: usize, column: usize, terrain_getter: F) -> bool
    where
        F: Fn(Position) -> Option<Terrain>,
    {
        let position = Position::new(row, column);
        let neighbour_getter = |delta_row: isize, delta_column: isize| {
            terrain_getter(position.checked_apply(
                self.size,
                self.size,
                &SignedDeltaPos::new(delta_row, delta_column),
            )?)
        };

        let test_terrain = terrain_getter(position);

        if let Some(test_terrain) = test_terrain {
            if self.problemless_terrain(test_terrain) {
                false
            } else {
                self.has_neighborhood_problem(&[
                    neighbour_getter(-1, -1), neighbour_getter(-1, 0), neighbour_getter(-1, 1),
                    neighbour_getter( 0, -1), Some(test_terrain),      neighbour_getter( 0, 1),
                    neighbour_getter( 1, -1), neighbour_getter( 1, 0), neighbour_getter( 1, 1),
                ])
            }
        } else {
            false
        }
    }

    fn problemless_terrain(&self, terrain: Terrain) -> bool {
        assert!(!Terrain::Water.is_ground());
        if terrain == Terrain::Water {
            self.one_tile_water
        } else {
            terrain.is_ground()
        }
    }

    fn has_neighborhood_problem(&self, neighborhood: &Neighborhood) -> bool {
        self.problem_patterns
            .iter()
            .any(|pattern| is_problem_pattern_matched(neighborhood, pattern))
    }
}
