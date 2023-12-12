use std::{collections::HashSet, fmt::Display};

use log::debug;

use crate::{grid::Grid, AocError, DailyInput, RowCol};

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

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    E,
    S,
    W,
    N,
    None,
}

pub struct PipeIterator<'a> {
    grid: &'a Grid,
    current: RowCol,
    direction: Direction,
}
impl<'a> Iterator for PipeIterator<'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        let next_row_col = match self.direction {
            Direction::E => self.current.plus_col(),
            Direction::S => self.current.plus_row(),
            Direction::W => self.current.minus_col(),
            Direction::N => self.current.minus_row(),
            Direction::None => return None,
        };

        let next_symbol = match self.grid.get(next_row_col) {
            Some(rc) => Symbol::from(rc),
            None => return None, // off of board
        };

        let (can_move, next_direction) = match next_symbol {
            Symbol::Ground => (false, Direction::None),
            Symbol::Start => (true, Direction::None),

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
        if can_move {
            self.current = next_row_col;
            Some(next_symbol)
        } else {
            None
        }
    }
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let grid = Grid::new(&lines);

    debug!("{}", grid);

    let mut to_find = HashSet::<u8>::new();
    to_find.insert(b'S');
    let found = grid.find(to_find);
    debug!("Start @ {:?}", found);

    let start_row_col = found.get(&b'S').unwrap().iter().next().unwrap();

    let loop_lengths = [Direction::E, Direction::W, Direction::S, Direction::N]
        .iter()
        .map(|dir| {
            let mut iter = PipeIterator {
                grid: &grid,
                current: *start_row_col,
                direction: dir.clone(),
            };

            iter.position(|s| s == Symbol::Start).map(|pos| pos + 1)
        })
        .collect::<Vec<_>>();

    debug!("Lengths: {:?}", loop_lengths);

    let max_loop_len = *loop_lengths.iter().flatten().max().unwrap() as f32;
    let halfway = (max_loop_len / 2_f32).ceil() as i64;

    Ok(halfway.to_string())
}
pub fn part2(_input: DailyInput) -> Result<String, AocError> {
    Ok("".to_string())
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
            ""
        );
    }
}
