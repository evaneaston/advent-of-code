use crate::{AocError, DailyInput};

fn parse_inputs(input: &DailyInput) -> Result<Vec<Vec<i64>>, AocError> {
    Ok(input
        .get_input_lines()?
        .iter()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

fn preprocess(report: &[i64]) -> Vec<(i64, i64)> {
    report
        .windows(2)
        .map(|w| {
            let diff = w[1] - w[0];
            (diff.signum(), diff.abs())
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let inputs = parse_inputs(&input)?;

    let answer = inputs
        .iter()
        .filter(|&report| is_safe(report))
        .count();

    Ok(format!("{answer}"))
}

fn is_safe(report: &Vec<i64>) -> bool {
    let preprocessed = preprocess(report);
    preprocessed.iter().map(|(sign, _)| *sign).sum::<i64>().unsigned_abs() as usize == (report.len() - 1)
        && preprocessed.iter().all(|(_, abs_diff)| *abs_diff >= 1 && *abs_diff <= 3)
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let inputs = parse_inputs(&input)?;

    let answer = inputs
        .iter()
        .filter(|&report| {
            for index_to_remove in 0..report.len() {
                let alt_report = report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if i == index_to_remove { None } else { Some(x) })
                    .collect::<Vec<_>>();

                if is_safe(&alt_report) {
                    return true;
                }
            }
            false
        })
        .count();

    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 2;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "2"
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
            "230"
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
            "4"
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
            "301"
        );
    }
}
