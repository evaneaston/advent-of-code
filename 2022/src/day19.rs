use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub},
};

use log::{debug, info};
use nom::{bytes::complete::tag, multi::separated_list1, sequence::tuple, IResult};

use crate::common::{AocError, InputType};

pub fn part1() -> Result<String, AocError> {
    let blueprints = get_input(InputType::Challenge)?;
    let answer = sum_quality(24, &blueprints);
    assert_eq!(answer, 1262);
    Ok(format!("{}", answer))
}

pub fn part2() -> Result<String, AocError> {
    let inputs = get_input(InputType::Challenge)?;
    let answer: usize = inputs[0..3].iter().map(|bp| compute(32, &bp).geode() as usize).product();
    assert_eq!(answer, 37191);
    Ok(format!("{}", answer))
}

fn sum_quality(minutes: usize, blueprints: &Vec<Blueprint>) -> usize {
    blueprints
        .iter()
        .map(|blueprint| (compute(minutes, &blueprint).geode() as usize) * blueprint.number)
        .sum()
}

fn compute(minutes: usize, blueprint: &Blueprint) -> Tally {
    info!("Blueprint {:?} for {} minutes", blueprint, minutes);

    let robots_mining = Tally::new(0, 0, 0, 1);
    let robots_being_built = Tally::new(0, 0, 0, 0);
    let rocks = Tally::new(0, 0, 0, 0);

    let mut max = Tally(0);

    max_after_minute(
        1,
        minutes,
        blueprint,
        &BlueprintTargets::new(blueprint),
        robots_mining,
        robots_being_built,
        rocks,
        &mut max,
    );
    max
}
fn max_after_minute(
    minute: usize,
    max_minute: usize,
    blueprint: &Blueprint,
    blueprint_targets: &BlueprintTargets,
    robots_mining: Tally,
    robots_being_built: Tally,
    rocks: Tally,
    max: &mut Tally,
) {
    if minute > max_minute {
        return;
    }

    let rocks_at_end_of_minute = rocks + robots_mining;
    let robots_mining_at_end_of_minute = robots_mining + robots_being_built;

    if minute == max_minute {
        if rocks_at_end_of_minute.0 > (*max).0 {
            debug!(
                "New max of {} (mining {})",
                rocks_at_end_of_minute, robots_mining
            );
            *max = rocks_at_end_of_minute;
        }
        return;
    }

    let remaining_minutes = max_minute - minute;
    let options = if remaining_minutes <= 1 {
        vec![(Tally(0), Tally(0))]
    } else {
        let mut options = Vec::with_capacity(4);

        if blueprint.add_if_possible(RType::GEODE, rocks_at_end_of_minute, &mut options) {
            // if we can build a geode robot, do it and don't try any other options
        } else {
            if blueprint
                .obsidian_robot_cost
                .all_lte(&robots_mining_at_end_of_minute)
            {
                if !blueprint.add_if_possible(RType::GEODE, rocks_at_end_of_minute, &mut options) {
                    options.push((Tally(0), Tally(0)));
                }
            } else {
                if robots_mining_at_end_of_minute.obsidian() < blueprint_targets.num_obsidian_robots
                {
                    blueprint.add_if_possible(
                        RType::OBSIDIAN,
                        rocks_at_end_of_minute,
                        &mut options,
                    );
                }
                if options.is_empty() {
                    options.push((Tally(0), Tally(0)));
                }
                if robots_mining_at_end_of_minute.clay() < blueprint_targets.num_clay_robots {
                    blueprint.add_if_possible(RType::CLAY, rocks_at_end_of_minute, &mut options);
                }
                if robots_mining_at_end_of_minute.ore() < blueprint_targets.num_ore_robots {
                    blueprint.add_if_possible(RType::ORE, rocks_at_end_of_minute, &mut options);
                }
            }
        }
        options
    };

    for i in 0..options.len() {
        let (option, cost) = options[i];
        max_after_minute(
            minute + 1,
            max_minute,
            blueprint,
            blueprint_targets,
            robots_mining_at_end_of_minute,
            option,
            rocks_at_end_of_minute - cost,
            max,
        );
    }
}

const GEODE_SHIFT: usize = 48;
const OBSIDIAN_SHIFT: usize = 32;
const CLAY_SHIFT: usize = 16;
const U16_MASK: u64 = u16::MAX as u64;
const GEODE_MASK: u64 = U16_MASK << GEODE_SHIFT;
const OBSIDIAN_MASK: u64 = U16_MASK << OBSIDIAN_SHIFT;
const CLAY_MASK: u64 = U16_MASK << CLAY_SHIFT;
const ORE_MASK: u64 = U16_MASK;

