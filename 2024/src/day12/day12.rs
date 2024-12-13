use std::collections::{BTreeMap, BTreeSet};

use crate::{
    coord::{rc, Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

fn get_regions(grid: &Grid) -> Vec<BTreeSet<RowCol>> {
    let mut regions = Vec::<BTreeSet<RowCol>>::new();
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
            let mut region = BTreeSet::<RowCol>::new();
            let mut visited = BTreeSet::<RowCol>::new();

            fn fill(
                grid: &Grid,
                region: &mut BTreeSet<RowCol>,
                visited: &mut BTreeSet<RowCol>,
                loc: RowCol,
                looking_for: u8,
            ) {
                if visited.contains(&loc) {
                    return;
                }
                region.insert(loc);
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
        answer += (area * plot_fence_count) as u64;
    }

    Ok(answer.to_string())
}

fn count_sides(region: &BTreeSet<RowCol>, rcs: impl Iterator<Item = RowCol>, direction: Direction) -> usize {
    let mut count = 0;
    let mut is_boundary = false;
    rcs.for_each(|rc| {
        let new_is_boundary = region.contains(&rc) && !region.contains(&rc.plus(&direction));
        if new_is_boundary && !is_boundary {
            count += 1;
        }
        is_boundary = new_is_boundary;
    });
    count
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);
    let mut answer = 0_u64;

    for region in get_regions(&grid).iter() {
        let area = region.len();

        let (west_sides, east_sides) = grid.cols().fold((0, 0), |(w, e), col| {
            (
                w + count_sides(region, grid.col_cell_locations_top_down(col), Direction::W),
                e + count_sides(region, grid.col_cell_locations_top_down(col), Direction::E),
            )
        });

        let (north_sides, south_sides) = grid.rows().fold((0, 0), |(n, s), row| {
            (
                n + count_sides(region, grid.row_cell_locations_left_right(row), Direction::N),
                s + count_sides(region, grid.row_cell_locations_left_right(row), Direction::S),
            )
        });

        let sides = west_sides + east_sides + north_sides + south_sides;

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
