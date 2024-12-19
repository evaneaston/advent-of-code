use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use regex::Regex;

use crate::{
    as_i64,
    coord::{RowCol, NSEW, XY},
    grid::Grid,
    AocError, DailyInput,
};

fn get_input(input: DailyInput) -> Result<(Grid, usize, Vec<XY>), AocError> {
    let re = Regex::new(r"^(\d+),(\d+)").unwrap();

    let lines = input.get_input_lines()?;
    let num_positions = lines[0].parse::<usize>().unwrap();
    let byte_positions = lines
        .iter()
        .skip(1)
        .map(|l| {
            let c = re.captures(l).unwrap();
            XY(as_i64(&c, 1), as_i64(&c, 2))
        })
        .collect::<Vec<_>>();

    let min_x = byte_positions.iter().map(|xy| xy.x()).min().unwrap();
    let max_x = byte_positions.iter().map(|xy| xy.x()).max().unwrap();
    let min_y = byte_positions.iter().map(|xy| xy.y()).min().unwrap();
    let max_y = byte_positions.iter().map(|xy| xy.y()).max().unwrap();

    Ok((
        Grid::new_repeating(XY(min_x, min_y).into(), XY(max_x, max_y).into(), b'.'),
        num_positions,
        byte_positions,
    ))
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let (mut grid, num_positions, byte_positions) = get_input(input)?;

    // eprintln!("{grid}");

    byte_positions.iter().take(num_positions).for_each(|location| {
        grid.set((*location).into(), b'#');
    });

    let start = grid.min();
    let end = grid.max();

    let answer = shortest_path_num_steps(&grid, start, end).unwrap();

    Ok(answer.to_string())
}

fn shortest_path_num_steps(grid: &Grid, start: RowCol, end: RowCol) -> Option<usize> {
    let mut min_steps = BTreeMap::<RowCol, usize>::new();
    grid.all_cell_locations_by_row_by_col().for_each(|location| {
        min_steps.insert(location, usize::MAX);
    });

    let mut heap: BinaryHeap<Reverse<(usize, RowCol)>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    while let Some(Reverse((steps, location))) = heap.pop() {
        NSEW.iter().for_each(|direction| {
            let next = location.plus(direction);
            if let Some(b'.') = grid.get(next) {
                if let Some(existing) = min_steps.get(&next) {
                    if steps + 1 < *existing {
                        min_steps.insert(next, steps + 1);
                        if next != end {
                            heap.push(Reverse((steps + 1, next)));
                        } else {
                            // eprintln!("Got to end with {} steps", steps + 1);
                        }
                    }
                }
            }
        });
    }

    match *min_steps.get(&end).unwrap() {
        usize::MAX => None,
        steps => Some(steps),
    }
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let (grid, _num_positions, byte_positions) = get_input(input)?;

    // byte_positions.iter().enumerate().for_each(|(i, xy)| {
    //     eprintln!("{i}: {xy}");
    // });
    let start = grid.min();
    let end = grid.max();

    let mut cache = BTreeMap::<usize, Option<usize>>::new();
    let mut solve = |n: usize| -> Option<usize> {
        *cache.entry(n).or_insert_with(|| {
            // eprintln!("   solving for index #{n}");
            let mut grid = grid.clone();
            byte_positions.iter().take(n).for_each(|location| {
                grid.set((*location).into(), b'#');
            });
            shortest_path_num_steps(&grid, start, end)
        })
    };

    let mut lower = 0;
    let mut upper = byte_positions.len();
    loop {
        let n = (upper + lower)/2;
        if n ==lower {
            panic!("No solutions {} = lower {}", n, lower);
        }
        if n == upper  {
            panic!("No solutions {} = lower {}", n, upper);
        }
        
        // eprint!("{lower} <= {n} <= {upper}: ");
        let solution: Option<usize> = solve(n);
        // eprintln!("{solution:?}");

        if solution.is_none() {
            if n == 0 {
                panic!("No solutions at all");
            }
            if solve(n - 1).is_some() {
                let blocking_byte_position = byte_positions[n - 1];
                return Ok(format!("{},{}", blocking_byte_position.x(), blocking_byte_position.y()));
            }
            upper = n;
        } else {
            lower = n;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 18;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "22"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "408"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "6,1"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "45,16"
        );
    }
}
