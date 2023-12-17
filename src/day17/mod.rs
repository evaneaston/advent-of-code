use crate::{day15::load, grid::Grid, AocError, DailyInput};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = load(input)?;
    Ok("".to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let grid = load(input)?;
    Ok("".to_string())
}

pub(crate) fn load_grid(input: DailyInput) -> Result<Grid, AocError> {
    let lines = input.get_input_lines()?;
    Ok(Grid::new(&lines))
}

#[cfg(test)]
mod test {
    use crate::{
        day17::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 17,
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
                day: 17,
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
                day: 17,
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
                day: 17,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            ""
        );
    }
}
