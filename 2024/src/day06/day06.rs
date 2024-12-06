use std::{collections::HashSet, hash::Hash};

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

fn get_visited_locations(g: &Grid, starting_location: RowCol, starting_direction: Direction) -> HashSet<RowCol> {
    let mut location = starting_location;
    let mut direction = starting_direction;
    let mut visited_locations = HashSet::new();
    while g.get(location).is_some() {
        visited_locations.insert(location);

        let ahead = location.plus(&direction);
        match g.get(ahead) {
            Some(b'#') => direction = direction.cw_90(),
            Some(_) => location = ahead,
            None => break,
        }
    }
    visited_locations
}

fn starting_location(grid: &Grid) -> (RowCol, Direction) {
    (*grid.find(HashSet::from([b'^'])).get(&b'^').unwrap().first().unwrap(), Direction::N)
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let (location, direction) = starting_location(&g);
    let answer = get_visited_locations(&g, location, direction).len();
    Ok(format!("{answer}"))
}

fn has_cycle(g: &Grid, starting_location: RowCol, starting_direction: Direction) -> bool {
    let mut visited = HashSet::<(RowCol,Direction)>::new();
    let mut location = starting_location;
    let mut direction = starting_direction;

    loop {
        if !visited.insert((location, direction)) {
            // we've been here before
            return true;
        }

        let ahead = location.plus(&direction);

        match g.get(ahead) {
            None => return false,
            Some(b'.') => location = ahead,
            Some(b'#') | Some(b'O') => direction = direction.cw_90(),
            Some(c) => panic!("Unknown cell {} at location {ahead}", c as char),
        }
    }
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let mut g = Grid::new(&input.get_input_lines()?);
    let (starting_location, starting_direction) = starting_location(&g);
    g.set(starting_location, b'.');

    let mut locations = get_visited_locations(&g, starting_location, starting_direction);
    locations.remove(&starting_location); /* cannot put anything in the starting point */

    let answer = locations
        .iter()
        .filter(|&option| {
            g.set(*option, b'O');
            let produces_cycle = has_cycle(&g, starting_location, starting_direction);
            g.set(*option, b'.');
            produces_cycle
        })
        .count();
    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 6;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "41"
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
            "5404"
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
            "6"
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
            "1984"
        );
    }
}
