use std::collections::{BTreeMap, BTreeSet};

use crate::{
    coord::{rc, Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

fn get_regions(grid: &Grid) -> Vec<Vec<RowCol>> {
    let mut regions = Vec::<Vec<RowCol>>::new();
    let mut location_to_region_index = BTreeMap::<RowCol, usize>::new();

    loop {
        let mut starting_point: Option<(RowCol, u8)> = None;
        'outer: for r in grid.rows() {
            for c in grid.cols() {
                let loc = rc(r, c);
                let c = grid.get(loc).unwrap();
                if !location_to_region_index.contains_key(&loc) {
                    starting_point.replace((loc, c));
                    break 'outer;
                }
            }
        }

        if let Some((start_loc, region_value)) = starting_point {
            let mut region = Vec::<RowCol>::new();
            let mut visited = BTreeSet::<RowCol>::new();

            fn fill(
                grid: &Grid,
                region: &mut Vec<RowCol>,
                visited: &mut BTreeSet<RowCol>,
                loc: RowCol,
                looking_for: u8,
            ) {
                if visited.contains(&loc) {
                    return;
                }
                region.push(loc);
                visited.insert(loc);

                [Direction::N, Direction::E, Direction::S, Direction::W].iter().for_each(|direction| {
                    let neighbor_loc = loc.plus(direction);
                    if let Some(neighbor) = grid.get(neighbor_loc) {
                        if neighbor == looking_for {
                            fill(grid, region, visited, neighbor_loc, looking_for);
                        }
                    }
                });
            }
            fill(grid, &mut region, &mut visited, start_loc, region_value);

            let region_index = regions.len();
            region.iter().for_each(|loc| {
                location_to_region_index.insert(*loc, region_index);
            });

            regions.push(region);
        } else {
            break;
        }
    }
    regions
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);
    let mut answer = 0_u64;
    for r in get_regions(&grid) {
        let region_value = grid.get(*r.first().unwrap()).unwrap();
        let area = r.len();

        let mut plot_fence_count = 0;
        for loc in r.iter() {
            for d in [Direction::N, Direction::E, Direction::S, Direction::W] {
                match grid.get(loc.plus(&d)) {
                    Some(neighbor) => {
                        if neighbor != region_value {
                            plot_fence_count += 1
                        }
                    }
                    None => plot_fence_count += 1,
                }
            }
        }
        //eprintln!("Region {r:?}");
        //eprintln!("  area={area} perimeter={plot_fence_count}");
        answer += (area * plot_fence_count) as u64;
    }

    Ok(answer.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);
    let mut answer = 0_u64;

    for (i, region) in get_regions(&grid).iter().enumerate() {
        // let region_value = grid.get(*region.first().unwrap()).unwrap();

        //eprintln!("Getting sides for region #{i} {}", region_value as char);
        let area = region.len();

        let min_row = region.iter().map(|rc| rc.row()).min().unwrap();
        let max_row = region.iter().map(|rc| rc.row()).max().unwrap();
        let min_col = region.iter().map(|rc| rc.col()).min().unwrap();
        let max_col = region.iter().map(|rc| rc.col()).max().unwrap();

        let mut left_sides = 0;
        let mut right_sides = 0;
        for col in min_col..=max_col {
            {
                let mut is_boundary = false;
                for row in min_row..=max_row {
                    let l = rc(row, col);
                    // let v = grid.get(l).unwrap();
                    let new_is_boundary = region.contains(&l) && !region.contains(&l.plus(&Direction::W));
                    // //eprintln!("   At {} left is boundary = {new_is_boundary}", rc(row, col));
                    if new_is_boundary && !is_boundary {
                        left_sides += 1;
                    }
                    is_boundary = new_is_boundary;
                }
            }

            {
                let mut is_boundary = false;
                for row in min_row..=max_row {
                    let l = rc(row, col);
                    // let v = grid.get(l).unwrap();
                    let new_is_boundary = region.contains(&l) && !region.contains(&l.plus(&Direction::E));
                    // //eprintln!("   At {} right is boundary = {new_is_boundary}", rc(row, col));
                    if new_is_boundary && !is_boundary {
                        right_sides += 1;
                    }
                    is_boundary = new_is_boundary;
                }
            }
        }

        let mut top_sides = 0;
        let mut bottom_sides = 0;
        for row in min_row..=max_row {
            {
                let mut is_boundary = false;
                for col in min_col..=max_col {
                    let l = rc(row, col);
                    // let v = grid.get(l).unwrap();
                    let new_is_boundary = region.contains(&l) && !region.contains(&l.plus(&Direction::N));
                    // //eprintln!("     At {} top is boundary = {new_is_boundary}", rc(row, col));
                    if new_is_boundary && !is_boundary {
                        top_sides += 1;
                    }
                    is_boundary = new_is_boundary;
                }
            }

            {
                let mut is_boundary = false;
                for col in min_col..=max_col {
                    let l = rc(row, col);
                    // let v = grid.get(l).unwrap();
                    let new_is_boundary = region.contains(&l) && !region.contains(&l.plus(&Direction::S));
                    // //eprintln!("   At {} right is boundary = {new_is_boundary}", rc(row, col));
                    if new_is_boundary && !is_boundary {
                        bottom_sides += 1;
                    }
                    is_boundary = new_is_boundary;
                }
            }
        }
        //eprintln!("   left sides={left_sides}");
        //eprintln!("   rights sides={right_sides}");
        //eprintln!("   top sides={top_sides}");
        //eprintln!("   bottom sides={bottom_sides}");

        let sides = left_sides + top_sides + right_sides + bottom_sides;
        //eprintln!("   area={area} sides={sides}");

        answer += (area * sides) as u64;
    }

    Ok(answer.to_string())
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 12;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "1930"
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
            "1374934"
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
            "1206"
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
            "841078"
        );
    }
}
