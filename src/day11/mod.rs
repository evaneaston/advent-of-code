use std::collections::HashSet;

use crate::{grid::Grid, AocError, DailyInput, RowCol};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    calc(input, 1)
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    calc(input, 1000000-1)
}

fn shortest_path(
    start: RowCol,
    end: RowCol,
    rows_to_expand: &[i64],
    cols_to_expand: &[i64],
    expand_by: usize,
) -> usize {
    let base_distance = ((end.col() - start.col()).abs() + (end.row() - start.row()).abs()) as usize;

    let row_range = start.row().min(end.row())..start.row().max(end.row());
    let crossed_rows = rows_to_expand.iter().filter(|&r| row_range.contains(r)).collect::<Vec<_>>();
    let crossed_rows_to_expand = crossed_rows.len();

    let col_range = start.col().min(end.col())..start.col().max(end.col());
    let crossed_cols = cols_to_expand.iter().filter(|&c| col_range.contains(c)).collect::<Vec<_>>();
    let crossed_cols_to_expand = crossed_cols.len();


    // println!("    {} -> {}", start, end);
    // println!("       rte crossed = {crossed_rows_to_expand} ({:?})", crossed_rows);
    // println!("       cte crossed = {crossed_cols_to_expand} ({:?})", crossed_cols);
    base_distance + (crossed_cols_to_expand + crossed_rows_to_expand) * expand_by
}

pub fn calc(input: DailyInput, expand_by: usize) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let grid = Grid::new(&lines);

//    println!("Grid: {grid}");

    let rows_to_expand = grid.rows().filter(|&row| grid.get_row(row).unwrap().all(|v| v == b'.')).collect::<Vec<_>>();
    // println!("rows to expand {:?}", rows_to_expand);

    let cols_to_expand = grid.cols().filter(|&col| grid.get_col(col).unwrap().all(|v| v == b'.')).collect::<Vec<_>>();
    // println!("cols to expand {:?}", cols_to_expand);

    let galaxies = grid.find(HashSet::from([b'#']));
    let found = galaxies.get(&b'#').expect("Should have found some #s");
    // println!("Galaxies found at {:?}", found);
    let sum: usize = (0..found.len())
        .flat_map(|a| (a + 1..found.len()).map(move |b| (a, b)))
        .map(|(a, b)| shortest_path(found[a], found[b], &rows_to_expand, &cols_to_expand, expand_by))
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use crate::{
        day11::{calc, part1, part2},
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
        let input = DailyInput {
            day: 11,
            input_type: InputType::Example,
            number: None,
        };
        assert_eq!(calc(input.clone(), 10-1).unwrap(), "1030");
        assert_eq!(calc(input, 100-1).unwrap(), "8410");
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
            "702770569197"
        );
    }
}
