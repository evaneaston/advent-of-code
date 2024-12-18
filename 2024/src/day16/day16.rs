use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, VecDeque},
    fmt::Display,
};

use crate::{
    coord::{Direction, RowCol},
    grid::Grid,
    AocError, DailyInput,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct RowColDir(RowCol, Direction);
impl Display for RowColDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct HeapEntry(u64, RowColDir);

struct Sim {
    grid: Grid,

    #[allow(dead_code)]
    start: RowCol,
    start_rcd: RowColDir,
    end: RowCol,
    costs: HashMap<RowColDir, u64>,
}

impl Sim {
    fn add_for_future_exploration(
        &mut self,
        heap: &mut BinaryHeap<Reverse<HeapEntry>>,
        cost: u64,
        rcd: RowColDir,
    ) -> bool {
        let existing_cost = *self.costs.get(&rcd).unwrap();
        if existing_cost > cost {
            self.costs.insert(rcd, cost);
            // eprintln!(
            //     "Existing cost for {rcd:?} of {existing_cost} is higher than {cost}, queuing for future exploration"
            // );
            heap.push(Reverse(HeapEntry(cost, rcd)));
            true
        } else {
            // eprintln!("{rcd:?} with cost {existing_cost} can already be reached at a lower cost than {cost}, will not explore futher ");
            false
        }
    }

    fn run(&mut self) -> u64 {
        self.grid.all_cell_locations_by_row_by_col().for_each(|loc| {
            [Direction::N, Direction::S, Direction::E, Direction::W].iter().for_each(|dir| {
                self.costs.insert(RowColDir(loc, *dir), u64::MAX);
            });
        });

        let mut heap = BinaryHeap::new();
        self.costs.insert(self.start_rcd, 0);
        heap.push(Reverse(HeapEntry(0, self.start_rcd)));

        loop {
            let entry = match heap.pop() {
                None => break,
                Some(e) => e,
            };
            // eprintln!("Checking {} @ {}>{}", entry.0 .0, entry.0 .1 .0, entry.0 .1 .1);

            let cost = entry.0 .0;
            let row_col_dir = entry.0 .1;

            // if !self.add_for_future_exploration(&mut heap, cost, row_col_dir) {
            //     continue;
            // }

            let existing_cost = *self.costs.get(&row_col_dir).unwrap();
            if existing_cost < cost {
                continue;
            }

            if row_col_dir.0 == self.end {
                // eprintln!("found end, not need to continue moving from here");
                continue;
            }

            // forward step
            let next_rcd = RowColDir(row_col_dir.0.plus(&row_col_dir.1), row_col_dir.1);
            match self.grid.get(next_rcd.0) {
                Some(b'.') | Some(b'E') => {
                    self.add_for_future_exploration(&mut heap, cost + 1, next_rcd);
                }
                _ => {}
            }

            // ccw
            self.add_for_future_exploration(
                &mut heap,
                cost + 1000,
                RowColDir(row_col_dir.0, row_col_dir.1.turn_ccw_90()),
            );

            // cw
            self.add_for_future_exploration(
                &mut heap,
                cost + 1000,
                RowColDir(row_col_dir.0, row_col_dir.1.turn_cw_90()),
            );
        }

        self.min_cost(self.end)
    }

    fn min_cost(&self, location: RowCol) -> u64 {
        [Direction::N, Direction::S, Direction::E, Direction::W]
            .iter()
            .map(|d| *self.costs.get(&RowColDir(location, *d)).unwrap())
            .min()
            .unwrap()
    }

    // #[allow(dead_code)]
    // fn n0(
    //     &self,
    //     start_rcd: RowColDir,
    //     current_rcd: RowColDir,
    //     chain: &mut BTreeSet<RowColDir>,
    //     solutions: &mut BTreeSet<RowCol>,
    // ) {
    //     eprintln!("Checking {}", current_rcd);
    //     if start_rcd == current_rcd {
    //         eprintln!("Found start {current_rcd}");

    //         for c in chain.iter() {
    //             solutions.insert(c.0);
    //         }
    //         return;
    //     }

    //     let current_cost = self.costs.get(&current_rcd).unwrap();

    //     // straight in
    //     {
    //         let prev_rcd = RowColDir(current_rcd.0.minus(&current_rcd.1), current_rcd.1);

    //         if !chain.contains(&prev_rcd) {
    //             if let Some(prev_value) = self.grid.get(prev_rcd.0) {
    //                 if prev_value == b'.' || prev_value == b'S' {
    //                     eprint!("  {prev_rcd} ({}) -?-> {current_rcd}", prev_value as char);

    //                     let prev_cost = *self.costs.get(&prev_rcd).unwrap();
    //                     if (prev_cost + 1) == *current_cost {
    //                         eprintln!("  check {prev_cost} + 1 = {current_cost}");
    //                         chain.insert(prev_rcd);
    //                         self.n(start_rcd, prev_rcd, chain, solutions);
    //                         chain.remove(&prev_rcd);
    //                     } else {
    //                         eprintln!(" NO - cost isn't 1");
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     // from ccw
    //     {
    //         let prev_rcd = RowColDir(current_rcd.0, current_rcd.1.turn_ccw_90());
    //         if !chain.contains(&prev_rcd) {
    //             eprint!("  {prev_rcd} -?-> {current_rcd}");
    //             let prev_cost = *self.costs.get(&prev_rcd).unwrap();
    //             if (prev_cost + 1000) == *current_cost {
    //                 eprintln!("  check {prev_cost} + 1000 = {current_cost}");
    //                 chain.insert(prev_rcd);
    //                 self.n(start_rcd, prev_rcd, chain, solutions);
    //                 chain.remove(&prev_rcd);
    //             } else {
    //                 eprintln!(" NO - cost isn't 1000");
    //             }
    //         }
    //     }

    //     // from cw
    //     {
    //         let prev_rcd = RowColDir(current_rcd.0, current_rcd.1.turn_cw_90());
    //         if !chain.contains(&prev_rcd) {
    //             eprint!("  {prev_rcd} -?-> {current_rcd}");
    //             let prev_cost = *self.costs.get(&prev_rcd).unwrap();
    //             if (prev_cost + 1000) == *current_cost {
    //                 eprintln!("  check {prev_cost} + 1000 = {current_cost}");
    //                 chain.insert(prev_rcd);
    //                 self.n(start_rcd, prev_rcd, chain, solutions);
    //                 chain.remove(&prev_rcd);
    //             } else {
    //                 eprintln!(" NO - cost isn't 1000");
    //             }
    //         }
    //     }
    // }

    fn draw(&self, chain: &BTreeSet<RowColDir>) {
        let mut grid = self.grid.clone();
        for rcd in chain {
            grid.set(rcd.0, rcd.1.into());
        }
        eprintln!("{grid}");
    }

    fn n(
        &self,
        start: RowColDir,
        current_rcd: RowColDir,
        chain: &mut BTreeSet<RowColDir>,
        solutions: &mut BTreeSet<RowCol>,
    ) {
        eprintln!("Checking {}", current_rcd);
        if start == current_rcd {
            eprintln!("Found start {current_rcd}");

            for c in chain.iter() {
                solutions.insert(c.0);
            }

            self.draw(chain);

            return;
        }

        let current_cost = self.costs.get(&current_rcd).unwrap();

        let mut recurses = VecDeque::<RowColDir>::new();
        // straight in
        {
            let prev_rcd = RowColDir(current_rcd.0.minus(&current_rcd.1), current_rcd.1);

            eprint!("  {prev_rcd} -?-> {current_rcd}");
            if !chain.contains(&prev_rcd) {
                if let Some(prev_value) = self.grid.get(prev_rcd.0) {
                    if prev_value == b'.' || prev_value == b'S' {
                        let prev_cost = *self.costs.get(&prev_rcd).unwrap();
                        if *current_cost >= 1 && (current_cost - 1) == prev_cost {
                            eprintln!("  check - {prev_cost} + 1 = {current_cost}");
                            recurses.push_back(prev_rcd);
                        } else {
                            eprintln!("  NO - {prev_cost} + 1 != {current_cost}");
                        }
                    } else {
                        eprintln!("  NO - invalid value {}", prev_value as char);
                    }
                } else {
                    eprintln!("  NO - not on grid");
                }
            }
        }

        // from ccw
        {
            let prev_rcd = RowColDir(current_rcd.0, current_rcd.1.turn_ccw_90());
            if !chain.contains(&prev_rcd) {
                eprint!("  {prev_rcd} -?-> {current_rcd}");
                let prev_cost = *self.costs.get(&prev_rcd).unwrap();
                if *current_cost >= 1000 && (current_cost - 1000) == prev_cost {
                    eprintln!("  check - {prev_cost} + 1000 = {current_cost}");
                    recurses.push_back(prev_rcd);
                } else {
                    eprintln!("  NO - {prev_cost} + 1000 != {current_cost}");
                }
            }
        }

        // from cw
        {
            let prev_rcd = RowColDir(current_rcd.0, current_rcd.1.turn_cw_90());
            if !chain.contains(&prev_rcd) {
                eprint!("  {prev_rcd} -?-> {current_rcd}");
                let prev_cost = *self.costs.get(&prev_rcd).unwrap();
                if *current_cost >= 1000 && (current_cost - 1000) == prev_cost {
                    eprintln!("  check - {prev_cost} + 1000 = {current_cost}");
                    recurses.push_back(prev_rcd);
                } else {
                    eprintln!("  NO - {prev_cost} + 1000 != {current_cost}");
                }
            }
        }

        while let Some(prev_rcd) = recurses.pop_front() {
            chain.insert(prev_rcd);
            self.n(start, prev_rcd, chain, solutions);
            chain.remove(&prev_rcd);
        }
    }

    fn search_back(&self, answer: u64) -> BTreeSet<RowCol> {
        let end = self.grid.find(b'E').unwrap();
        let start = self.grid.find(b'S').unwrap();

        let mut chain = BTreeSet::new();
        let mut solutions = BTreeSet::<RowCol>::new();

        for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
            let end_rcd = RowColDir(end, direction);
            if *self.costs.get(&end_rcd).unwrap() == answer {
                chain.insert(end_rcd);
                self.n(RowColDir(start, Direction::E), end_rcd, &mut chain, &mut solutions);
                chain.remove(&end_rcd);
            }
        }
        solutions
    }

    fn new(grid: Grid) -> Self {
        let start = grid.find(b'S').unwrap();
        Self {
            start_rcd: RowColDir(start, Direction::E),
            start,
            end: grid.find(b'E').unwrap(),
            grid,
            costs: HashMap::new(),
        }
    }
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);

    let mut sim = Sim::new(grid);

    let answer = sim.run();

    Ok(answer.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let grid = Grid::new(&input.get_input_lines()?);

    let mut sim = Sim::new(grid);

    let answer = sim.run();
    let answers = sim.search_back(answer);

    answers.iter().for_each(|rc| {
        sim.grid.set(*rc, b'O');
    });

    eprintln!("{}", sim.grid);

    let answer = answers.len();

    Ok(answer.to_string())
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 16;

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
                number: None,
            })
            .unwrap(),
            "590"
        );
    }
}
