use crate::{AocError, DailyInput};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
input.get_input_lines()?;
    Ok("".to_string())
}

pub fn part2(_input: DailyInput) -> Result<String, AocError> {
    Ok("".to_string())
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 15;

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(1),
            })
            .unwrap(),
            "10092"
        );
    }
    #[test]
    fn test_part1_example2() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(2),
            })
            .unwrap(),
            "2028"
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
            ""
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
            ""
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
            ""
        );
    }
}
