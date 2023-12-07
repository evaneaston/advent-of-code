
use crate::{
    day06::{part1, part2, comp_dist},
    DailyInput, InputType,
};


#[test]
fn test_comp_dist() {
    assert_eq!(comp_dist(7, 0),  0);
    assert_eq!(comp_dist(7, 1), 6);
    assert_eq!(comp_dist(7, 2), 10);
    assert_eq!(comp_dist(7, 3), 12);
    assert_eq!(comp_dist(7, 4), 12);
    assert_eq!(comp_dist(7, 5), 10);
    assert_eq!(comp_dist(7, 6), 6);
    assert_eq!(comp_dist(7, 7), 0);
}

#[test]
fn test_part1_example() {
    assert_eq!(
        part1(DailyInput {
            day: 6,
            part: None,
            input_type: InputType::Example
        })
        .unwrap(),
        "288"
    );
}

#[test]
fn test_part1_challenge() {
    assert_eq!(
        part1(DailyInput {
            day: 6,
            part: None,
            input_type: InputType::Challenge
        })
        .unwrap(),
        "1083852"
    );
}

#[test]
fn test_part2_example() {
    assert_eq!(
        part2(DailyInput {
            day: 6,
            part: None,
            input_type: InputType::Example
        })
        .unwrap(),
        "71503"
    );
}

#[test]
fn test_part2_challenge() {
    assert_eq!(
        part2(DailyInput {
            day: 6,
            part: None,
            input_type: InputType::Challenge
        })
        .unwrap(),
        "23501589"
    );
}
