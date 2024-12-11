use std::collections::{BTreeSet, HashSet};

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Ready,
    OffGrid,
    LoopDetected,
}

#[derive(Clone)]
struct Simulation {
    grid: Grid,
    start_location: RowCol,
    //start_direction: Direction,
    location: RowCol,
    direction: Direction,
    path: Vec<(RowCol, Direction)>,
    index: HashSet<(RowCol, Direction)>,
}
impl Simulation {
    fn start(mut grid: Grid) -> Self {
        let (location, direction) = starting_location(&grid);
        grid.set(location, b'.');
        let path = vec![(location, direction)];
        let index = HashSet::from([(location, direction)]);
        Simulation {
            grid,
            start_location: location,
            // start_direction: direction,
            location,
            direction,
            path,
            index,
        }
    }
    fn peek_in_front(&self) -> (RowCol, Option<u8>) {
        let next_location = self.location.plus(&self.direction);
        (next_location, self.grid.get(next_location))
    }

    fn advance(&mut self) -> State {
        let (next, value) = self.peek_in_front();
        match value {
            None => State::OffGrid,
            Some(b'.') => {
                let next_entry = (next, self.direction);
                if !self.index.insert(next_entry) {
                    return State::LoopDetected;
                }
                self.location = next;
                self.path.push((self.location, self.direction));
                State::Ready
            }
            Some(b'#') | Some(b'O') => {
                let next_entry = (self.location, self.direction.turn_cw_90());
                if !self.index.insert(next_entry) {
                    return State::LoopDetected;
                }
                self.direction = next_entry.1;
                self.path.push((self.location, self.direction));
                State::Ready
            }
            Some(c) => panic!("Unknown cell {} at location {next}", c as char),
        }
    }
    fn back(&mut self) -> bool {
        let last = match self.path.pop() {
            Some(x) => x,
            None => return false,
        };
        self.index.remove(&last);
        let tail = match self.path.last() {
            Some(x) => x,
            None => return false,
        };
        self.location = tail.0;
        self.direction = tail.1;
        true
    }
    fn run_to_end(&mut self) -> State {
        loop {
            match self.advance() {
                State::Ready => continue,
                State::OffGrid => return State::OffGrid,
                State::LoopDetected => return State::LoopDetected,
            }
        }
    }
}

fn starting_location(grid: &Grid) -> (RowCol, Direction) {
    (grid.find(b'^').expect("Unable to find start"), Direction::N)
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let mut s = Simulation::start(g);
    s.run_to_end();

    let answer = s.path.iter().map(|(l, _)| l).collect::<HashSet<_>>().len();
    Ok(answer.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let g = Grid::new(&input.get_input_lines()?);
    let mut s = Simulation::start(g);
    s.run_to_end();

    let mut o_locs = BTreeSet::new();
    loop {
        if !s.back() {
            break;
        }
        // eprint!(
        //     "At {0} facing {1} (path len={2}) ",
        //     s.location,
        //     s.direction,
        //     s.path.len()
        // );

        let (in_front_location, v) = s.peek_in_front();
        if in_front_location == s.start_location {
            // the guard was standing here at the start, can't place the O
            continue;
        }
        match v {
            None => {
                // in front of us is off grid, can't place an O there
                continue;
            }
            Some(b'#') => {
                // there's already something in front of us
                continue;
            }
            Some(b'.') => {
                if s.index.iter().any(|(location, _)| location == &in_front_location) {
                    // We went through that block in front of us to get here.  Putting an O there would not allow us to get here on the path we did
                } else {
                    
                    // run sim from here with an O in front of us                   
                    let mut cloned_sim = s.clone();
                    cloned_sim.grid.set(in_front_location, b'O');

                    if cloned_sim.run_to_end() == State::LoopDetected {
                        o_locs.insert(in_front_location);
                    }
                }
            }
            Some(c) => panic!("Unknown cell {} at location {in_front_location}", c as char),
        }
    }
    
    let answer = o_locs.len();
    Ok(answer.to_string())
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
