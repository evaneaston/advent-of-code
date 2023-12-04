use std::collections::{BTreeSet, HashMap};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{grid::Grid, AocError, DailyInput, RowCol};

lazy_static! {
    static ref RE: Regex = Regex::new(r"Card\s+(\d+):\s([^|]*)\|(.*)").unwrap();
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;

    let mut sum: u32 = 0;

    for line in lines {
        let c = RE.captures(&line).unwrap();
        let card = c.get(1).unwrap().as_str();
        let winning_numbers = c
            .get(2)
            .unwrap()
            .as_str()
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u16>().unwrap())
            .collect::<BTreeSet<_>>();
        let my_numbers = c
            .get(3)
            .unwrap()
            .as_str()
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u16>().unwrap())
            .collect::<BTreeSet<_>>();

        let num_winning = my_numbers.intersection(&winning_numbers).count();
        if num_winning > 0 {
            sum += 2_u32.pow((num_winning as u32) - 1);
        }
    }
    Ok(sum.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        day04::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(DailyInput {
                day: 4,
                part: None,
                input_type: InputType::Example
            })
            .unwrap(),
            "13"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 4,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            ""
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(DailyInput {
                day: 4,
                part: None,
                input_type: InputType::Example
            })
            .unwrap(),
            ""
        );
    }
}
