use crate::{AocError, DailyInput};

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(\d+)\s+(\d+)").unwrap();
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.get_input_lines()?.iter().map( |l| RE.captures(l).unwrap()).map(|c| {
        (c.get(1).unwrap().as_str().parse::<i64>().unwrap(), c.get(2).unwrap().as_str().parse::<i64>().unwrap())
    }).unzip();

    left.sort();
    right.sort();
    
    let sum: i64 = left.iter().zip(right.iter()).map(|(l,r)| {
         (l-r).abs()
     }).sum();

    Ok(format!("{sum}"))
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let ( left, right): (Vec<_>, Vec<_>) = input.get_input_lines()?.iter().map( |l| RE.captures(l).unwrap()).map(|c| {
        (c.get(1).unwrap().as_str().parse::<i64>().unwrap(), c.get(2).unwrap().as_str().parse::<i64>().unwrap())
    }).unzip();

    let mut right_counts = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let sum: i64 = left.iter().map(|l| {
        match right_counts.get(l) {
            Some(c) => l*c,
            None => 0,
        }
    }).sum();

    Ok(format!("{sum}"))
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
