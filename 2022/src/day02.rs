use log::debug;
use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{Error, ErrorKind},
    str::FromStr,
};

use crate::common::{AocError, InputType};

#[derive(Debug, Eq)]
pub enum Play {
    ROCK,
    PAPER,
    SCISSORS,
}
impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match *self {
            Play::ROCK => match other {
                Play::ROCK => Some(Ordering::Equal),
                Play::PAPER => Some(Ordering::Less),
                Play::SCISSORS => Some(Ordering::Greater),
            },
            Play::PAPER => match other {
                Play::ROCK => Some(Ordering::Greater),
                Play::PAPER => Some(Ordering::Equal),
                Play::SCISSORS => Some(Ordering::Less),
            },
            Play::SCISSORS => match other {
                Play::ROCK => Some(Ordering::Less),
                Play::PAPER => Some(Ordering::Greater),
                Play::SCISSORS => Some(Ordering::Equal),
            },
        }
    }
}
impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Play {
    pub fn score_against(&self, opponent: &Play) -> u32 {
        let me_base = match self {
            Play::ROCK => 1,
            Play::PAPER => 2,
            Play::SCISSORS => 3,
        };

        let me_additional = match self.cmp(opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };

        me_base + me_additional
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum EncodedPlay {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

impl FromStr for EncodedPlay {
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "A" => Ok(EncodedPlay::A),
            "B" => Ok(EncodedPlay::B),
            "C" => Ok(EncodedPlay::C),
            "X" => Ok(EncodedPlay::X),
            "Y" => Ok(EncodedPlay::Y),
            "Z" => Ok(EncodedPlay::Z),
            _ => Err(Error::new(ErrorKind::Other, format!("invalid play{}", s))),
        }
    }
    type Err = std::io::Error;
}
impl EncodedPlay {
    fn score_against(
        &self,
        encoded_other: &EncodedPlay,
        scheme: &HashMap<EncodedPlay, Play>,
    ) -> u32 {
        let self_play = scheme.get(self).unwrap();
        let other_play = scheme.get(encoded_other).unwrap();
        self_play.score_against(other_play)
    }
}

struct Guide {
    turns: Vec<(EncodedPlay, EncodedPlay)>,
}

impl Guide {
    fn parse_guide() -> Result<Guide, std::io::Error> {
        let lines = InputType::Challenge.get_input_lines(2)?;

        let mut turns = Vec::<(EncodedPlay, EncodedPlay)>::new();
        let mut line_number = 0;
        for line in lines {
            line_number += 1;
            if !line.is_empty() {
                let turn = match parse_guide_line(&line) {
                    Ok(t) => t,
                    Err(e) => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Error on line {}: {:?}", line_number, e),
                        ))
                    }
                };

                turns.push(turn);
            }
        }
        Ok(Guide { turns })
    }

    fn scores_for_scheme(&self, scheme: &HashMap<EncodedPlay, Play>) -> (u32, u32) {
        let mut left_total_score: u32 = 0;
        let mut right_total_score: u32 = 0;

        for turn in self.turns.iter() {
            let left_score = turn.0.score_against(&turn.1, scheme);
            let right_score: u32 = turn.1.score_against(&turn.0, scheme);

            left_total_score += left_score;
            right_total_score += right_score;

            debug!(
                "{:?} => {:?}  vs {:?} => {:?}  Score  {} vs {}",
                turn.0,
                scheme.get(&turn.0).unwrap(),
                turn.1,
                scheme.get(&turn.1).unwrap(),
                left_score,
                right_score
            );
        }
        // debug!(
        //     "Scores for scheme {:?} left={} right={} winner={}",
        //     scheme,
        //     left_total_score,
        //     right_total_score,
        //     if left_total_score > right_total_score {
        //         "LEFT"
        //     } else if right_total_score > left_total_score {
        //         "RIGHT"
        //     } else {
        //         "TIED"
        //     }
        // );
        (left_total_score, right_total_score)
    }

    fn scores_for_scheme2(&self, scheme: &HashMap<EncodedPlay, Play>) -> (u32, u32) {
        let mut left_total_score: u32 = 0;
        let mut right_total_score: u32 = 0;

        for turn in self.turns.iter() {
            let left_play = scheme.get(&turn.0).unwrap();
            let right_play = match turn.1 {
                EncodedPlay::X => {
                    // must lose
                    match left_play {
                        Play::ROCK => &Play::SCISSORS,
                        Play::PAPER => &Play::ROCK,
                        Play::SCISSORS => &Play::PAPER,
                    }
                }
                EncodedPlay::Y => left_play,
                EncodedPlay::Z => {
                    // must win
                    match left_play {
                        Play::ROCK => &&Play::PAPER,
                        Play::PAPER => &&Play::SCISSORS,
                        Play::SCISSORS => &&Play::ROCK,
                    }
                }
                _ => panic!("Shouldn't happen"),
            };

            let left_score = left_play.score_against(right_play);
            let right_score: u32 = right_play.score_against(left_play);

            left_total_score += left_score;
            right_total_score += right_score;
        }
        debug!(
            "Scores for scheme {:?} left={} right={} winner={}",
            scheme,
            left_total_score,
            right_total_score,
            if left_total_score > right_total_score {
                "LEFT"
            } else if right_total_score > left_total_score {
                "RIGHT"
            } else {
                "TIED"
            }
        );
        (left_total_score, right_total_score)
    }
}

