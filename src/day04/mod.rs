use std::collections::BTreeSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{AocError, DailyInput};

lazy_static! {
    static ref RE: Regex = Regex::new(r"Card\s+(\d+):\s([^|]*)\|(.*)").unwrap();
}

#[allow(dead_code)]
#[derive(Debug)]
struct Card {
    pub number: u16,
    pub winning_numbers: BTreeSet<u16>,
    pub my_numbers: BTreeSet<u16>,
    pub my_winning_numbers: BTreeSet<u16>,
}

fn lines_to_cards(lines: &[String]) -> Vec<Card> {
    lines
        .iter()
        .map(|line| {
            let c = RE.captures(line).unwrap();
            let card_number = c.get(1).unwrap().as_str().parse::<u16>().unwrap();
            let winning_numbers = split_into_number_set(c.get(2).unwrap().as_str());
            let my_numbers = split_into_number_set(c.get(3).unwrap().as_str());
            let my_winning_numbers = my_numbers
                .intersection(&winning_numbers)
                .cloned()
                .collect::<BTreeSet<_>>();
            Card {
                number: card_number,
                winning_numbers,
                my_numbers,
                my_winning_numbers,
            }
        })
        .collect()
}

fn split_into_number_set(s: &str) -> BTreeSet<u16> {
    s.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u16>().unwrap())
        .collect::<BTreeSet<_>>()
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let cards = lines_to_cards(&lines);

    let sum: u32 = cards
        .iter()
        .map(|c| {
            if !c.my_winning_numbers.is_empty() {
                2_u32.pow((c.my_winning_numbers.len() as u32) - 1)
            } else {
                0
            }
        })
        .sum();

    Ok(sum.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let cards = lines_to_cards(&lines);
    let mut card_counts = vec![1_u32; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let next_n = card.my_winning_numbers.len();       
        if next_n > 0 {
            for n in (i+1)..(i + next_n+1).clamp(i + 1, cards.len()) {
                card_counts[n] += card_counts[i];
            }
        }
    }

    let sum: u32 = card_counts.iter().sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        day04::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
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
            "25174"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 4,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            "6420979"
        );
    }   

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 4,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            "6420979"
        );
    }   
}
