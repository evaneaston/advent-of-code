use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{grid::Grid, AocError, DailyInput, RowCol};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let number_locations = get_number_locations(&lines);
    let grid = Grid::new(&lines);

    let mut sum: u32 = 0;
    for number_location in number_locations {
        if is_part_number(&grid, &number_location) {
            sum += number_location.2;
        }
    }

    Ok(sum.to_string())
}

fn surrounding_symbols(grid: &Grid, number_location: &(RowCol, RowCol, u32)) -> Vec<(RowCol, u8)> {
    let (start, end, _) = number_location;

    let mut surrounding: Vec<RowCol> = Vec::new();

    let start_col = if start.col() > grid.min_col() {
        surrounding.push(RowCol(start.row(), start.col() - 1));
        start.col() - 1
    } else {
        start.col()
    };

    let end_col = if end.col() < grid.max_col() {
        surrounding.push(RowCol(end.row(), end.col() + 1));
        end.col() + 1
    } else {
        end.col()
    };

    if start.row() > grid.min_row() {
        surrounding.extend((start_col..=end_col).map(|col| RowCol(start.row() - 1, col)));
    }
    if start.row() < grid.max_row() {
        surrounding.extend((start_col..=end_col).map(|col| RowCol(start.row() + 1, col)));
    }

    surrounding
        .iter()
        .flat_map(|rc| match grid.get(*rc) {
            Some(c) => {
                if c != b'.' && !c.is_ascii_digit() {
                    Some((*rc, c))
                } else {
                    None
                }
            }
            None => None,
        })
        .collect::<Vec<_>>()
}

fn is_part_number(grid: &Grid, number_location: &(RowCol, RowCol, u32)) -> bool {
    !surrounding_symbols(grid, number_location).is_empty()
}

fn get_number_locations(lines: &[String]) -> Vec<(RowCol, RowCol, u32)> {
    let num_locations = lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            //println!("{line}");
            RE.find_iter(line).map(move |m| {
                (
                    RowCol(row as i64, m.start() as i64),
                    RowCol(row as i64, (m.end() - 1) as i64),
                    m.as_str().parse::<u32>().unwrap(),
                )
            })
        })
        .collect::<Vec<_>>();
    num_locations
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let number_locations = get_number_locations(&lines);
    let grid = Grid::new(&lines);

    let mut symbol_numbers = HashMap::<(RowCol, u8), Vec<u32>>::new();
    for number_location in number_locations {
        for position_and_symbol in surrounding_symbols(&grid, &number_location) {
            symbol_numbers
                .entry(position_and_symbol)
                .and_modify(|e| e.push(number_location.2))
                .or_insert_with(|| vec![number_location.2]);
        }
    }

    let sum: u64 = symbol_numbers
        .iter()
        .filter_map(|((_, symbol), numbers)| {
            let star = b'*';
            println!("{symbol} {star} {:?}", numbers);
            if *symbol == b'*' && numbers.len() == 2 {
                Some(numbers.iter().map(|n| *n as u64).product::<u64>())
            } else {
                None
            }
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        day03::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(DailyInput {
                day: 3,
                part: None,
                input_type: InputType::Example
            })
            .unwrap(),
            "4361"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 3,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            "514969"
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(DailyInput {
                day: 3,
                part: None,
                input_type: InputType::Example
            })
            .unwrap(),
            "467835"
        );
    }
}
