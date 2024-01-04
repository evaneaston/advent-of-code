use std::{collections::HashSet, fmt::Display};

use log::debug;

use crate::{get_num_interior_points, grid::Grid, AocError, DailyInput, RowCol, XY};

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
        day10::{part1, part2},
        DailyInput, InputType,
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
}
