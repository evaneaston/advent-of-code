use std::{collections::HashSet, fmt::Display};

use log::debug;

use crate::{grid::Grid, AocError, DailyInput, RowCol, XY};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Symbol {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
impl Symbol {
    fn is_corner(&self) -> bool {
        matches!(self, Symbol::NE | Symbol::NW | Symbol::SW | Symbol::SE)
    }
}
impl From<u8> for Symbol {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Self::NS,
            b'-' => Self::EW,
            b'L' => Self::NE,
            b'J' => Self::NW,
            b'7' => Self::SW,
            b'F' => Self::SE,
            b'.' => Self::Ground,
            b'S' => Self::Start,
            _ => panic!("Invalid symbol {}", value),
        }
    }
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::NS => '|',
                Symbol::EW => '-',
                Symbol::NE => 'L',
                Symbol::NW => 'J',
                Symbol::SW => '7',
                Symbol::SE => 'F',
                Symbol::Ground => '.',
                Symbol::Start => 'S',
            }
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    E,
    S,
    W,
    N,
    None,
    Ambiguous,
}

pub struct PipeIterator<'a> {
    grid: &'a Grid,
    current: RowCol,
    direction: Direction,
}
impl<'a> PipeIterator<'a> {
    fn current(&self) -> Option<(Symbol, RowCol, Direction)> {
        let current_symbol = match self.grid.get(self.current) {
            Some(rc) => Symbol::from(rc),
            None => return None,
        };
        Some((current_symbol, self.current, self.direction))
    }
}
impl<'a> Iterator for PipeIterator<'a> {
    type Item = (Symbol, RowCol, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.current();

        if last.is_some() && self.direction == Direction::Ambiguous {
            self.direction = Direction::None;
            debug!("  {:?}", last);

            return last;
        }

        let next_row_col = match self.direction {
            Direction::E => self.current.plus_col(),
            Direction::S => self.current.plus_row(),
            Direction::W => self.current.minus_col(),
            Direction::N => self.current.minus_row(),
            Direction::None => return None,
            Direction::Ambiguous => self.current,
        };

        let next_symbol = match self.grid.get(next_row_col) {
            Some(rc) => Symbol::from(rc),
            None => return None, // off of board
        };

        let (can_move, next_direction) = match next_symbol {
            Symbol::Ground => (false, Direction::None),
            Symbol::Start => (true, Direction::Ambiguous),

            Symbol::NS => {
                if self.direction == Direction::N {
                    (true, Direction::N)
                } else if self.direction == Direction::S {
                    (true, Direction::S)
                } else {
                    (false, Direction::None)
                }
            }

            Symbol::EW => {
                if self.direction == Direction::E {
                    (true, Direction::E)
                } else if self.direction == Direction::W {
                    (true, Direction::W)
                } else {
                    (false, Direction::None)
                }
            }

            Symbol::NE => {
                if self.direction == Direction::S {
                    (true, Direction::E)
                } else if self.direction == Direction::W {
                    (true, Direction::N)
                } else {
                    (false, Direction::None)
                }
            }
            Symbol::NW => {
                if self.direction == Direction::S {
                    (true, Direction::W)
                } else if self.direction == Direction::E {
                    (true, Direction::N)
                } else {
                    (false, Direction::None)
                }
            }
            Symbol::SW => {
                if self.direction == Direction::N {
                    (true, Direction::W)
                } else if self.direction == Direction::E {
                    (true, Direction::S)
                } else {
                    (false, Direction::None)
                }
            }
            Symbol::SE => {
                if self.direction == Direction::N {
                    (true, Direction::E)
                } else if self.direction == Direction::W {
                    (true, Direction::S)
                } else {
                    (false, Direction::None)
                }
            }
        };

        self.direction = next_direction;
        let ret = if can_move {
            self.current = next_row_col;
            last
        } else {
            None
        };
        debug!("  {:?}", ret);
        ret
    }
}

fn find_start(grid: &Grid) -> RowCol {
    let to_find = HashSet::<u8>::from([b'S']);
    let found = grid.find(to_find);

    debug!("Start @ {:?}", found);
    let start_locations = found.get(&b'S').expect("Expected to find 'S' in the grid");
    if start_locations.len() != 1 {
        panic!("Expected a single start location");
    }
    *start_locations.first().unwrap()
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let grid = Grid::new(&lines);

    let start_row_col = find_start(&grid);
    let loop_lengths = [Direction::E, Direction::W, Direction::S, Direction::N]
        .iter()
        .map(|dir| {
            let mut iter = PipeIterator {
                grid: &grid,
                current: start_row_col,
                direction: *dir,
            };
            iter.next(); // move off the start in the chosen direction

            // attempt to find next start
            iter.position(|e| e.0 == Symbol::Start).map(|pos| pos + 1)
        })
        .collect::<Vec<_>>();

    debug!("Lengths: {:?}", loop_lengths);

    let max_loop_len = *loop_lengths.iter().flatten().max().unwrap() as f32;
    let halfway = (max_loop_len / 2_f32).ceil() as i64;

    Ok(halfway.to_string())
}

