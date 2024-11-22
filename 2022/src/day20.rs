use crate::common::{AocError, InputType};
use log::debug;
use std::collections::VecDeque;

pub fn part1() -> Result<String, AocError> {
    let numbers = parse_numbers(InputType::Challenge)?;

    let mixed = mix(&numbers, 1);
    let coord = get_coordinates(&mixed);
    let answer = coord.0 + coord.1 + coord.2;

    assert_eq!(answer, 4914);
    Ok(format!("{}", answer))
}

pub fn part2() -> Result<String, AocError> {
    let numbers = parse_numbers(InputType::Challenge)?;
    let numbers = numbers.iter().map(|n| *n * 811589153).collect();
    let mixed = mix(&numbers, 10);
    let coord = get_coordinates(&mixed);
    let answer = coord.0 + coord.1 + coord.2;

    assert_eq!(answer, 7973051839072);
    Ok(format!("{}", answer))
}

fn parse_numbers(input_type: InputType) -> Result<Vec<i64>, AocError> {
    Ok(input_type
        .get_input_lines(20)?
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>())
}

fn mix(numbers: &Vec<i64>, times: usize) -> Vec<i64> {
    let mut indexed = numbers.iter().enumerate().collect::<VecDeque<(_, _)>>();
    let num_numbers = indexed.len();
    for _ in 0..times {
        for i in 0..num_numbers {
            debug!("Before: {:?}", indexed);

            let removal_index = locate(i, &indexed);
            debug!("  Removal Index: {:?}", removal_index);
            let val = indexed.remove(removal_index).unwrap();
            debug!("  Removed: {:?}", val.1);
            debug!("  After removal: {:?}", indexed);
            let mut insertion_index =
                ((removal_index as i64) + *val.1).rem_euclid((num_numbers as i64) - 1) as usize;
            if insertion_index == 0 {
                insertion_index = indexed.len();
            }
            debug!("  Insertion Index: {:?}", insertion_index);
            indexed.insert(insertion_index, val);
            debug!("  After Insertion: {:?}", indexed);
        }
    }
    indexed.iter().map(|v| *v.1).collect::<Vec<i64>>()
}

fn locate(index: usize, indexed: &VecDeque<(usize, &i64)>) -> usize {
    for j in 0..indexed.len() {
        if indexed[j].0 == index {
            return j;
        }
    }
    panic!("Couldn't find value originally indexed as {}", index);
}

fn get_coordinates(mixed: &Vec<i64>) -> (i64, i64, i64) {
    let zero_index = mixed.iter().position(|v| *v == 0).unwrap();
    (
        mixed[(zero_index + 1000).rem_euclid(mixed.len())],
        mixed[(zero_index + 2000).rem_euclid(mixed.len())],
        mixed[(zero_index + 3000).rem_euclid(mixed.len())],
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        common::InputType,
        day20::{get_coordinates, mix, parse_numbers},
    };

    #[test]
    fn test_part1_example() {
        let numbers = parse_numbers(InputType::Example).unwrap();

        let mixed = mix(&numbers, 1);

        assert_eq!(mixed, vec![1, 2, -3, 4, 0, 3, -2]);

        let coordinate = get_coordinates(&mixed);
        assert_eq!(coordinate.0 + coordinate.1 + coordinate.2, 3);
    }
}
