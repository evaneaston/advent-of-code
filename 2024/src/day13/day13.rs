use crate::{algo::line_intersection, coord::XY, AocError, DailyInput};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"Button.*:\s+X\+(?P<x>\d+),\s+Y\+(?P<y>\d+)"#)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"Prize.*:\s+X=(?P<x>\d+),\s+Y=(?P<y>\d+)"#)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Machine {
    button_a: XY,
    button_b: XY,
    prize: XY,
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
            button_a: XY(button_a.x, button_a.y),
            button_b: XY(button_b.x, button_b.y),
            prize: XY(prize.x, prize.y),
        })
    }
    let mut machines = Vec::<Machine>::new();
    while let Some(m) = next_machine(&mut i) {
        machines.push(m);
    }
    Ok(machines)
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let machines = parse(&input)?;

    let mut tokens = 0;
    for m in machines {
        let solution = line_intersection(
            (m.button_a.x() as i32).into(),
            (m.button_b.x() as i32).into(),
            ((-m.prize.x()) as i32).into(),
            (m.button_a.y() as i32).into(),
            (m.button_b.y() as i32).into(),
            ((-m.prize.y()) as i32).into(),
        );
        // eprintln!("{m:?} {solution:?}");
        if let Some(solution) = solution {
            tokens += solution.x() * 3 + solution.y();
        }
    }

    Ok(tokens.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let machines = parse(&input)?;
    let machines = machines.iter().map(|m| Machine {
        button_a: m.button_a,
        button_b: m.button_b,
        prize: XY(m.prize.x() + 10000000000000, m.prize.y() + 10000000000000),
    }).collect::<Vec<_>>();

    let mut tokens = 0;
    for m in machines {
        let solution = line_intersection(
            m.button_a.x() as f64,
            m.button_b.x() as f64,
            (-m.prize.x()) as f64,
            m.button_a.y() as f64,
            m.button_b.y() as f64,
            (-m.prize.y()) as f64,
        );
        // eprintln!("{m:?} {solution:?}");
        if let Some(solution) = solution {
            tokens += solution.x() * 3 + solution.y();
        }
    }

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
