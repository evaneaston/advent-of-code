use std::collections::HashMap;

use crate::{AocError, DailyInput};

fn solve(stones: Vec<i64>, times: usize) -> usize {
    let mut counter = HashMap::new();
    for stone in &stones {
        counter.insert(*stone, 1);
    }

    let mut blink = || {
        let mut new_counter = HashMap::with_capacity(counter.capacity());

        for (&stone, count) in counter.iter() {
            if stone == 0 {
                *new_counter.entry(1).or_insert(0) += count;
            } else {
                let ndigits = stone.checked_ilog10().unwrap_or(0) + 1;
                if ndigits % 2 == 0 {
                    let s = stone.to_string();
                    let split_at = s.len() / 2;
                    let a = s[0..split_at].parse::<i64>().unwrap();
                    let b = s[split_at..].parse::<i64>().unwrap();

                    *new_counter.entry(a).or_insert(0) += count;
                    *new_counter.entry(b).or_insert(0) += count;
                } else {
                    let n = stone * 2024;
                    *new_counter.entry(n).or_insert(0) += count;
                }
            }
        }

        counter = new_counter;
    };

    (0..times).for_each(|_| blink());
    let answer: usize = counter.values().sum();
    answer
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let stones = (input.get_input_as_single_string()?)
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let times = match input.input_type {
        crate::InputType::Example => 6,
        crate::InputType::Challenge => 25,
    };
    Ok(format!("{}", solve(stones, times)))
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let stones = (input.get_input_as_single_string()?)
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let times = match input.input_type {
        crate::InputType::Example => 6,
        crate::InputType::Challenge => 75,
    };

    Ok(format!("{}", solve(stones, times)))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 11;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "22"
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
            "188902"
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
            "22"
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
            "223894720281135"
        );
    }
}
