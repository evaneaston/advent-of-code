use crate::{grid::Grid, AocError, DailyInput, RowCol};
use log::debug;
use nom::AsChar;

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub(crate) struct MirrorLine {
    before: i64,
    after: i64,
    num_before: i64,
}

#[derive(PartialEq, Debug, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub(crate) enum MirrorLineMatch {
    Vertical(MirrorLine),
    Horizontal(MirrorLine),
}

pub(crate) fn find_h_mirror_line(grid: &Grid, num_differences_allowed: usize) -> Option<MirrorLine> {
    for row in grid.min_row()..grid.max_row() {
        let num_to_compare = (row - grid.min_row() + 1).min(grid.max_row() - row);
        // println!("   num to compare = {num_to_compare}");
        let before_range = row - num_to_compare + 1..=row;
        let after_range = row + 1..=row + num_to_compare;

        // println!("Comparing ranges {:?} and {:?}", before_range, after_range);
        let above = before_range
            .rev()
            .flat_map(|r| grid.get_row(r).unwrap().rev())
            .collect::<Vec<_>>();
        let below = after_range
            .flat_map(|r| grid.get_row(r).unwrap().rev())
            .collect::<Vec<_>>();
        // println!(" CMP {num_to_compare} around {row}:{}", row+1);
        // println!("      {:?} ", above.iter().map(|d| d.as_char()).collect::<String>());
        // println!("      {:?} ", below.iter().map(|d| d.as_char()).collect::<String>());
        if above.iter().zip(below).filter(|&(&a, b)| a != b).count() == num_differences_allowed {
            return Some(MirrorLine {
                before: row,
                after: row + 1,
                num_before: row - grid.min_row() + 1,
            });
        }
    }
    None
}

pub(crate) fn find_all_mirror_lines_btree(grid: &Grid, num_differences_allowed: usize) -> Option<MirrorLineMatch> {
    find_h_mirror_line(&grid, num_differences_allowed)
        .map(MirrorLineMatch::Horizontal)
        .or_else(|| find_h_mirror_line(&grid.transpose(), num_differences_allowed).map(MirrorLineMatch::Vertical))
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let lines_to_left_and_above: i64 = lines
        .split(|line| line.is_empty())
        .map(Vec::from)
        .map(|lines_vec| Grid::new_offset(RowCol::new(1, 1), &lines_vec))
        .enumerate()
        .map(|(index, grid)| {
            println!("Grid {index}");
            println!(" {}", grid);
            let ml = find_all_mirror_lines_btree(&grid, 0);
            println!("  m: {:?}", ml);
            ml
        })
        .map(|mlms| {
            let miror_line = mlms.expect("No mirror lines found");
            println!("   {:?}", miror_line);
            miror_line
        })
        .map(|mlm| match mlm {
            MirrorLineMatch::Vertical(mlm) => mlm.num_before,
            MirrorLineMatch::Horizontal(mlm) => mlm.num_before * 100,
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
            println!("Grid {index}");
            println!(" {}", grid);
            let ml = find_all_mirror_lines_btree(&grid, 1);
            println!("  m: {:?}", ml);
            ml
        })
        .map(|mlms| {
            let miror_line = mlms.expect("No mirror lines found");
            println!("   {:?}", miror_line);
            miror_line
        })
        .map(|mlm| match mlm {
            MirrorLineMatch::Vertical(mlm) => mlm.num_before,
            MirrorLineMatch::Horizontal(mlm) => mlm.num_before * 100,
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
            "28627"
        );
    }
}
