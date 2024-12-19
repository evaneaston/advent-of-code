use std::collections::BTreeSet;

use crate::{AocError, DailyInput};
use std::collections::HashMap;

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let input = parse(input)?;
    let available = input.available_patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let mut already_computed = HashMap::new();
    let max_pattern_len = input.available_patterns.iter().map(|v| v.len()).max().unwrap();
    Ok(input
        .desired_designs
        .iter()
        .filter(|p| count_valid_pattern(p.as_str(), &available, &mut already_computed, max_pattern_len) > 0)
        .count()
        .to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let input = parse(input)?;
    let available = input.available_patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let mut already_computed = HashMap::new();
    let max_pattern_len = input.available_patterns.iter().map(|v| v.len()).max().unwrap();
    let a = input
        .desired_designs
        .iter()
        .map(|p| count_valid_pattern(p.as_str(), &available, &mut already_computed, max_pattern_len));
    let answer: usize = a.sum();
    Ok(answer.to_string())
}

fn count_valid_pattern<'a>(
    pattern: &'a str,
    valid: &Vec<&'a str>,
    already_computed: &mut HashMap<&'a str, usize>,
    max_len: usize,
) -> usize {
    let mut combinations = 0;
    if already_computed.contains_key(pattern) {
        return *already_computed.get(pattern).unwrap();
    }
    if pattern.is_empty() {
        return 1;
    }

    for i in 1..=max_len.min(pattern.len()) {
        if valid.contains(&&pattern[..i]) {
            let subcount = count_valid_pattern(&pattern[i..], valid, already_computed, max_len);
            combinations += subcount;
        }
    }
    already_computed.insert(pattern, combinations);
    combinations
}

struct Day19Input {
    available_patterns: BTreeSet<String>,
    desired_designs: Vec<String>,
}
fn parse(input: DailyInput) -> Result<Day19Input, AocError> {
    let lines = input.get_input_lines()?;

    Ok(Day19Input {
        available_patterns: lines[0].split(",").map(|p| p.trim().to_string()).collect(),
        desired_designs: lines.iter().skip(2).map(|d| d.trim().to_string()).collect(),
    })
}

// fn is_possible(available: &BTreeSet<String>, design: &str) -> bool {
//     fn matches_any(index: usize, available: &BTreeSet<String>, design: &str) -> bool {
//         // let c = design.chars().next().unwrap();
//         // for a in available.iter().filter(|&s| s.chars().next().unwrap() == c) {
//         for a in available.iter() {
//             if design[index..].starts_with(a) {
//                 if (index + a.len()) == (design.len() - 1) {
//                     return true;
//                 } else if matches_any(index + a.len(), available, design) {
//                     return true;
//                 }
//             }
//         }
//         false
//     }
//     matches_any(0, available, design)
// }
// pub fn part1(input: DailyInput) -> Result<String, AocError> {
//     let input = parse(input)?;

//     let answer = input.desired_designs.iter().enumerate().filter(|(i, d)| {
//         eprint!("{i}) {d}: ");
//         let is = is_possible(&input.available_patterns, &d);
//         eprintln!("{is}");
//         is
//     }).count();
//     Ok(answer.to_string())
// }

// pub fn part2(_input: DailyInput) -> Result<String, AocError> {
//     Ok("".to_string())
// }

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 19;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "6"
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
            "311"
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
            "16"
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
            "616234236468263"
        );
    }
}
