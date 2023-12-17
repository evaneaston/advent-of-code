use crate::{AocError,DailyInput};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    input.get_input_as_single_string()?;
    Ok("".to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    input.get_input_as_single_string()?;
    Ok("".to_string())
}

#[cfg(test)]
mod test {
    use crate::{
        day11::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 11,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "102"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 11,
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
                day: 11,
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
                day: 11,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            ""
        );
    }
}
