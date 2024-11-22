use crate::common::{AocError, InputType};
use log::{debug, trace};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub fn part1() -> Result<String, AocError> {
    let area = simple_solve(InputType::Challenge);
    assert_eq!(area, 4548);
    Ok(format!("{area}"))
}

pub fn part2() -> Result<String, AocError> {
    let area = count_exposed_faces_reachable_from_outside(InputType::Challenge);
    assert_eq!(area, 2588);
    Ok(format!("{area}"))
}

/* x (+=up), y (+=right), z (+=away) */
#[derive(Debug, Clone)]
struct Point(f32, f32, f32);

fn cube_face_centers(cube: &Point) -> Vec<Point> {
    vec![
        Point(cube.0, cube.1, cube.2 - 0.5), // front
        Point(cube.0, cube.1, cube.2 + 0.5), // back
        Point(cube.0, cube.1 - 0.5, cube.2), // bottom
        Point(cube.0, cube.1 + 0.5, cube.2), // top
        Point(cube.0 - 0.5, cube.1, cube.2), // left
        Point(cube.0 + 0.5, cube.1, cube.2), // right
    ]
}

fn cell_neighbors(cell: &Point) -> Vec<Point> {
    vec![
        Point(cell.0 - 1.0, cell.1, cell.2), // left
        Point(cell.0 + 1.0, cell.1, cell.2), // right
        Point(cell.0, cell.1 - 1.0, cell.2), // down
        Point(cell.0, cell.1 + 1.0, cell.2), // up
        Point(cell.0, cell.1, cell.2 + 1.0), // back
        Point(cell.0, cell.1, cell.2 - 1.0), // front
    ]
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}", self).hash(state);
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
impl Eq for Point {}

fn load_cubes(input_type: InputType) -> Result<Vec<Point>, AocError> {
    Ok(input_type
        .get_input_lines(18)?
        .iter()
        .map(|l| {
            let values: Vec<f32> = l.split(',').map(|s| s.parse::<f32>().unwrap()).collect();
            Point(values[0], values[1], values[2])
        })
        .collect::<Vec<_>>())
}

fn cube_face_histogram(cubes: &Vec<Point>) -> HashMap<Point, usize> {
    let mut face_counts = HashMap::<Point, usize>::new();
    for cube in cubes {
        for face_center in cube_face_centers(cube) {
            match face_counts.get_mut(&face_center) {
                Some(p) => *p += 1,
                None => {
                    face_counts.insert(face_center, 1);
                }
            }
        }
    }
    face_counts
}

fn simple_solve(input_type: InputType) -> usize {
    let input = load_cubes(input_type).unwrap();
    trace!("input={:?}", input);
    count_exposed_faces(&input)
}

fn count_exposed_faces(input: &Vec<Point>) -> usize {
    let histogram = cube_face_histogram(&input);
    trace!("faces {:?}", histogram);
    debug!("faces size={}", histogram.len());
    histogram.iter().filter(|e| *e.1 == 1).count()
}

fn count_exposed_faces_reachable_from_outside(input_type: InputType) -> usize {
    let input = load_cubes(input_type).unwrap();
    trace!("input={:?}", input);

    let mut search = OpenToOutsideSearch::new(&input);

    // fill in all search 3d-grid cells values
    for x in search.min.0..=search.max.0 {
        for y in search.min.1..=search.max.1 {
            for z in search.min.2..=search.max.2 {
                let cell = Point(x as f32, y as f32, z as f32);

                search.is_open_to_outside(cell);
            }
        }
    }

    let closed_to_outside = search
        .open_to_outside
        .iter()
        .filter(|e| *e.1 == false)
        .map(|e| e.0.clone())
        .collect::<Vec<_>>();
    debug!("CLOSED to outside: {:?}", closed_to_outside);

    let closed_to_outside_not_cube = closed_to_outside
        .iter()
        .filter(|c| !search.cube_set.contains(c))
        .map(|c| c.clone())
        .collect::<Vec<_>>();
    debug!(
        "CLOSED to outside & not cube: {:?}",
        closed_to_outside_not_cube
    );

    let simple_solution = count_exposed_faces(&input);

    // reuse simple solution to determine number of faces on the voids with in the droplet
    let num_faces_to_remove = count_exposed_faces(&closed_to_outside_not_cube);

    simple_solution - num_faces_to_remove
}

