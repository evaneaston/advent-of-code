use lazy_static::lazy_static;
use regex::Regex;

use crate::{grid::Grid, AocError, DailyInput, RowCol};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
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
    let grid = Grid::new(&lines);

    let mut sum: u32 = 0;
    for (start, end, value) in num_locations {
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
        //println!(" {start} {end} {value}: {:?}", surrounding);

        if surrounding
            .iter()
            .flat_map(|rc| grid.get(*rc))
            .any(|c| c != b'.' && !c.is_ascii_digit())
        {
            sum += value;
        }
    }

    Ok(sum.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    input.get_input_as_single_string()?;
    Ok("".to_string())
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
}
