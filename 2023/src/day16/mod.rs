use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use log::debug;

use crate::{grid::Grid, AocError, DailyInput, RowCol};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    E,
    S,
    W,
    N,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Beam(RowCol, Direction);
impl Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.0, self.1)
    }
}

fn try_reflect(beam: Beam, c: u8) -> Vec<Beam> {
    match (beam.1, c) {
        (Direction::E, b'\\') => vec![Direction::S],
        (Direction::E, b'/') => vec![Direction::N],
        (Direction::E, b'|') => vec![Direction::N, Direction::S],
        (Direction::E, b'-') => vec![Direction::E],

        (Direction::W, b'\\') => vec![Direction::N],
        (Direction::W, b'/') => vec![Direction::S],
        (Direction::W, b'|') => vec![Direction::N, Direction::S],
        (Direction::W, b'-') => vec![Direction::W],

        (Direction::N, b'\\') => vec![Direction::W],
        (Direction::N, b'/') => vec![Direction::E],
        (Direction::N, b'|') => vec![Direction::N],
        (Direction::N, b'-') => vec![Direction::W, Direction::E],

        (Direction::S, b'\\') => vec![Direction::E],
        (Direction::S, b'/') => vec![Direction::W],
        (Direction::S, b'|') => vec![Direction::S],
        (Direction::S, b'-') => vec![Direction::W, Direction::E],

        _ => vec![beam.1],
    }
    .iter()
    .map(|d| {
        Beam(
            match d {
                Direction::E => beam.0.plus_col(),
                Direction::S => beam.0.plus_row(),
                Direction::W => beam.0.minus_col(),
                Direction::N => beam.0.minus_row(),
            },
            *d,
        )
    })
    .collect::<Vec<_>>()
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = load_grid(input)?;
    debug!("{grid}");

    let answer = energy_from(&grid, Beam(RowCol::new(0, 0), Direction::E));
    Ok(answer.to_string())
}

fn energy_from(grid: &Grid, start: Beam) -> usize {
    let mut beams_set: HashSet<Beam> = HashSet::new();
    let mut beams: VecDeque<Beam> = VecDeque::new();

    beams.push_back(start);

    while !beams.is_empty() {
        let b = beams.pop_front().unwrap();
        match grid.get(b.0) {
            Some(character) => {
                debug!("  {b} is on grid");

                if beams_set.insert(b) {
                    debug!("    {b} is new");

                    let next = try_reflect(b, character);
                    debug!("      next: {:?}", next);

                    next.iter().for_each(|next_beam| beams.push_back(*next_beam));
                } else {
                    debug!("    {b} has been checked already");
                }
            }
            None => debug!("  {b} is off grid"),
        }
    }
    debug!("beams set: {:?}", beams_set);

    let answer = beams_set.iter().map(|b| b.0).collect::<HashSet<_>>().len();
    answer
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let grid = load_grid(input)?;
    debug!("{grid}");

    let max: usize = (grid.min_row()..=grid.max_row())
        .flat_map(|row| {
            let from_start = Beam(RowCol::new(row, grid.min_col()), Direction::E);
            let from_end = Beam(RowCol::new(row, grid.max_col()), Direction::W);
            vec![
                (from_start, energy_from(&grid, from_start)),
                (from_end, energy_from(&grid, from_end)),
            ]
        })
        .map(|e| {
            debug!("{:?}", e);
            e.1
        })
        .chain(
            (grid.min_col()..=grid.max_col())
                .flat_map(|col| {
                    let from_start = Beam(RowCol::new(grid.min_row(), col), Direction::S);
                    let from_end = Beam(RowCol::new(grid.max_row(), col), Direction::N);
                    vec![
                        (from_start, energy_from(&grid, from_start)),
                        (from_end, energy_from(&grid, from_end)),
                    ]
                })
                .map(|e| {
                    debug!("{:?}", e);
                    e.1
                }),
        )
        .max()
        .unwrap();

    Ok(max.to_string())
}

pub(crate) fn load_grid(input: DailyInput) -> Result<Grid, AocError> {
    let lines = input.get_input_lines()?;
    Ok(Grid::new(&lines))
}

#[cfg(test)]
mod test {
    use crate::{
        day16::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 16,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "46"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 16,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "8539"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 16,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "51"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 16,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "8674"
        );
    }
}
