use crate::h3m::Surface;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SurfaceRelation {
    Same, // Some neighborhood == test surface
    Diff, // Some neighborhood != test surface
    Any,  // any neighborhood, including None
}

pub const PROBLEM_PATTERN_SIZE: usize = 9;
const TEST_SURFACE_INDEX: usize = 4;

type ProblemPattern = [SurfaceRelation; PROBLEM_PATTERN_SIZE];
pub type Neighborhood = [Option<Surface>; PROBLEM_PATTERN_SIZE];

fn is_surface_relation_matched(
    test_surface: Surface,
    neighbour: &Option<Surface>,
    relation: SurfaceRelation,
) -> bool {
    use SurfaceRelation::*;

    if relation == Any {
        return true;
    }

    if let Some(neighbour) = neighbour {
        if *neighbour == test_surface {
            relation == Same
        } else {
            relation == Diff
        }
    } else {
        false
    }
}

fn is_pattern_matched(neighborhood: &Neighborhood, problem_pattern: &ProblemPattern) -> bool {
    let test_surface = neighborhood[TEST_SURFACE_INDEX].unwrap();
    for (neighbour, &relation) in neighborhood.iter().zip(problem_pattern) {
        if !is_surface_relation_matched(test_surface, neighbour, relation) {
            return false;
        }
    }
    true
}

#[rustfmt::skip]
fn surface_problem_patterns() -> Vec<ProblemPattern> {
    use SurfaceRelation::*;
    vec![
        [
            Any,  Any,  Any,
            Diff, Same, Diff,
            Any,  Any,  Any,
        ],
        [
            Diff, Same, Any,
            Same, Same, Same,
            Any,  Same, Diff,
        ],
        [
            Diff, Any,  Any,
            Any,  Same, Diff,
            Any,  Diff, Any,
        ],
/////////////////////////////////////////////////////////
        [
            Any,  Diff, Any,
            Any,  Same, Any,
            Any,  Diff, Any,
        ],
        [
            Any,  Same, Diff,
            Same, Same, Same,
            Diff, Same, Any,
        ],
        [
            Any,  Any,  Diff,
            Diff, Same, Any,
            Any,  Diff, Any,
        ],
        [
            Any,  Diff, Any,
            Any,  Same, Diff,
            Diff, Any,  Any,
        ],
        [
            Any,  Diff, Any,
            Diff, Same, Any,
            Any,  Any,  Diff,
        ],
/////////////////////////////////////////////////////////
    ]
}

pub struct SurfaceCheck {
    problem_patterns: Vec<ProblemPattern>,
}

impl SurfaceCheck {
    pub fn new() -> SurfaceCheck {
        SurfaceCheck {
            problem_patterns: surface_problem_patterns(),
        }
    }

    #[rustfmt::skip]
    pub fn has_problem<F>(&self, row: usize, column: usize, surface_getter: F) -> bool
    where
        F: Fn(usize, usize) -> Option<Surface>,
    {
        let neighbour_getter = |delta_row: i32, delta_column: i32| {
            let row = SurfaceCheck::checked_delta_add(row, delta_row)?;
            let column = SurfaceCheck::checked_delta_add(column, delta_column)?;
            surface_getter(row, column)
        };

        let test_surface = surface_getter(row, column);

        if let Some(test_surface) = test_surface {
            if test_surface.is_ground() {
                return false;
            }
            self.has_neighborhood_problem(&[
                neighbour_getter(-1, -1), neighbour_getter(-1, 0), neighbour_getter(-1, 1),
                neighbour_getter( 0, -1), Some(test_surface),      neighbour_getter( 0, 1),
                neighbour_getter( 1, -1), neighbour_getter( 1, 0), neighbour_getter( 1, 1),
            ])
        } else {
            false
        }
    }

    fn has_neighborhood_problem(&self, neighborhood: &Neighborhood) -> bool {
        self.problem_patterns
            .iter()
            .any(|pattern| is_pattern_matched(neighborhood, pattern))
    }

    fn checked_delta_add(val: usize, delta: i32) -> Option<usize> {
        let delta_abs = usize::try_from(delta.abs()).unwrap();
        if delta.is_negative() {
            val.checked_sub(delta_abs)
        } else {
            Some(val.checked_add(delta_abs).unwrap())
        }
    }
}