fn parse_guide_line(line: &str) -> Result<(EncodedPlay, EncodedPlay), Error> {
    let parts = line.split(" ").collect::<Vec<&str>>();
    assert!(parts.len() >= 2);
    let opponents_play = EncodedPlay::from_str(parts[0])?;
    let response = EncodedPlay::from_str(parts[1])?;
    Ok((opponents_play, response))
}

pub fn part1() -> Result<String, AocError> {
    let guide = Guide::parse_guide()?;
    let result = guide
        .scores_for_scheme(&HashMap::from([
            (EncodedPlay::A, Play::ROCK),
            (EncodedPlay::B, Play::PAPER),
            (EncodedPlay::C, Play::SCISSORS),
            (EncodedPlay::X, Play::ROCK),
            (EncodedPlay::Y, Play::PAPER),
            (EncodedPlay::Z, Play::SCISSORS),
        ]))
        .1;
    Ok(format!("{}", result))
}

pub fn part2() -> Result<String, AocError> {
    let guide = Guide::parse_guide()?;
    let result = guide
        .scores_for_scheme2(&HashMap::from([
            (EncodedPlay::A, Play::ROCK),
            (EncodedPlay::B, Play::PAPER),
            (EncodedPlay::C, Play::SCISSORS),
            (EncodedPlay::X, Play::ROCK),
            (EncodedPlay::Y, Play::PAPER),
            (EncodedPlay::Z, Play::SCISSORS),
        ]))
        .1;

    Ok(format!("{}", result))
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, collections::HashMap};

    use log::debug;

    use crate::day02::{EncodedPlay, Guide, Play};

    #[test]
    fn test_cmp() {
        debug!("{:?}", Play::SCISSORS.cmp(&Play::ROCK));

        assert_eq!(Play::ROCK.cmp(&Play::ROCK), Ordering::Equal);
        assert_eq!(Play::ROCK.cmp(&Play::PAPER), Ordering::Less);
        assert_eq!(Play::ROCK.cmp(&Play::SCISSORS), Ordering::Greater);

        assert_eq!(Play::PAPER.cmp(&Play::ROCK), Ordering::Greater);
        assert_eq!(Play::PAPER.cmp(&Play::PAPER), Ordering::Equal);
        assert_eq!(Play::PAPER.cmp(&Play::SCISSORS), Ordering::Less);

        assert_eq!(Play::SCISSORS.cmp(&Play::ROCK), Ordering::Less);
        assert_eq!(Play::SCISSORS.cmp(&Play::PAPER), Ordering::Greater);
        assert_eq!(Play::SCISSORS.cmp(&Play::SCISSORS), Ordering::Equal);
    }

    #[test]
    fn test_score() {
        assert_eq!(Play::ROCK.score_against(&Play::SCISSORS), 7);
        assert_eq!(Play::PAPER.score_against(&Play::ROCK), 8);
        assert_eq!(Play::SCISSORS.score_against(&Play::PAPER), 9);
        assert_eq!(Play::ROCK.score_against(&Play::PAPER), 1);
        assert_eq!(Play::PAPER.score_against(&Play::SCISSORS), 2);
        assert_eq!(Play::SCISSORS.score_against(&Play::ROCK), 3);
        assert_eq!(Play::ROCK.score_against(&Play::ROCK), 4);
        assert_eq!(Play::PAPER.score_against(&Play::PAPER), 5);
        assert_eq!(Play::SCISSORS.score_against(&Play::SCISSORS), 6);
    }

    #[test]
    fn test_example_guide() {
        let example_guide = Guide {
            turns: vec![
                (EncodedPlay::A, EncodedPlay::Y),
                (EncodedPlay::B, EncodedPlay::X),
                (EncodedPlay::C, EncodedPlay::Z),
            ],
        };
        let scores = example_guide.scores_for_scheme(&HashMap::from([
            (EncodedPlay::A, Play::ROCK),
            (EncodedPlay::B, Play::PAPER),
            (EncodedPlay::C, Play::SCISSORS),
            (EncodedPlay::X, Play::ROCK),
            (EncodedPlay::Y, Play::PAPER),
            (EncodedPlay::Z, Play::SCISSORS),
        ]));
        assert_eq!(scores.1, 15);
    }
}
