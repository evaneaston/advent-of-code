use crate::common::{AocError, InputType};
use log::debug;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    io::Error,
    str::FromStr,
};

#[derive(Debug)]
enum Direction {
    U,
    L,
    R,
    D,
}

impl FromStr for Direction {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "R" => Ok(Direction::R),
            "L" => Ok(Direction::L),
            c => Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Unable to parse {} as a Direction", c),
            )),
        }
    }
}

fn parse_moves() -> Result<Vec<(Direction, u16)>, Error> {
    let lines = InputType::Challenge.get_input_lines(9)?;

    Ok(lines
        .iter()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();
            assert_eq!(parts.len(), 2);

            let direction: Direction = parts[0].parse().unwrap();
            let count: u16 = parts[1].parse().unwrap();

            (direction, count)
        })
        .collect())
}

/***
 * A cartesion oriented point
 */
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct XYPair {
    x: i32,
    y: i32,
}
impl XYPair {
    fn new(x: i32, y: i32) -> XYPair {
        XYPair { x, y }
    }
    fn plus(&self, other: &XYPair) -> XYPair {
        XYPair::new(self.x + other.x, self.y + other.y)
    }
    fn minus(&self, other: &XYPair) -> XYPair {
        XYPair::new(self.x - other.x, self.y - other.y)
    }

    fn step(&self, direction: &Direction) -> XYPair {
        match direction {
            Direction::U => XYPair::new(self.x, self.y + 1),
            Direction::L => XYPair::new(self.x - 1, self.y),
            Direction::R => XYPair::new(self.x + 1, self.y),
            Direction::D => XYPair::new(self.x, self.y - 1),
        }
    }
}
impl Display for XYPair {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Simulation {
    knots: Vec<XYPair>,
}

fn is_0_0(xy: &XYPair) -> bool {
    xy.x.abs() == 0 && xy.y.abs() == 0
}
fn is_0_1(xy: &XYPair) -> bool {
    (xy.x.abs() == 1 && xy.y.abs() == 0) || (xy.x.abs() == 0 && xy.y.abs() == 1)
}
fn is_0_2(xy: &XYPair) -> bool {
    (xy.x.abs() == 2 && xy.y.abs() == 0) || (xy.x.abs() == 0 && xy.y.abs() == 2)
}
fn is_1_2(xy: &XYPair) -> bool {
    (xy.x.abs() == 2 && xy.y.abs() == 1) || (xy.x.abs() == 1 && xy.y.abs() == 2)
}
fn is_1_1(xy: &XYPair) -> bool {
    xy.x.abs() == 1 && xy.y.abs() == 1
}
fn is_2_2(xy: &XYPair) -> bool {
    xy.x.abs() == 2 && xy.y.abs() == 2
}

impl Simulation {
    fn new(size: usize) -> Self {
        Self {
            knots: vec![XYPair::new(0, 0); size],
        }
    }

    fn step(&mut self, direction: &Direction) {
        self.knots[0] = self.knots[0].step(direction);

        for i in 1..self.knots.len() {
            let preceding = &self.knots[i - 1];
            let current = &self.knots[i];

            let ht_delta = preceding.minus(current);

            if is_0_2(&ht_delta) || is_1_2(&ht_delta) || is_2_2(&ht_delta) {
                self.knots[i] =
                    self.knots[i].plus(&XYPair::new(1 * ht_delta.x.signum(), ht_delta.y.signum()));
            } else if is_0_0(&ht_delta) || is_0_1(&ht_delta) || is_1_1(&ht_delta) {
                // nothing to do
            } else {
                panic!(
                    "Something went wrong on move {:?} for knot {}.  Follower {:?} is too far from preceding {:?}.",
                    direction, i, current, preceding
                )
            }
        }
    }
}

impl Display for Simulation {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.knots)
    }
}

#[cfg(test)]
mod tests {
    use log::debug;

    use crate::day09::{Direction, Simulation, XYPair};

    #[test]
    fn test_sim_step() {
        let mut sim = Simulation::new(2);

        debug!("{:?}", sim);

        sim.step(&Direction::R);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(1, 0));
        assert_eq!(sim.knots[1], XYPair::new(0, 0));

        sim.step(&Direction::R);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(2, 0));
        assert_eq!(sim.knots[1], XYPair::new(1, 0));

        sim.step(&Direction::R);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(3, 0));
        assert_eq!(sim.knots[1], XYPair::new(2, 0));

        sim.step(&Direction::U);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(3, 1));
        assert_eq!(sim.knots[1], XYPair::new(2, 0));

        sim.step(&Direction::U);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(3, 2));
        assert_eq!(sim.knots[1], XYPair::new(3, 1));

        sim.step(&Direction::U);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(3, 3));
        assert_eq!(sim.knots[1], XYPair::new(3, 2));

        sim.step(&Direction::D);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(3, 2));
        assert_eq!(sim.knots[1], XYPair::new(3, 2));

        sim.step(&Direction::L);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(2, 2));
        assert_eq!(sim.knots[1], XYPair::new(3, 2));

        sim.step(&Direction::D);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(2, 1));
        assert_eq!(sim.knots[1], XYPair::new(3, 2));

        sim.step(&Direction::D);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(2, 0));
        assert_eq!(sim.knots[1], XYPair::new(2, 1));

        sim.step(&Direction::R);
        debug!("{:?}", sim);
        sim.step(&Direction::U);
        debug!("{:?}", sim);
        sim.step(&Direction::U);
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(3, 2));
        assert_eq!(sim.knots[1], XYPair::new(2, 1));
    }

    #[test]
    fn test_sim_example() {
        let mut sim = Simulation::new(2);

        debug!("{:?}", sim);

        for _ in 0..4 {
            sim.step(&Direction::R)
        }
        debug!("{:?}", sim);

        for _ in 0..4 {
            sim.step(&Direction::U);
        }
        debug!("{:?}", sim);

        for _ in 0..3 {
            sim.step(&Direction::L);
        }
        debug!("{:?}", sim);

        sim.step(&Direction::D);
        debug!("{:?}", sim);

        for _ in 0..4 {
            sim.step(&Direction::R)
        }
        debug!("{:?}", sim);

        sim.step(&Direction::D);
        debug!("{:?}", sim);

        for _ in 0..5 {
            sim.step(&Direction::L);
        }
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(0, 2));
        assert_eq!(sim.knots[1], XYPair::new(1, 2));

        for _ in 0..2 {
            sim.step(&Direction::R);
        }
        debug!("{:?}", sim);
        assert_eq!(sim.knots[0], XYPair::new(2, 2));
        assert_eq!(sim.knots[1], XYPair::new(1, 2));
    }
}

pub fn part1() -> Result<String, AocError> {
    let mut sim = Simulation::new(2);
    debug!("{:?}", sim);
    let mut set = HashSet::<XYPair>::new();
    for m in &parse_moves()? {
        for _ in 0..m.1 {
            sim.step(&m.0);
            // debug!("{:?} -> {}", m.0, sim);
            set.insert(sim.knots[1].clone());
        }
    }
    Ok(format!("Tail has been in {} unique locations.", set.len()))
}

pub fn part2() -> Result<String, AocError> {
    let mut sim = Simulation::new(10);
    //debug!("{:?}", sim);
    let mut set = HashSet::<XYPair>::new();
    for m in &parse_moves()? {
        for _ in 0..m.1 {
            sim.step(&m.0);
            //debug!("{:?} -> {}", m.0, sim);
            set.insert(sim.knots[9].clone());
        }
    }
    Ok(format!("Tail has been in {} unique locations.", set.len()))
}
