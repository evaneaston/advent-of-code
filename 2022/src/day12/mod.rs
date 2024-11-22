use crate::{
    common::{AocError, InputType, RowCol},
    day12::search::{Search, SearchState},
    grid::Grid,
};
use log::{debug, info};
use std::collections::HashSet;

mod search;

pub fn get_candidates(
    grid: &Grid,
    from: RowCol,
    candidate_test: fn(u8, u8) -> bool,
) -> Vec<RowCol> {
    let mut result = Vec::with_capacity(4);
    let from_e = get_elevation(grid, from.clone());
    if from_e.is_none() {
        return result;
    }
    let from_e = from_e.unwrap();

    let mut add_if_passable = |to: RowCol| {
        let to_e = get_elevation(&grid, to.clone());
        if to_e.is_none() {
            return;
        }
        let to_e = to_e.unwrap();
        if candidate_test(from_e, to_e) {
            result.push(to);
        }
    };

    add_if_passable(RowCol::new(from.row() - 1, from.col()));
    add_if_passable(RowCol::new(from.row() + 1, from.col()));
    add_if_passable(RowCol::new(from.row(), from.col() + 1));
    add_if_passable(RowCol::new(from.row(), from.col() - 1));

    result
}

pub fn get_elevation(grid: &Grid, rc: RowCol) -> Option<u8> {
    grid.get(rc).map(|re| match re {
        b'S' => b'a',
        b'E' => b'z',
        other => other,
    })
}

pub fn part1() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(12)?;
    let grid = Grid::new(&lines);
    let (start, end) = get_start_and_end(&grid);

    info!("Elevations:\n{}", &grid);

    let search = Search::new_uphill_search(&grid, start, end);

    let result = search.find_shortest_path(&None);

    if let Some(path) = result.shortest {
        debug!("Shortest Path={:?}", path);
        grid.log_moves_over_dots(log::Level::Info, &path);
        assert_eq!(path.len() - 1, 380);
        return Ok(format!("{}", path.len() - 1));
    }
    panic!("Unable to find a shortest path")
}

pub fn part2() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(12)?;
    let grid = Grid::new(&lines);
    let locations = grid.find(HashSet::from(['S' as u8, 'a' as u8, 'E' as u8]));
    let end = *locations[&('E' as u8)].get(0).unwrap();

    let all_a_elevation_locations: Vec<RowCol> = locations[&('a' as u8)]
        .iter()
        .chain(locations[&('S' as u8)].iter())
        .map(|r| *r)
        .collect();

    let mut shortest: Option<Vec<RowCol>> = None;

    let mut last_search_result: Option<SearchState> = None;
    for current_low_spot in &all_a_elevation_locations {
        debug!("Getting shortest path to low spot {:?}", current_low_spot);

        let search_result = Search::new_uphill_search(&grid, *current_low_spot, end)
            .find_shortest_path(&last_search_result);
        if let Some(found) = &search_result.shortest {
            debug!("    got path of length {:?}", found.len() - 1);

            if let Some(s) = &shortest {
                if found.len() <= s.len() {
                    debug!(
                        "    path of len {} is shorter than shortest of len {}",
                        found.len() - 1,
                        s.len() - 1
                    );
                    shortest.replace(found.clone());
                }
            } else {
                debug!("    path is first complete path found");
                shortest.replace(found.clone());
            }
        } else {
            debug!(
                "    didn't find a path from {:?} to {:?}",
                current_low_spot, end
            );
        }
        last_search_result.replace(search_result);
    }

    debug!("Shortest Path={:?}", shortest);

    if let Some(path) = shortest {
        grid.log_moves_over_dots(log::Level::Info, &path);
        assert_eq!(path.len() - 1, 375);
        return Ok(format!("{}", path.len() - 1));
    }
    panic!("Unable to find a shortest path");
}

fn get_start_and_end(grid: &Grid) -> (RowCol, RowCol) {
    let locations = grid.find(HashSet::from(['E' as u8, 'S' as u8]));
    (
        *locations[&('S' as u8)].get(0).unwrap(),
        *locations[&('E' as u8)].get(0).unwrap(),
    )
}

// fn get_shortest(grid: &Grid, start: (usize, usize), end: (usize, usize)) -> Option<Path> {
//     let mut shortest: Option<Path> = None;
//     let mut path = Path::new();
//     path.push(start);
//     search(&grid, &mut shortest, &mut path, &end);
//     shortest
// }

#[cfg(test)]
mod tests {
    use log::info;

    use crate::{
        common::{RowCol, InputType},
        day12::{get_candidates, get_elevation, get_start_and_end, search::Search, Grid},
    };

    #[test]
    fn test_a() {
        let lines = InputType::Challenge.get_input_lines(12).unwrap();

        let grid = Grid::new(&lines);

        let (start, end) = get_start_and_end(&grid);
        assert_eq!(start, (20, 0).into());
        assert_eq!(end, (20, 88).into());

        info!("Need to get from {:?} to {:?}", start, end);

        assert_eq!(get_elevation(&grid, RowCol::new(20, 0)).unwrap(), b'a');
        assert_eq!(get_elevation(&grid, RowCol::new(20, 88)).unwrap(), b'z');
        assert_eq!(get_elevation(&grid, RowCol::new(0, 0)).unwrap(), b'a');
        assert_eq!(get_elevation(&grid, RowCol::new(9, 93)).unwrap(), b'j');
        assert_eq!(get_elevation(&grid, RowCol::new(40, 1)).unwrap(), b'b');
        assert_eq!(get_elevation(&grid, RowCol::new(29, 45)).unwrap(), b'a');
        assert_eq!(get_elevation(&grid, RowCol::new(29, 46)).unwrap(), b'c');

        let candidates = get_candidates(&grid, (20, 87).into(), Search::uphill_test);
        assert_eq!(candidates.len(), 3);
        assert!(candidates.contains(&(19, 87).into()));
        assert!(candidates.contains(&(21, 87).into()));
        assert!(candidates.contains(&(20, 86).into()));

        let candidates = get_candidates(&grid, (20, 0).into(), Search::uphill_test);
        assert_eq!(candidates.len(), 3);
        assert!(candidates.contains(&(19, 0).into()));
        assert!(candidates.contains(&(20, 1).into()));
        assert!(candidates.contains(&(21, 0).into()));
    }
}
