use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};
use strum::IntoEnumIterator;

fn is_xmas_match(grid: &Grid, rc: RowCol, direction: Direction) -> bool {
    grid.get(rc) == Some(b'X')
        && grid.get(rc.plus_n(&direction, 1)) == Some(b'M')
        && grid.get(rc.plus_n(&direction, 2)) == Some(b'A')
        && grid.get(rc.plus_n(&direction, 3)) == Some(b'S')
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);

    let mut count: i64 = 0;
    for r in grid.rows() {
        for c in grid.cols() {
            for d in Direction::iter() {
                if is_xmas_match(&grid, RowCol::new(r, c), d) {
                    count += 1;
                }
            }
        }
    }
    Ok(format!("{count}"))
}

fn is_x_mas_match(grid: &Grid, rc: RowCol) -> bool {
    if grid.get(rc) == Some(b'A') {
        let is_mas = |d1: Direction, d2: Direction| {
            grid.get(rc.plus(&d1)) == Some(b'M') && grid.get(rc.plus(&d2)) == Some(b'S')
        };

        (is_mas(Direction::NW, Direction::SE) || is_mas(Direction::SE, Direction::NW))
            && (is_mas(Direction::NE, Direction::SW) || is_mas(Direction::SW, Direction::NE))
    } else {
        false
    }
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
