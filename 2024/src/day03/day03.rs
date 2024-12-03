use crate::{AocError, DailyInput};
use regex::Regex;

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let input = input.get_input_as_single_string()?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut answer = 0_i64;
    for mat in re.captures_iter(&input) {
        let a = mat[1].parse::<i64>().unwrap();
        let b = mat[2].parse::<i64>().unwrap();
        answer += a * b;
    }

    Ok(format!("{answer}"))
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let input = input.get_input_as_single_string()?;

    let mut active = true;

    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))").unwrap();
    let mut answer = 0_i64;
    for mat in re.captures_iter(&input) {
        if &mat[0] == "do()" {
            active = true;
        } else if &mat[0] == "don't()" {
            active = false;
        } else if active {
            let a = mat[2].parse::<i64>().unwrap();
            let b = mat[3].parse::<i64>().unwrap();
            answer += a * b;
        }
    }

    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 3;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(1),
            })
            .unwrap(),
            "161"
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
            "174336360"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(2),
            })
            .unwrap(),
            "48"
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
            "88802350"
        );
    }
}
