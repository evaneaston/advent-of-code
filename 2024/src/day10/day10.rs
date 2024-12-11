use std::collections::HashSet;

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let (g, trailheads) = grid_and_trailheads(input)?;

    let answer = trailheads
        .iter()
        .map(|&start| {
            let mut reachable_summit_locations = HashSet::<RowCol>::new();
            climb_to_all_summits(&g, b'0', &start, &mut |coord: &RowCol| {
                reachable_summit_locations.insert(*coord);
            });
            reachable_summit_locations.len()
        })
        .sum::<usize>();

        Ok(answer.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let (g, trailheads) = grid_and_trailheads(input)?;

    let answer = trailheads
        .iter()
        .map(|&start| {
            let mut rating = 0_usize;
            climb_to_all_summits(&g, b'0', &start, &mut |_: &RowCol| {
                rating += 1;
            });
            rating
        })
        .sum::<usize>();

    Ok(answer.to_string())
}

fn climb_to_all_summits(g: &Grid, last_val: u8, coord: &RowCol, report_summit: &mut impl FnMut(&RowCol)) {
    for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
        let next = coord.plus(&dir);
        if let Some(v) = g.get(next) {
            if v == last_val + 1 {
                if v == b'9' {
                    report_summit(&next);
                } else {
                    climb_to_all_summits(g, v, &next, report_summit);
                }
            }
        }
    }
}

fn grid_and_trailheads(input: DailyInput) -> Result<(Grid, Vec<RowCol>), AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let mut index = g.index(HashSet::from_iter([b'0']));
    let trailheads = index.remove(&b'0').unwrap();
    Ok((g, trailheads))
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
