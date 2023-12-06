use crate::{
    DailyInput, InputType, day05::{part1::part1, part2::part2},
};

#[test]
fn test_part1_example() {
    assert_eq!(
        part1(DailyInput {
            day: 5,
            part: None,
            input_type: InputType::Example
        })
        .unwrap(),
        "35"
    );
}

#[test]
fn test_part1_challenge() {
    assert_eq!(
        part1(DailyInput {
            day: 5,
            part: None,
            input_type: InputType::Challenge
        })
        .unwrap(),
        "251346198"
    );
}

#[test]
fn test_part2_example() {
    assert_eq!(
        part2(DailyInput {
            day: 5,
            part: None,
            input_type: InputType::Example
        })
        .unwrap(),
        "46"
    );
}

#[test]
fn test_part2_challenge() {
    assert_eq!(
        part2(DailyInput {
            day: 5,
            part: None,
            input_type: InputType::Challenge
        })
        .unwrap(),
        "72263011"
    );
}
