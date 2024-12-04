use crate::{coord::RowCol, grid::Grid, AocError, DailyInput};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(EnumIter, Debug, Display, Clone)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Direction {
    fn next(&self, rc: &RowCol) -> RowCol {
        match self {
            Direction::N => rc.minus_row(),
            Direction::NE => rc.minus_row().plus_col(),
            Direction::E => rc.plus_col(),
            Direction::SE => rc.plus_row().plus_col(),
            Direction::S => rc.plus_row(),
            Direction::SW => rc.plus_row().minus_col(),
            Direction::W => rc.minus_col(),
            Direction::NW => rc.minus_row().minus_col(),
        }
    }
}
impl RowCol {
    fn step(&self, direction: &Direction) -> RowCol {
        direction.next(self)
    }
}

fn is_xmas_match(grid: &Grid, rc: RowCol, direction: &Direction) -> bool {
    if grid.get(rc) == Some(b'X') {
        let rc = rc.step(&direction);
        if grid.get(rc) == Some(b'M') {
            let rc = rc.step(&direction);
            if grid.get(rc) == Some(b'A') {
                let rc = rc.step(&direction);
                if grid.get(rc) == Some(b'S') {
                    return true;
                }
            }
        }
    }
    false
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);

    let mut count: i64 = 0;
    for r in grid.rows() {
        for c in grid.cols() {
            for d in Direction::iter() {
                if is_xmas_match(&grid, RowCol::new(r, c), &d) {
                    count += 1;
                }
            }
        }
    }
    Ok(format!("{count}"))
}

impl Grid {
    fn matches(&self, rc: &RowCol, dir: &Direction, v: u8) -> bool {
        self.get(rc.step(dir)) == Some(v)
    }
}

fn is_x_mas_match(grid: &Grid, rc: RowCol) -> bool {
    if grid.get(rc) == Some(b'A') {
        let down_right = (grid.matches(&rc, &Direction::NW, b'M') && grid.matches(&rc, &Direction::SE, b'S'))
            || (grid.matches(&rc, &Direction::NW, b'S') && grid.matches(&rc, &Direction::SE, b'M'));

        let up_right = (grid.matches(&rc, &Direction::SW, b'M') && grid.matches(&rc, &Direction::NE, b'S'))
            || (grid.matches(&rc, &Direction::SW, b'S') && grid.matches(&rc, &Direction::NE, b'M'));

        if down_right && up_right {
            return true;
        }
    }
    false
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);

    let mut count: i64 = 0;
    for r in grid.rows() {
        for c in grid.cols() {
            if is_x_mas_match(&grid, RowCol::new(r, c)) {
                count += 1;
            }
        }
    }
    Ok(format!("{count}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 4;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "18"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "2397"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "9"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "1824"
        );
    }
}