/**
 * Helps determine if a point has a path to the outside by moving from cell face to cell face until it gets to cells known to be outside each drople.
 */
struct OpenToOutsideSearch {
    min: (i32, i32, i32),
    max: (i32, i32, i32),
    cube_set: HashSet<Point>,
    open_to_outside: HashMap<Point, bool>,
    visiting: HashSet<Point>,
}
impl OpenToOutsideSearch {
    fn new(cube_centers: &[Point]) -> Self {
        let mut min = (0_i32, 0_i32, 0_i32);
        let mut max = (0_i32, 0_i32, 0_i32);
        for cube in cube_centers {
            min.0 = min.0.min(cube.0 as i32);
            min.1 = min.1.min(cube.1 as i32);
            min.2 = min.2.min(cube.2 as i32);

            max.0 = max.0.max(cube.0 as i32);
            max.1 = max.1.max(cube.1 as i32);
            max.2 = max.2.max(cube.2 as i32);
        }

        debug!("Min={:?}, max={:?}", min, max);

        let mut cube_set = HashSet::<Point>::new();
        for cube in cube_centers {
            cube_set.insert(cube.clone());
        }

        let mut open_to_outside = HashMap::<Point, bool>::new();

        // pre record that each cube is not open to the outside
        for cube in cube_centers {
            open_to_outside.insert(cube.clone(), false);
        }

        // surround the entire droplet with a box of cells that are marked as open to the outside so the path-to-open search has somewhere to terminate
        for x in min.0 - 1..=max.0 + 1 {
            for y in min.1 - 1..=max.1 + 1 {
                for z in min.2 - 1..=max.2 + 1 {
                    if x + 1 == min.0
                        || x - 1 == max.0
                        || y + 1 == min.1
                        || y - 1 == max.1
                        || z + 1 == min.2
                        || z - 1 == max.2
                    {
                        open_to_outside.insert(Point(x as f32, y as f32, z as f32), true);
                    }
                }
            }
        }

        Self {
            min,
            max,
            cube_set,
            open_to_outside,
            visiting: HashSet::<Point>::new(),
        }
    }

    // recursively, breadth first, scan for a path from a cell that hasn't been evaluated to cells already marked as open to the outside
    fn is_open_to_outside(&mut self, cell: Point) -> bool {
        if let Some(value) = self.open_to_outside.get(&cell) {
            return *value;
        }

        // prevents re
        self.visiting.insert(cell.clone());

        // println!("{:?}", self.visiting);
        let mut has_path_to_outside = false;
        for neighbor in cell_neighbors(&cell) {
            if self.visiting.contains(&neighbor) {
                // don't check a neighbor that's being evaluated at recursion level above is.
                continue;
            }
            if self.is_open_to_outside(neighbor) {
                has_path_to_outside = true;
                break;
            }
        }

        self.visiting.remove(&cell);

        self.open_to_outside.insert(cell, has_path_to_outside);
        has_path_to_outside
    }
}

#[cfg(test)]
mod tests {
    use super::{count_exposed_faces_reachable_from_outside, simple_solve};
    use crate::common::InputType;

    #[test]
    fn test_part1_example() {
        let area = simple_solve(InputType::Example);
        assert_eq!(area, 64);
    }

    #[test]
    fn test_part2_example() {
        let area = count_exposed_faces_reachable_from_outside(InputType::Example);
        assert_eq!(area, 58);
    }
}
