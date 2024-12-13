use crate::{algo::line_intersection, AocError, DailyInput};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Recap, Clone, Copy)]
#[recap(regex = r#"Button.*:\s+X\+(?P<x>\d+),\s+Y\+(?P<y>\d+)"#)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug, Deserialize, PartialEq, Recap, Clone, Copy)]
#[recap(regex = r#"Prize.*:\s+X=(?P<x>\d+),\s+Y=(?P<y>\d+)"#)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}
impl Machine {
    fn solution(&self) -> Option<(i64, i64)> {
        line_intersection(
            self.button_a.x as f64,
            self.button_b.x as f64,
            (-self.prize.x) as f64,
            self.button_a.y as f64,
            self.button_b.y as f64,
            (-self.prize.y) as f64,
        )
    }
}
fn parse(input: &DailyInput) -> Result<Vec<Machine>, AocError> {
    let lines = input.get_input_lines()?;
    let mut i = lines.iter();

    fn next_machine(i: &mut core::slice::Iter<String>) -> Option<Machine> {
        let mut l = i.next()?;
        if l.is_empty() {
            l = i.next()?;
        }
        let button_a: Button = l.parse::<Button>().unwrap();
        let button_b: Button = i.next()?.parse::<Button>().unwrap();
        let prize: Prize = i.next()?.parse::<Prize>().unwrap();
        Some(Machine {
            button_a,
            button_b,
            prize,
        })
    }
    let mut machines = Vec::<Machine>::new();
    while let Some(m) = next_machine(&mut i) {
        machines.push(m);
    }
    Ok(machines)
}

fn count_tokens_for_all_machines_with_solutions(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|m| m.solution().filter(|(a, b)| *a >= 0 && *b >= 0)) // only soltions with zero or more button presses
        .map(|(a, b)| (a * 3 + b) as usize)
        .sum::<usize>()
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let machines = parse(&input)?;
    let tokens = count_tokens_for_all_machines_with_solutions(&machines);
    Ok(tokens.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let mut machines = parse(&input)?;
    machines.iter_mut().for_each(|m| {
        m.prize.x += 10000000000000;
        m.prize.y += 10000000000000;
    });

    let tokens = count_tokens_for_all_machines_with_solutions(&machines);

    Ok(tokens.to_string())
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 13;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "480"
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
            "35255"
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
            "875318608908"
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
            "87582154060429"
        );
    }
}
