use itertools::{repeat_n, Itertools};
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{AocError, DailyInput};

trait Apply {
    fn apply(&self, op1: &i64, op2: &i64) -> i64;
}

#[derive(Debug, EnumIter, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Apply for Operator {
    fn apply(&self, op1: &i64, op2: &i64) -> i64 {
        match self {
            Self::Add => *op1 + *op2,
            Self::Multiply => *op1 * *op2,
        }
    }
}

fn parse(input: &DailyInput) -> Result<Vec<(i64, Vec<i64>)>, AocError> {
    let re = Regex::new(r"^(\d+): (.*)$").unwrap();
    Ok(input
        .get_input_lines()?
        .iter()
        .map(|line| {
            let t = re
                .captures(line)
                .expect("line matches pattern")
                .iter()
                .skip(1)
                .map(|sm| sm.unwrap().as_str())
                .collect::<Vec<_>>();
            let answer = t[0].parse::<i64>().unwrap();
            let operands = t[1].trim().split(' ').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
            (answer, operands)
        })
        .collect::<Vec<_>>())
}

fn search_permutations<O: IntoEnumIterator + Apply>(
    expected_value: &i64,
    parent_result: &i64,
    values: &[i64],
    current_index: usize,
    last_index: usize,
) -> Option<i64> {
    let o2 = values[current_index];
    O::iter().find_map(|op| {
        let result = op.apply(parent_result, &o2);
        if current_index == last_index {
            if result == *expected_value {
                Some(result)
            } else {
                None
            }
        } else {
            search_permutations::<O>(expected_value, &result, values, current_index + 1, last_index)
        }
    })
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let inputs = parse(&input)?;
    let answer = inputs
        .iter()
        .filter_map(|(expected_answer, operands)| {
            search_permutations::<Operator>(expected_answer, &operands[0], operands, 1, operands.len() - 1)
        })
        .sum::<i64>();
    Ok(format!("{answer}"))
}

#[derive(Debug, EnumIter, Clone)]
enum Operator2 {
    Add,
    Multiply,
    Concatenate,
}
impl Apply for Operator2 {
    fn apply(&self, op1: &i64, op2: &i64) -> i64 {
        match self {
            Self::Add => *op1 + *op2,
            Self::Multiply => *op1 * *op2,
            Self::Concatenate => format!("{op1}{}", op2).parse::<i64>().unwrap(),
        }
    }
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let inputs = parse(&input)?;
    let answer = inputs
        .iter()
        .filter_map(|(expected_answer, operands)| {
            search_permutations::<Operator2>(expected_answer, &operands[0], operands, 1, operands.len() - 1)
        })
        .sum::<i64>();
    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 7;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "3749"
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
            "7579994664753"
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
            "11387"
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
            "438027111276610"
        );
    }
}
