use std::collections::BTreeSet;

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

fn get_regions(grid: &Grid) -> Vec<(u8, BTreeSet<RowCol>)> {
    let mut regions = Vec::<(u8, BTreeSet<RowCol>)>::new();

    let mut remaining = BTreeSet::from_iter(grid.all_cell_locations_by_row_by_col());

    loop {
        match remaining.pop_first() {
            None => break,
            Some(starting_point) => {
                let region_value = grid.get(starting_point).unwrap();

                let mut region = BTreeSet::<RowCol>::new();
                let mut visited = BTreeSet::<RowCol>::new();

                region.insert(starting_point);
                remaining.remove(&starting_point);

                fn fill(
                    grid: &Grid,
                    region: &mut BTreeSet<RowCol>,
                    visited: &mut BTreeSet<RowCol>,
                    remaining: &mut BTreeSet<RowCol>,
                    loc: RowCol,
                    looking_for: u8,
                ) {
                    [Direction::N, Direction::E, Direction::S, Direction::W].iter().for_each(|direction| {
                        let neighbor_loc = loc.plus(direction);
                        if visited.insert(neighbor_loc) {
                            if let Some(neighbor) = grid.get(neighbor_loc) {
                                if neighbor == looking_for {
                                    remaining.remove(&neighbor_loc);
                                    region.insert(neighbor_loc);
                                    fill(grid, region, visited, remaining, neighbor_loc, looking_for);
                                }
                            }
                        }
                    });
                }
                fill(
                    grid,
                    &mut region,
                    &mut visited,
                    &mut remaining,
                    starting_point,
                    region_value,
                );

                regions.push((region_value, region));
            }
        }
    }
    regions
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);
    let mut answer = 0_u64;
    for r in get_regions(&grid) {
        // continue;
        let region_value = r.0;
        let area = r.1.len();

        let edges: usize =
            r.1.iter()
                .map(|location| {
                    [Direction::N, Direction::E, Direction::S, Direction::W]
                        .iter()
                        .map(|direction| grid.get(location.plus(direction)))
                        .map(|v| match v {
                            None => 1,
                            Some(neighbor_value) => {
                                if neighbor_value != region_value {
                                    1
                                } else {
                                    0
                                }
                            }
                        })
                        .sum::<usize>()
                })
                .sum();

        answer += (area * edges) as u64;
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

    for (_, region_locations) in get_regions(&grid).iter() {
        let area = region_locations.len();

        let (west_sides, east_sides) = grid.cols().fold((0, 0), |(w, e), col| {
            (
                w + count_sides(region_locations, grid.col_cell_locations_top_down(col), Direction::W),
                e + count_sides(region_locations, grid.col_cell_locations_top_down(col), Direction::E),
            )
        });

        let (north_sides, south_sides) = grid.rows().fold((0, 0), |(n, s), row| {
            (
                n + count_sides(region_locations, grid.row_cell_locations_left_right(row), Direction::N),
                s + count_sides(region_locations, grid.row_cell_locations_left_right(row), Direction::S),
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
