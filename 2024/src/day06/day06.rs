use std::{collections::HashSet, process::Command};

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let mut g = Grid::new(&input.get_input_lines()?);
    let locations = g.find(HashSet::from([b'^']));
    let mut location = *locations.get(&b'^').unwrap().first().unwrap();
    let mut direction = Direction::N;

    let mut visited_locations = HashSet::new();
    while g.get(location).is_some() {
        visited_locations.insert(location);
        g.set(location, b'X');

        eprintln!("{location}");
        let maybe_next = location.plus(&direction);
        match g.get(maybe_next) {
            Some(b'#') => direction = direction.cw_90(),
            Some(_) => location = maybe_next,
            None => break,
        }
    }
    // eprintln!("{g}");
    let answer = visited_locations.len();

    Ok(format!("{answer}"))
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let mut original_grid = Grid::new(&lines);
    let locations = original_grid.find(HashSet::from([b'^']));
    let starting_location = *locations.get(&b'^').unwrap().first().unwrap();
    let starting_direction = Direction::N;
    original_grid.set(starting_location, b'.');

    let mut tried_options = HashSet::<RowCol>::new();
    let mut successful_options = HashSet::<RowCol>::new();
    eprintln!("F");

    'outer: loop {
        let mut g = original_grid.clone();
        let mut location = starting_location;
        let mut direction = starting_direction;
        let mut candidate_option: Option<RowCol> = None;

        let mut visited = HashSet::<(RowCol, Direction)>::new();

        eprintln!("Starting trial having already tried {} options and found {} successful options", tried_options.len(), successful_options.len());
        'trial: loop {
            if !visited.insert((location, direction)) {
                // we've been here before
                if let Some(option) = candidate_option {
                    successful_options.insert(option);
                }
                break 'trial;
            }

            g.set(location, b'X');
            let ahead = location.plus(&direction);

            match g.get(ahead) {
                None => match candidate_option {
                    Some(_) => break 'trial, // went off grid while testing an option, try again, there may be more
                    None => break 'outer, // went off grid without any tested option, trying again won't work
                },
                Some(b'.') | Some(b'X') => {
                    if candidate_option.is_none() && tried_options.insert(ahead) {
                        g.set(ahead, b'O');
                        candidate_option.replace(ahead);
                        direction = direction.cw_90();
                    } else {
                        location = ahead;
                    }
                }
                Some(b'#') | Some(b'O') => direction = direction.cw_90(),
                Some(c) => panic!("Unknown cell {} at location {ahead}", c as char),
            }
        }   
    }
    let answer = successful_options.len();
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
