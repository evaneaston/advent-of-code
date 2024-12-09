use std::collections::HashSet;

use crate::{coord::RowCol, grid::Grid, AocError, DailyInput};
use itertools::Itertools;

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let mut s = HashSet::<u8>::new();
    (b'a'..=b'z').for_each(|b| {
        s.insert(b);
    });
    (b'A'..=b'Z').for_each(|b| {
        s.insert(b);
    });
    (b'0'..=b'9').for_each(|b| {
        s.insert(b);
    });

    let found = g.find(s);

    let mut anti_nodes = HashSet::<RowCol>::new();
    for v in found.values() {
        let a = v.iter().combinations(2);
        for p in a {
            let a = p[0];
            let b = p[1];
            let o = b.diff(a);
            let bpo = b.plus_offset(&o);
            let amo = a.minus_offset(&o);

            if g.get(bpo).is_some() {
                anti_nodes.insert(bpo);
            }
            if g.get(amo).is_some() {
                anti_nodes.insert(amo);
            }
        }
    }
    let answer = anti_nodes.len();

    Ok(format!("{answer}"))
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let mut s = HashSet::<u8>::new();
    (b'a'..=b'z').for_each(|b| {
        s.insert(b);
    });
    (b'A'..=b'Z').for_each(|b| {
        s.insert(b);
    });
    (b'0'..=b'9').for_each(|b| {
        s.insert(b);
    });

    let found = g.find(s);

    let mut anti_nodes = HashSet::<RowCol>::new();
    for v in found.values() {
        let a = v.iter().combinations(2);
        for p in a {
            let mut a = *p[0];
            let mut b = *p[1];
            anti_nodes.insert(a);
            anti_nodes.insert(b);
            let o = b.diff(&a);
            loop {
                b = b.plus_offset(&o);
                match g.get(b) {
                    Some(_) => {
                        anti_nodes.insert(b);
                    }
                    None => break,
                }
            }
            loop {
                a = a.minus_offset(&o);
                match g.get(a) {
                    Some(_) => {
                        anti_nodes.insert(a);
                    }
                    None => break,
                }
            }
        }
    }

    let answer = anti_nodes.len();

    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 8;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "14"
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
            "379"
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
            "34"
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
            "1339"
        );
    }
}