enum RType {
    GEODE,
    OBSIDIAN,
    CLAY,
    ORE,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Tally(u64);
impl Tally {
    fn one_of(rtype: &RType) -> Self {
        match rtype {
            RType::GEODE => Self(0b1 << GEODE_SHIFT),
            RType::OBSIDIAN => Self(0b1 << OBSIDIAN_SHIFT),
            RType::CLAY => Self(0b1 << CLAY_SHIFT),
            RType::ORE => Self(0b1),
        }
    }
    fn new(geode: u16, obsidian: u16, clay: u16, ore: u16) -> Self {
        Tally(
            (geode as u64) << GEODE_SHIFT
                | (obsidian as u64) << OBSIDIAN_SHIFT
                | (clay as u64) << CLAY_SHIFT
                | (ore as u64),
        )
    }
    fn geode(&self) -> u16 {
        ((self.0 >> GEODE_SHIFT) & U16_MASK) as u16
    }
    fn obsidian(&self) -> u16 {
        ((self.0 >> OBSIDIAN_SHIFT) & U16_MASK) as u16
    }
    fn clay(&self) -> u16 {
        ((self.0 >> CLAY_SHIFT) & U16_MASK) as u16
    }
    fn ore(&self) -> u16 {
        (self.0 & U16_MASK) as u16
    }
    fn all_lte(&self, rhs: &Self) -> bool {
        (self.0 & ORE_MASK) <= (rhs.0 & ORE_MASK)
            && (self.0 & CLAY_MASK) <= (rhs.0 & CLAY_MASK)
            && (self.0 & OBSIDIAN_MASK) <= (rhs.0 & OBSIDIAN_MASK)
            && (self.0 & GEODE_MASK) <= (rhs.0 & GEODE_MASK)
    }
}

impl AddAssign for Tally {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Add for Tally {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Tally {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Display for Tally {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} geode, {} obsidian, {} clay, {} ore",
            self.geode(),
            self.obsidian(),
            self.clay(),
            self.ore()
        )
    }
}
impl std::fmt::Debug for Tally {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tally")
            .field("0", &self.0)
            .field("geode", &self.geode())
            .field("obsidian", &self.obsidian())
            .field("clay", &self.clay())
            .field("ore", &self.ore())
            .finish()
    }
}

#[derive(Debug)]
struct Blueprint {
    number: usize,
    ore_robot_cost: Tally,
    clay_robot_cost: Tally,
    obsidian_robot_cost: Tally,
    geode_robot_cost: Tally,
}
impl Blueprint {
    fn add_if_possible(
        &self,
        rtype: RType,
        rock_count: Tally,
        result: &mut Vec<(Tally, Tally)>,
    ) -> bool {
        let cost = self.cost_of(&rtype);
        if cost.all_lte(&rock_count) {
            result.push((Tally::one_of(&rtype), cost));
            return true;
        }
        false
    }

    fn cost_of(&self, rtype: &RType) -> Tally {
        match rtype {
            RType::GEODE => self.geode_robot_cost,
            RType::OBSIDIAN => self.obsidian_robot_cost,
            RType::CLAY => self.clay_robot_cost,
            RType::ORE => self.ore_robot_cost,
        }
    }
}
fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    tuple((
        tag("Blueprint "),
        nom::character::complete::u16,
        tag(": Each ore robot costs "),
        nom::character::complete::u16,
        tag(" ore. Each clay robot costs "),
        nom::character::complete::u16,
        tag(" ore. Each obsidian robot costs "),
        nom::character::complete::u16,
        tag(" ore and "),
        nom::character::complete::u16,
        tag(" clay. Each geode robot costs "),
        nom::character::complete::u16,
        tag(" ore and "),
        nom::character::complete::u16,
        tag(" obsidian."),
    ))(input)
    .map(|(input, parts)| {
        (
            input,
            Blueprint {
                number: parts.1 as usize,
                ore_robot_cost: Tally::new(0, 0, 0, parts.3),
                clay_robot_cost: Tally::new(0, 0, 0, parts.5),
                obsidian_robot_cost: Tally::new(0, 0, parts.9, parts.7),
                geode_robot_cost: Tally::new(0, parts.13, 0, parts.11),
            },
        )
    })
}

fn get_input(input_type: InputType) -> Result<Vec<Blueprint>, AocError> {
    let input = input_type.get_input_as_single_string(19)?;
    let result = match separated_list1(tag("\n"), parse_blueprint)(&input) {
        Ok((remaining, parsed)) => {
            if !remaining.is_empty() {
                panic!(
                    "Unable to parse entire input.\nremaining={}\nparsed={:?}",
                    remaining, parsed
                );
            }
            Ok(parsed)
        }
        Err(e) => panic!("Unable to parse entire {:?}", e),
    };
    result
}

