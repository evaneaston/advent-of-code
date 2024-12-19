use crate::{as_i64, AocError, DailyInput};

use std::collections::HashMap;

use regex::Regex;

fn parse(input: DailyInput) -> Result<(Vec<i64>, Vec<i64>), AocError> {
    let re = Regex::new(r"^(\d+)\s+(\d+)").unwrap();
    Ok(input
        .get_input_lines()?
        .iter()
        .map(|l| {
            let c = re.captures(l).unwrap();
            (as_i64(&c, 1), as_i64(&c, 2))
        })
        .map(|r| (r.0, r.1))
        .unzip())
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let (mut left, mut right) = parse(input)?;

    left.sort();
    right.sort();

    let sum: i64 = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum();

    Ok(sum.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let (left, right) = parse(input)?;

    let right_counts = right.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    let sum: i64 = left
        .iter()
        .map(|l| match right_counts.get(l) {
            Some(c) => l * c,
            None => 0,
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 1;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "11"
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
            "2000468"
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
            "31"
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
            "18567089"
        );
    }
}
