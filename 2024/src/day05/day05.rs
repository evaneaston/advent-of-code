use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::{AocError, DailyInput};

struct Day05Input(HashMap<i64, HashSet<i64>>, Vec<Vec<i64>>);
impl Day05Input {
    fn afters(&self) -> &HashMap<i64, HashSet<i64>> {
        &self.0
    }
    fn page_lists(&self) -> &Vec<Vec<i64>> {
        &self.1
    }
}

fn parse(input: &DailyInput) -> Day05Input {
    let lines = input.get_input_lines().unwrap();

    let mut afters: HashMap<i64, HashSet<i64>> = HashMap::new();

    let rules_re = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    let pages_re = Regex::new(r"^(\d+)(,\d+)*$").unwrap();

    lines.iter().flat_map(|line| rules_re.captures(line)).for_each(|c| {
        let before = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let after = c.get(2).unwrap().as_str().parse::<i64>().unwrap();
        afters.entry(before).or_default().insert(after);
    });
    let pages = lines
        .iter()
        .filter(|line| pages_re.is_match(line))
        .map(|line| line.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Day05Input(afters, pages)
}

fn is_order_valid(input: &Day05Input, pages: &[i64]) -> bool {
    for (i, &page) in pages.iter().enumerate() {
        if let Some(afters) = input.afters().get(&page) {
            for p in &pages[0..i] {
                if afters.contains(p) {
                    return false;
                }
            }
        }
    }
    true
}

fn middle(pages: &[i64]) -> i64 {
    *pages.get((pages.len() - 1) / 2).unwrap()
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let input = parse(&input);
    let answer = input
        .page_lists()
        .iter()
        .filter(|&pages| is_order_valid(&input, pages))
        .map(|pages| middle(pages))
        .sum::<i64>();

    Ok(format!("{answer}"))
}

fn reorder(input: &Day05Input, pages: &[i64]) -> Vec<i64> {
    let mut copy = Vec::from_iter(pages.iter().copied());

    let mut counter = 0;
    let mut retry = true;
    'outer: while retry {
        counter += 1;
        if counter > 100 {
            panic!("This is not working");
        }
        retry = false;
        for i in 0..copy.len() {
            for j in i + 1..copy.len() {
                let left = copy[i];
                let right = copy[j];
                if let Some(afters) = input.afters().get(&right) {
                    if afters.contains(&left) {
                        copy.remove(j);
                        copy.insert(i, right);
                        retry = true;
                        continue 'outer;
                    }
                }
            }
        }
    }
    copy
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let input = parse(&input);
    let answer = input
        .page_lists()
        .iter()
        .filter(|pages| !is_order_valid(&input, pages))
        .map(|pages| reorder(&input, pages))
        .map(|pages| middle(&pages))
        .sum::<i64>();
    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{day05::middle, DailyInput, InputType};

    const DAY: usize = 5;

    #[test]
    fn test_middle() {
        assert_eq!(middle(&[1]), 1);
        assert_eq!(middle(&[1, 2, 3]), 2);
        assert_eq!(middle(&[1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "143"
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
            "4766"
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
            "123"
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
            "6257"
        );
    }
}