struct BlueprintTargets {
    num_ore_robots: u16,
    num_clay_robots: u16,
    num_obsidian_robots: u16,
}
impl BlueprintTargets {
    fn new(blueprint: &Blueprint) -> Self {
        // How many  robots of each type are needed to be able to generate, in one minute, the
        // number of rocks required to build/buy any robot we want?
        Self {
            num_ore_robots: blueprint
                .ore_robot_cost
                .ore()
                .max(blueprint.clay_robot_cost.ore())
                .max(blueprint.obsidian_robot_cost.ore())
                .max(blueprint.geode_robot_cost.ore()),
            num_clay_robots: blueprint
                .ore_robot_cost
                .clay()
                .max(blueprint.clay_robot_cost.clay())
                .max(blueprint.obsidian_robot_cost.clay())
                .max(blueprint.geode_robot_cost.clay()),
            num_obsidian_robots: blueprint
                .ore_robot_cost
                .obsidian()
                .max(blueprint.clay_robot_cost.obsidian())
                .max(blueprint.obsidian_robot_cost.obsidian())
                .max(blueprint.geode_robot_cost.obsidian()),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        common::InputType,
        day19::{compute, get_input, sum_quality},
    };

    use super::Tally;

    #[test]
    fn test_tally() {
        assert_eq!(Tally::new(0, 0, 0, 1).0, 0x0000_0000_0000_0001);
        assert_eq!(Tally::new(0, 0, 1, 0).0, 0x0000_0000_0001_0000);
        assert_eq!(Tally::new(0, 1, 0, 0).0, 0x0000_0001_0000_0000);
        assert_eq!(Tally::new(1, 0, 0, 0).0, 0x0001_0000_0000_0000);

        let a = Tally::new(9123, 1024, 32973, 23);
        assert_eq!(a.ore(), 23);
        assert_eq!(a.clay(), 32973);
        assert_eq!(a.obsidian(), 1024);
        assert_eq!(a.geode(), 9123);
    }

    #[test]
    fn test_tally_sum_whole() {
        let t = Tally(
            Tally::new(0, 0, 0, 1).0
                + Tally::new(0, 0, 1, 0).0
                + Tally::new(0, 1, 0, 0).0
                + Tally::new(1, 0, 0, 0).0,
        );
        assert_eq!(t.0, 0x0001_0001_0001_0001);
    }

    #[test]
    fn test_tally_add() {
        let t = Tally::new(0, 0, 0, 1)
            + Tally::new(0, 0, 2, 0)
            + Tally::new(0, 3, 0, 0)
            + Tally::new(4, 0, 0, 0);
        assert_eq!(t, Tally::new(4, 3, 2, 1));
    }

    #[test]
    fn test_tally_add_assign() {
        let mut t = Tally::new(0, 0, 0, 0);
        t += Tally::new(0, 0, 0, 10);
        assert_eq!(t, Tally::new(0, 0, 0, 10));
        t += Tally::new(0, 0, 11, 0);
        assert_eq!(t, Tally::new(0, 0, 11, 10));
        t += Tally::new(0, 12, 0, 0);
        assert_eq!(t, Tally::new(0, 12, 11, 10));
        t += Tally::new(13, 0, 0, 0);
        assert_eq!(t, Tally::new(13, 12, 11, 10));
    }

    #[test]
    fn test_tally_all_lte() {
        assert_eq!(
            Tally::new(0, 0, 0, 3).all_lte(&Tally::new(0, 7, 0, 2)),
            false
        );
        assert_eq!(
            Tally::new(0, 8, 0, 3).all_lte(&Tally::new(0, 20, 0, 10)),
            true
        );
        assert_eq!(
            Tally::new(0, 8, 0, 3).all_lte(&Tally::new(0, 20, 0, 10)),
            true
        );
        assert_eq!(
            Tally::new(1, 0, 1, 1).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(1, 0, 0, 0).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(0, 1, 0, 0).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(0, 0, 1, 0).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(0, 0, 0, 1).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );

        assert_eq!(
            Tally::new(0, 0, 0, 3).all_lte(&Tally::new(0, 7, 0, 2)),
            false
        );
        assert_eq!(
            Tally::new(0, 8, 0, 3).all_lte(&Tally::new(0, 20, 0, 10)),
            true
        );
        assert_eq!(
            Tally::new(1, 0, 1, 1).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(1, 0, 0, 0).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(0, 1, 0, 0).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(0, 0, 1, 0).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(0, 0, 0, 1).all_lte(&Tally::new(0, 0, 0, 0)),
            false
        );
        assert_eq!(
            Tally::new(0, 0, 14, 3).all_lte(&Tally::new(0, 0, 0, 4)),
            false
        );
    }

    #[test]
    fn test_parse_challenge_data() {
        let input = get_input(InputType::Challenge).unwrap();
        assert_eq!(input.len(), 30);
    }

    #[test]
    fn test_part1_example() {
        let input = get_input(InputType::Example).unwrap();
        assert_eq!(sum_quality(24, &input), 33);
    }

    #[test]
    fn test_part2_example() {
        let input = get_input(InputType::Example).unwrap();
        assert_eq!(compute(32, &input[0]).geode(), 56);
        assert_eq!(compute(32, &input[1]).geode(), 62);
    }
}
