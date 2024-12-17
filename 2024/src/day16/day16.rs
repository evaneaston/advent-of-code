use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
};

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

struct Sim {
    grid: Grid,
    costs: HashMap<RowCol, Cost>,
}
#[derive(Debug, Clone, Copy)]
struct Cost {
    cost: u64,
    at_direction: Direction,
}
impl Display for Cost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.cost, self.at_direction)
    }
}
fn count_cw_turns(current: Direction, target: Direction) -> u64 {
    let mut d = current;
    let mut count = 0;
    while d != target {
        d = d.turn_cw_90();
        count += 1;
    }
    count
}
fn count_ccw_turns(current: Direction, target: Direction) -> u64 {
    let mut d = current;
    let mut count = 0;
    while d != target {
        d = d.turn_ccw_90();
        count += 1;
    }
    count
}
impl Sim {
    fn run(&mut self) -> u64 {
        // eprintln!("run");
        let start = self.grid.find(b'S').unwrap();
        let end = self.grid.find(b'E').unwrap();

        eprintln!("Start {start} End {end}");

        self.grid.set(start, b'.');
        self.costs.insert(
            start,
            Cost {
                cost: 0,
                at_direction: Direction::E,
            },
        );
        self.search(start, Direction::E, end);

        self.costs.get(&end).unwrap().cost
    }

    fn search(&mut self, location: RowCol, direction: Direction, end: RowCol) {
        // eprintln!("search({location}, {direction}, {depth})");
        let candidates: Vec<(RowCol, Direction, u64)> = [Direction::N, Direction::S, Direction::E, Direction::W]
            .iter()
            .filter_map(|candidate_direction| {
                let candidate_location = location.plus(candidate_direction);

                match self.grid.get(candidate_location) {
                    Some(b'#') => None,
                    Some(b'.') | Some(b'E') => {
                        let count_cw_turns = count_cw_turns(direction, *candidate_direction);
                        let count_ccw_turns = count_ccw_turns(direction, *candidate_direction);
                        Some((
                            candidate_location,
                            *candidate_direction,
                            1 + (1000 * count_ccw_turns.min(count_cw_turns)),
                        ))
                    }
                    Some(other) => panic!("Unexpected value {} at location {}", other as char, candidate_location),
                    None => panic!("Shouldn't be able to walk off the edge of the world."),
                }
            })
            .collect::<Vec<_>>();
        let current_cost = *self.costs.get(&location).unwrap();

        // eprintln!(
        //     "At {} facing {} with current cost of {}",
        //     location, direction, current_cost
        // );

        for candidate in candidates {
            // eprintln!(
            //     "  Considering going {} to {} for an added cost of {}",
            //     candidate.1, candidate.0, candidate.2
            // );
            // };

            let proceed = match self.costs.get(&candidate.0) {
                Some(existing_minimum_cost) => {
                    let new_cost = current_cost.cost + candidate.2;
                    let new_cost_at_existing_min_direction = new_cost
                        + count_ccw_turns(candidate.1, existing_minimum_cost.at_direction)
                            .min(count_cw_turns(candidate.1, existing_minimum_cost.at_direction))
                            * 1000;

                    if new_cost_at_existing_min_direction < existing_minimum_cost.cost {
                        if candidate.0 == end {
                            eprintln!(
                                "   replacing {} min cost of {} with new min cost of {} at direction {}",
                                candidate.0, existing_minimum_cost, new_cost, candidate.1
                            );
                        }
                        // eprintln!(
                        //     "       Existing cost is {}.  We can do it for less {}.",
                        //     *existing_minimum_cost,
                        //     (current_cost + candidate.2),
                        // );
                        true
                    } else {
                        // eprintln!("    Can already get there with a lower cost.");
                        false
                    }
                }
                None => {
                    // eprintln!(
                    //     "    No cost has been registered for {}.  Will go check it out.",
                    //     candidate.0,
                    // );
                    true
                }
            };

            if proceed {
                self.costs.insert(
                    candidate.0,
                    Cost {
                        cost: current_cost.cost + candidate.2,
                        at_direction: candidate.1,
                    },
                );
                self.search(candidate.0, candidate.1, end);
            }
        }
    }

    fn n(&self, location: RowCol, chain: &mut Vec<RowCol>, solutions: &mut BTreeSet<RowCol>) {
        let current_cost = self.costs.get(&location).unwrap();
        if current_cost.cost == 0 {
            for c in chain {
                solutions.insert(*c);
            }
            solutions.insert(location);
        } else {
            for direction in [Direction::N, Direction::E, Direction::S, Direction::W] {
                let next_location = location.plus(&direction);
                if let Some(next_value) = self.grid.get(next_location) {
                    if next_value == b'.' {
                        let next_cost = self.costs.get(&next_location).unwrap();
                        if next_cost.cost < current_cost.cost {
                            chain.push(next_location);
                            self.n(next_location, chain, solutions);
                            chain.pop();
                        }
                    }
                }
            }
        }
    }

    fn search_back(&self) -> BTreeSet<RowCol> {
        let end = self.grid.find(b'E').unwrap();

        let mut chain = Vec::new();
        chain.push(end);
        let mut solutions = BTreeSet::<RowCol>::new();
        self.n(end, &mut chain, &mut solutions);
        solutions
    }
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);

    let mut sim = Sim {
        grid,
        costs: HashMap::new(),
    };

    let answer = sim.run();
    sim.grid.all_cell_locations_by_row_by_col().for_each(|rc| {
        let v = sim.grid.get(rc).unwrap();
        if (v == b'.' || v == b'E') && sim.costs.get(&rc).is_none() {
            panic!("Location {rc} doesn't have a cost but should.");
        }
    });

    eprintln!("==================================================");
    for r in sim.grid.rows() {
        for c in sim.grid.cols() {
            match sim.costs.get(&RowCol(r, c)) {
                Some(cost) => eprint!("{},", cost.cost),
                None => eprint!(","),
            }
        }
        eprintln!();
    }
    eprintln!("==================================================");

    Ok(answer.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);

    let mut sim = Sim {
        grid,
        costs: HashMap::new(),
    };

    sim.run();
    let answers = sim.search_back();
    answers.iter().for_each(|rc| {
        eprintln!("{rc}");
    });
    let answer = answers.len();

    Ok(answer.to_string())
}

#[cfg(test)]
mod test {
    use super::{count_ccw_turns, count_cw_turns, part1, part2};
    use crate::{coord::Direction, DailyInput, InputType};

    const DAY: usize = 16;

    #[test]
    fn test_count_ccw_turns() {
        assert_eq!(count_ccw_turns(Direction::N, Direction::N), 0);
        assert_eq!(count_ccw_turns(Direction::N, Direction::W), 1);
        assert_eq!(count_ccw_turns(Direction::N, Direction::S), 2);
        assert_eq!(count_ccw_turns(Direction::N, Direction::E), 3);
    }

    #[test]
    fn test_count_cw_turns() {
        assert_eq!(count_cw_turns(Direction::N, Direction::N), 0);
        assert_eq!(count_cw_turns(Direction::N, Direction::W), 3);
        assert_eq!(count_cw_turns(Direction::N, Direction::S), 2);
        assert_eq!(count_cw_turns(Direction::N, Direction::E), 1);
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(1),
            })
            .unwrap(),
            "7036"
        );
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(2),
            })
            .unwrap(),
            "11048"
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
            "82460"
        );
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(1),
            })
            .unwrap(),
            "45"
        );
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: Some(2),
            })
            .unwrap(),
            "64"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Challenge,
                number: Some(2),
            })
            .unwrap(),
            ""
        );
    }
}
