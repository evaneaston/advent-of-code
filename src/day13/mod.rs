use std::collections::HashSet;

use log::debug;

use crate::{grid::Grid, AocError, DailyInput, RowCol};

fn are_cols_the_same(grid: &Grid, col1: i64, col2: i64) -> bool {
    (grid.min_row()..=grid.max_row())
        .map(|row| {
            (
                grid.get(RowCol::new(row, col1)).unwrap(),
                grid.get(RowCol::new(row, col2)).unwrap(),
            )
        })
        .all(|(c1, c2)| c1 == c2)
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub(crate) struct MirrorLine {
    before: i64,
    after: i64,
    num_before: i64,
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
pub(crate) enum MirrorLineMatch {
    Vertical(MirrorLine),
    Horizontal(MirrorLine),
    None,
}

pub(crate) fn find_mirror_line(grid: &Grid) -> Option<MirrorLine> {
    for col in grid.min_col()..grid.max_col() {
        let mut comp = (col, col + 1);

        loop {
            if !are_cols_the_same(grid, comp.0, comp.1) {
                break;
            }
            if comp.0 <= grid.min_col() || comp.1 >= grid.max_col() {
                return Some(MirrorLine {
                    before: col,
                    after: col + 1,
                    num_before: col - grid.min_col() + 1,
                });
            }
            comp = (comp.0 - 1, comp.1 + 1);
        }
    }
    None
}
pub(crate) fn find_mirror_lines(grid: &Grid) -> Vec<MirrorLineMatch> {
    [
        find_mirror_line(grid).map(MirrorLineMatch::Vertical),
        find_mirror_line(&grid.transpose()).map(MirrorLineMatch::Horizontal),
    ]
    .iter()
    .flatten()
    .copied()
    .collect::<Vec<_>>()
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let lines_to_left_and_above: i64 = lines
        .split(|line| line.is_empty())
        .map(Vec::from)
        .map(|lines_vec| Grid::new_offset(RowCol::new(1, 1), &lines_vec))
        .enumerate()
        .map(|(index, grid)| {
            debug!("Grid {index}");
            debug!(" {}", grid);

            let transposed = grid.transpose();

            let mut all: Vec<MirrorLineMatch> = vec![];
            if let Some(mirror_line) = find_mirror_line(&grid) {
                all.push(MirrorLineMatch::Vertical(mirror_line));
            }
            if let Some(mirror_line) = find_mirror_line(&transposed) {
                all.push(MirrorLineMatch::Horizontal(mirror_line));
            }
            all
        })
        .map(|mlms| {
            if mlms.is_empty() {
                panic!("No mirror lines found")
            }
            *mlms.first().unwrap()
        })
        .map(|mlm| match mlm {
            MirrorLineMatch::Vertical(mlm) => mlm.num_before,
            MirrorLineMatch::Horizontal(mlm) => mlm.num_before * 100,
            MirrorLineMatch::None => 0,
        })
        .sum();

    println!("{:?}", lines_to_left_and_above);

    Ok(lines_to_left_and_above.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let lines_to_left_and_above: i64 = lines
        .split(|line| line.is_empty())
        .map(Vec::from)
        .map(|lines_vec| Grid::new_offset(RowCol::new(1, 1), &lines_vec))
        .enumerate()
        .map(|(index, grid)| {
            println!("====================================");
            println!("Grid {index}");

            let mlms = find_mirror_lines(&grid);

            assert_eq!(mlms.len(), 1);

            let initial = *mlms.first().unwrap();
            println!("  Initial     {:?}", initial);

            let mut results = HashSet::<MirrorLineMatch>::new();

            let mut grid2 = grid.clone();
            for r in grid.min_row()..=grid.max_row() {
                for c in grid.min_col()..=grid.max_col() {
                    let rc = RowCol::new(r, c);
                    print!(" {}", rc);
                    let ch = grid.get(rc).unwrap();
                    let flipped_ch = if ch == b'.' { b'#' } else { b'.' };

                    grid2.set(rc, flipped_ch); // flip

                    let mlms2 = find_mirror_lines(&grid2);
                    // for m in mlms2 {
                    //     results.insert(m);
                    // }
                    if let Some(new_mlm) = mlms2.iter().find(|&mlm| *mlm != initial) {
                        results.insert(*new_mlm);
                    }

                    grid2.set(rc, ch); // restore
                }
            }

            println!("  Final       {:?}", results);
            // if results.len() != 1 {
            //     panic!("Should be a single different reflection line");
            // }
            let result = *results.iter().next().unwrap_or(&MirrorLineMatch::None);
            println!("  Final       {:?}", result);
            result
        })
        .map(|mlm| match mlm {
            MirrorLineMatch::Vertical(mlm) => mlm.num_before,
            MirrorLineMatch::Horizontal(mlm) => mlm.num_before * 100,
            MirrorLineMatch::None => 0,
        })
        .sum();

    println!("{:?}", lines_to_left_and_above);

    Ok(lines_to_left_and_above.to_string())
}

#[cfg(test)]
mod test {
    use crate::{
        day13::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 13,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "405"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 13,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "40006"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 13,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "400"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 13,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "" // "20686" // too low
               //    "42789" // too high
        );
    }
}
