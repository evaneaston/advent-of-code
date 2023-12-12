use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{AocError, DailyInput};

lazy_static! {
    static ref NUMERIC_MAPPINGS: HashMap<&'static str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .copied()
    .collect();
    static ref RE: Regex =
        Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    static ref RRE: Regex =
        Regex::new(r".*([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
}

fn get_first_digit(s: &str) -> u32 {
    let forward_cap = RE.captures(s).unwrap();
    let s = forward_cap.get(1).unwrap().as_str();
    match s.parse::<u32>() {
        Ok(n) => n,
        Err(_) => *NUMERIC_MAPPINGS.get(s).unwrap(),
    }
}

fn get_last_digit(s: &str) -> u32 {
    let cap = RRE.captures(s).unwrap();
    let s = cap.get(1).unwrap().as_str();
    str_to_u32(s)
}

fn str_to_u32(s: &str) -> u32 {
    match s.parse::<u32>() {
        Ok(n) => n,
        Err(_) => *NUMERIC_MAPPINGS.get(s).unwrap(),
    }
}
fn get_first_and_last_digits_part1(s: &str) -> (u32, u32) {
    (
        s.chars().flat_map(|c| c.to_digit(10)).next().unwrap(),
        s.chars().rev().flat_map(|c| c.to_digit(10)).next().unwrap(),
    )
}


pub fn part1(input: DailyInput) -> Result<String, AocError> {
    Ok(input
        .get_input_lines()?
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| get_first_and_last_digits_part1(line))
        .map(|(first, last)| first * 10 + last)
        .sum::<u32>().to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    Ok(input
        .get_input_lines()?
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| (get_first_digit(line), get_last_digit(line)))
        .map(|(first, last)| first * 10 + last)
        .sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        day01::{get_first_digit, part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(DailyInput {
                day: 1,
                input_type: InputType::Example,
                number: Some(1),
            })
            .unwrap(),
            "142"
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(DailyInput {
                day: 1,
                input_type: InputType::Example,
                number: Some(2),
            })
            .unwrap(),
            "281"
        );
    }

    #[test]
    fn test_oneight() {
        assert_eq!(get_first_digit("123"), 1);
        assert_eq!(get_first_digit("aone23"), 1);
    }
}
