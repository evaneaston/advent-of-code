use std::collections::BTreeMap;

use itertools::Itertools;
use regex::Regex;

use crate::{
    as_i64,
    coord::{Offset, RowCol, XY},
    grid::Grid,
    AocError, DailyInput,
};

// fn part1(robots: &Vec<Robot>, arena_dimensions: Coordinate<i32>, seconds: i32) -> usize {
//     let mut quadrant_counts = [0;4];
//     for robot in robots {
//         let final_pos = Coordinate{
//                         x:  (robot.position.x + (robot.velocity.x * seconds)).rem_euclid(arena_dimensions.x),
//                         y: (robot.position.y + (robot.velocity.y * seconds)).rem_euclid(arena_dimensions.y),
//                     };
//         if final_pos.x < (arena_dimensions.x -1) /2  && final_pos.y < (arena_dimensions.y -1) /2 {
//             quadrant_counts[0] +=1
//         } else if final_pos.x > (arena_dimensions.x -1) /2  && final_pos.y < (arena_dimensions.y -1) /2 {
//             quadrant_counts[1] +=1
//         } else if final_pos.x < (arena_dimensions.x -1) /2  && final_pos.y > (arena_dimensions.y -1) /2 {
//             quadrant_counts[2] +=1
//         } else if final_pos.x > (arena_dimensions.x -1) /2  && final_pos.y > (arena_dimensions.y -1) /2 {
//             quadrant_counts[3] +=1
//         }
//     }
//     quadrant_counts.iter().product()
// }

#[derive(Debug)]
struct Robot {
    location: RowCol,
    velocity: RowCol,
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let v = lines[0].split(',').take(2).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let grid = Grid::new_repeating(RowCol(0, 0), RowCol(v[0] - 1, v[1] - 1), b'.').transpose();
    eprintln!("{} {}", grid.min(), grid.max());
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = lines
        .iter()
        .skip(1)
        .map(|l| {
            let c = re.captures(l).unwrap();
            let x = as_i64(&c, 1);
            let y = as_i64(&c, 2);
            let dx = as_i64(&c, 3);
            let dy = as_i64(&c, 4);
            Robot {
                location: XY(x, y).into(),
                velocity: XY(dx, dy).into(),
            }
        })
        .collect::<Vec<_>>();
    for robot in &robots {
        eprintln!(
            " p={},{} v={},{} (loc rc={}, vel rc={})",
            XY::from(robot.location).x(),
            XY::from(robot.location).y(),
            XY::from(robot.velocity).x(),
            XY::from(robot.velocity).y(),
            robot.location,
            robot.velocity,
        );
    }

    eprintln!("Initial");
    print_counts_as_xy(&grid, &robots);

    (0..100).for_each(|i| {
        //eprintln!("Step {}", i + 1);
        for robot in &mut robots {
            let next_row = (robot.location.row() + robot.velocity.row()).rem_euclid(grid.max_row() + 1);
            let next_col = (robot.location.col() + robot.velocity.col()).rem_euclid(grid.max_col() + 1);
            robot.location = RowCol(next_row, next_col);
        }
        if i == 0 {
            eprintln!("After one step:");
            print_counts_as_xy(&grid, &robots);
        }
    });

    eprintln!("Final");
    print_counts_as_xy(&grid, &robots);

    let middle_row = (grid.max_row() + grid.min_row()) / 2;
    let middle_col = (grid.max_col() + grid.min_col()) / 2;
    eprintln!("{middle_row} {middle_col}");

    let mut qc = [0, 0, 0, 0];
    robots.iter().for_each(|r| {
        let RowCol(r, c) = r.location;
        if r < middle_row  && c < middle_col {
            qc[0] += 1;
        } else if r < middle_row && c > middle_col {
            qc[1] += 1;
        } else if r > middle_row && c < middle_col {
            qc[2] += 1;
        } else if r > middle_row && c > middle_col {
            qc[3] += 1;
        }
    });
    eprintln!("{qc:?}");

    let answer: usize = qc.iter().product();
    Ok(answer.to_string())
}

fn print_counts_as_xy(grid: &Grid, robots: &[Robot]) {
    let binding = robots.iter().into_group_map_by(|r| r.location);
    let counts = binding.iter().map(|(k, v)| (k, v.len())).collect::<BTreeMap<_, _>>();
    // eprintln!("{counts:?}");
    let mut gc = grid.clone();
    for (&rc, c) in counts {
        gc.set(rc, c.to_string().chars().next().unwrap() as u8);
    }
    // let gc = gc.transpose();
    eprintln!("{gc}");
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let v = lines[0].split(',').take(2).map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let grid = Grid::new_repeating(RowCol(0, 0), RowCol(v[0] - 1, v[1] - 1), b'.').transpose();
    eprintln!("{} {}", grid.min(), grid.max());
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = lines
        .iter()
        .skip(1)
        .map(|l| {
            let c = re.captures(l).unwrap();
            let x = as_i64(&c, 1);
            let y = as_i64(&c, 2);
            let dx = as_i64(&c, 3);
            let dy = as_i64(&c, 4);
            Robot {
                location: XY(x, y).into(),
                velocity: XY(dx, dy).into(),
            }
        })
        .collect::<Vec<_>>();
    for robot in &robots {
        eprintln!(
            " p={},{} v={},{} (loc rc={}, vel rc={})",
            XY::from(robot.location).x(),
            XY::from(robot.location).y(),
            XY::from(robot.velocity).x(),
            XY::from(robot.velocity).y(),
            robot.location,
            robot.velocity,
        );
    }

    eprintln!("Initial");
    print_counts_as_xy(&grid, &robots);

    (0..20000).for_each(|i| {
        //eprintln!("Step {}", i + 1);
        for robot in &mut robots {
            let next_row = (robot.location.row() + robot.velocity.row()).rem_euclid(grid.max_row() + 1);
            let next_col = (robot.location.col() + robot.velocity.col()).rem_euclid(grid.max_col() + 1);
            robot.location = RowCol(next_row, next_col);
        }
        // if i == 0 {
            eprintln!("####### {i} #######");
            print_counts_as_xy(&grid, &robots);
        // }
    });


    let answer: usize = 0;
    Ok(answer.to_string())
}


#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 14;

    #[test]
    fn test_wrap() {
        assert_eq!((98_i64 + 4).rem_euclid(101), 1);
        assert_eq!((0_i64 - 2).rem_euclid(101), 99);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "12"
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
            "230436441"
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
            ""
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
            ""
        );
    }
}
