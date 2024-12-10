use std::collections::HashSet;

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

fn score(g: &Grid, start: RowCol) -> usize {
    let mut nines = HashSet::<RowCol>::new();
    climb(g, b'0', &start, &mut nines);
    nines.len()
}
fn climb(g: &Grid, last_val: u8, coord: &RowCol, nines: &mut HashSet<RowCol>) {
    for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
        let next = coord.plus(&dir);
        if let Some(v) = g.get(next) {
            if v == last_val + 1 {
                if v == b'9' {
                    nines.insert(next);
                } else {
                    climb(g, v, &next, nines);
                }
            }
        }
    }
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let index = g.find(HashSet::from_iter([b'0']));
    let trailheads = index.get(&b'0').unwrap();

    let answer = trailheads.iter().map(|&start| score(&g, start)).sum::<usize>();
    Ok(format!("{answer}"))
}

fn rating(g: &Grid, start: RowCol) -> usize {
    let mut count = 0_usize;
    climb_for_rating(g, b'0', &start, &mut count);
    count
}
fn climb_for_rating(g: &Grid, last_val: u8, coord: &RowCol, count: &mut usize) {
    for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
        let next = coord.plus(&dir);
        if let Some(v) = g.get(next) {
            if v == last_val + 1 {
                if v == b'9' {
                    *count += 1;
                } else {
                    climb_for_rating(g, v, &next, count);
                }
            }
        }
    }
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let index = g.find(HashSet::from_iter([b'0']));
    let trailheads = index.get(&b'0').unwrap();

    let answer = trailheads.iter().map(|&start| rating(&g, start)).sum::<usize>();
    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 10;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "36"
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
            "512"
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
            "81"
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
            "1045"
        );
    }
}
