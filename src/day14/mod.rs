use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hasher,
};

use log::debug;

use crate::{day10::Direction, grid::Grid, AocError, DailyInput, RowCol};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let mut grid = Grid::new(&input.get_input_lines()?);

    tip(&mut grid, Direction::N);

    let sum = load(&grid);

    Ok(sum.to_string())
}

fn load(grid: &Grid) -> usize {
    let mut sum: usize = 0;
    for row in grid.rows() {
        let count_o_s = grid.get_row(row).unwrap().filter(|v| *v == b'O').count();
        let multiplier = grid.row_count() - (row - grid.min_row()) as usize;
        sum += multiplier * count_o_s;
    }
    sum
}

fn tip(grid: &mut Grid, direction: Direction) {
    let found = grid.find(HashSet::from([b'O']));
    let o_s = found.get(&b'O').expect("Expected there to be some O's");
    let mut o_s = o_s.clone();
    match direction {
        Direction::E => o_s.sort_by(|a, b| match b.col().cmp(&a.col()) {
            Ordering::Equal => a.row().cmp(&b.row()),
            cmp => cmp,
        }),
        Direction::S => o_s.sort_by(|a, b| match b.row().cmp(&a.row()) {
            Ordering::Equal => a.col().cmp(&b.col()),
            cmp => cmp,
        }),
        Direction::W => o_s.sort_by(|a, b| match a.col().cmp(&b.col()) {
            Ordering::Equal => a.row().cmp(&b.row()),
            cmp => cmp,
        }),
        Direction::N => o_s.sort_by(|a, b| match a.row().cmp(&b.row()) {
            Ordering::Equal => a.col().cmp(&b.col()),
            cmp => cmp,
        }),
        _ => todo!(),
    }
    for rc in o_s {
        let mut o_s_rc: RowCol = rc;
        'inner: loop {
            let next_rc = match direction {
                Direction::E => o_s_rc.plus_col(),
                Direction::S => o_s_rc.plus_row(),
                Direction::W => o_s_rc.minus_col(),
                Direction::N => o_s_rc.minus_row(),
                _ => todo!(),
            };
            if let Some(next) = grid.get(next_rc) {
                if next == b'.' {
                    grid.set(next_rc, b'O');
                    grid.set(o_s_rc, b'.');
                    o_s_rc = next_rc;
                    continue 'inner;
                }
            }
            break 'inner;
        }
    }
}

fn hash(grid: &Grid) -> u64 {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    hasher.finish()
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    input.get_input_as_single_string()?;

    let mut grid = Grid::new(&input.get_input_lines()?);

    let mut grid_hashes = HashMap::<u64, Vec<usize>>::new();

    grid_hashes.entry(hash(&grid)).or_default().push(0);

    let mut grids = vec![grid.clone()];

    debug!("Grid 0:\n{}", grids[0]);

    let (cycle_start, cycle_end) = 'block: {
        for index in 1..=1000000000 {
            tip(&mut grid, Direction::N);
            tip(&mut grid, Direction::W);
            tip(&mut grid, Direction::S);
            tip(&mut grid, Direction::E);

            grids.push(grid.clone());

            let hash_indexes = grid_hashes.entry(hash(&grid)).or_default();
            hash_indexes.push(index);

            if hash_indexes.len() > 1 {
                debug!("found cycle between {:?}", hash_indexes);
                break 'block (hash_indexes[0], hash_indexes[1]);
            }
        }
        todo!();
    };

    let remaining = 1000000000 - cycle_end;
    let cycle_size = cycle_end - cycle_start;
    let div = remaining.div_euclid(cycle_size);
    let mult = cycle_size*(div+1);
    let rem = remaining.rem_euclid(mult);
    debug!("remaining={remaining}");
    debug!("div={div}");
    debug!("mult={mult}");
    debug!("rem={rem}");
    debug!("Cycle {:?}", (cycle_start, cycle_end));
    for (i, g) in grids.iter().enumerate() {
        debug!("Grid {} load: {}", i, load(g));
    }
    let sum = load(&grids[cycle_start+ remaining.rem_euclid(cycle_size)]);
    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use crate::{
        day14::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_rem_euclid() {
        assert_eq!(12_usize.rem_euclid(10), 2);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 14,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "136"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 14,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "110565"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 14,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "64"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 14,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "89845"
        );
    }
}
