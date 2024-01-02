use std::{collections::HashSet};

use crate::{grid::Grid, AocError, DailyInput, RowCol};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let mut grid = Grid::new(&lines);

    let grid = double_empty_rows_and_cols(&mut grid);

    println!("Grid: {grid}");
    let binding = grid.find(HashSet::from([b'#']));
    let found = binding.get(&b'#').expect("Should have found some #s");
    println!("Found at {:?}", found);

    let sum: usize = (0..found.len())
        .flat_map(|a| {
            (a + 1..found.len()).map(move |b| {
                let c = shortest_path(found[a], found[b]);
                println!("{a} -> {b} = {c}");
                c
            })
        })
        .sum();

    Ok(sum.to_string())
}

fn shortest_path(start: RowCol, end: RowCol) -> usize {
    ((end.col() - start.col()).abs() + (end.row() - start.row()).abs()) as usize
}

fn double_empty_rows_and_cols(grid: &mut Grid) -> Grid {
    double_empty_rows(grid);
    let mut grid = grid.transpose();
    double_empty_rows(&mut grid);
    grid.transpose()
}

fn double_empty_rows(grid: &mut Grid) {
    let rows_to_dup = grid.rows().filter(|&row| grid.get_row(row).unwrap().all(|c| c == b'.')).collect::<Vec<_>>();
    println!("Rows to duplicate: {:?}", rows_to_dup);
    for (i, row) in rows_to_dup.iter().enumerate() {
        let row = row + i as i64;
        grid.insert_row_after(row);
        for col in grid.cols() {
            grid.set(RowCol(row, col + 1), b'.');
        }
    }
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
            "374"
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
            "9974721"
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