// https://en.m.wikipedia.org/wiki/Shoelace_formula
// 1/2 Σ i->n (x[i]* y[i+1] - x[i+1]*y[i])
// this should work even if XY were changed to use non integers
pub(crate) fn shoelace_area(vertices: &[XY]) -> f64 {
    let mut sum = 0_f64;
    for i in 0..vertices.len() {
        let (v1, v2) = (
            vertices[i],
            if i == vertices.len() - 1 {
                vertices[0]
            } else {
                vertices[i + 1]
            },
        );

        sum += (v1.x() * v2.y() - v1.y() * v2.x()) as f64;
    }
    (sum / 2.).abs()
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem
//
// Pick's   A = i + b/2 - 1
//  A=area
//  i=iterior points
//  b=boundary points
//
// i= A+1-b/2
// This only works with integer vertices
pub(crate) fn get_num_interior_points(vertices: &[XY]) -> i64 {
    let vertices = &vertices;
    let area = shoelace_area(vertices);
    debug!(" shoelace area={area}");

    let mut boundary_points_not_in_vertices = 0_usize;

    let mut looped: Vec<XY> = Vec::from(*vertices);
    looped.push(looped[0]);

    for p in looped.windows(2) {
        let a = p[0];
        let b = p[1];
        if a.x() == b.x() {
            let num_missing = ((b.y() - a.y()).abs() - 1).max(0);
            debug!(" Between {:?} and {:?} there are {num_missing}", a, b);
            boundary_points_not_in_vertices += num_missing as usize;
        } else if a.y() == b.y() {
            let num_missing = ((b.x() - a.x()).abs() - 1).max(0) ;
            debug!(" Between {:?} and {:?} there are {num_missing}", a, b);
            boundary_points_not_in_vertices += num_missing as usize;
        } else {
            // todo if we ever need angled ones, find integer intersections
            panic!("Assumed no angled edges");
        }
    }

    debug!(" boundary_points_not_in_vertices={boundary_points_not_in_vertices}");

    let num_boundary_points = vertices.len() + boundary_points_not_in_vertices;
    let num_interior_points = area + 1_f64 - num_boundary_points as f64 / 2.;

    debug!(" num_boundary_points={num_boundary_points}");
    debug!(" num_interior_points={num_interior_points}");

    num_interior_points.round() as i64
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let grid = Grid::new(&lines);

    let start_row_col = find_start(&grid);
    let first_loop = [Direction::E, Direction::W, Direction::S, Direction::N]
        .iter()
        .map(|dir| {
            let iter = PipeIterator {
                grid: &grid,
                current: start_row_col,
                direction: *dir,
            };

            let path_positions: Vec<(Symbol, RowCol, Direction)> = iter.fold(vec![], |mut vec, val| {
                vec.push(val);
                vec
            });
            path_positions
        })
        .find(|l| !l.is_empty())
        .unwrap();

    debug!(" first_loop {:?}", first_loop);

    let vertices = first_loop
        .iter()
        .enumerate()
        .filter(|(index,(symbol, _,_))| /* include the starting point */ *index == 0 || symbol.is_corner())
        .map(|(_, (_, rc, _))| XY::from(*rc))
        .collect::<Vec<_>>();
    debug!(" vertices: {:?}", vertices);

    assert!(vertices.first().unwrap().1 != vertices.last().unwrap().1);

    let num_interior_points = get_num_interior_points(&vertices);
    debug!(" num_interior_points={num_interior_points}");

    Ok(num_interior_points.to_string())
}

#[cfg(test)]
mod test {
    use crate::{
        day10::{get_num_interior_points, part1, part2, shoelace_area},
        DailyInput, InputType, XY,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 10,
                input_type: InputType::Example,
                number: Some(1),
            })
            .unwrap(),
            "4"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 10,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "6717"
        );
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(
            part2(DailyInput {
                day: 10,
                input_type: InputType::Example,
                number: Some(2),
            })
            .unwrap(),
            "4"
        );
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(
            part2(DailyInput {
                day: 10,
                input_type: InputType::Example,
                number: Some(3),
            })
            .unwrap(),
            "8"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 10,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "381"
        );
    }

    #[test]
    fn test_shoelace_area() {
        assert_eq!(
            shoelace_area(&[
                XY::new(1, 6),
                XY::new(3, 1),
                XY::new(7, 2),
                XY::new(4, 4),
                XY::new(8, 5),
            ]),
            16.5
        );
    }

    #[test]
    fn test_picks() {
        // assert_eq!(
        //     get_num_interior_points(&[
        //         XY::new(1, 6),
        //         XY::new(3, 1),
        //         XY::new(7, 2),
        //         XY::new(4, 4),
        //         XY::new(8, 5),
        //     ]),
        //     15
        // );

        // day2 first example
        assert_eq!(
            get_num_interior_points(&[
                XY::new(1, 1),
                XY::new(9, 1),
                XY::new(9, 7),
                XY::new(6, 7),
                XY::new(6, 5),
                XY::new(8, 5),
                XY::new(8, 2),
                XY::new(2, 2),
                XY::new(2, 5),
                XY::new(4, 5),
                XY::new(4, 7),
                XY::new(1, 7)
            ]),
            4
        );
    }
}
