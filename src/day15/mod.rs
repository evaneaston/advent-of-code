use std::fmt::Display;

use lazy_static::lazy_static;
use log::debug;
use regex::Regex;

use crate::{AocError, DailyInput};

lazy_static! {
    static ref EQUAL_RE: Regex = Regex::new(r"^(\w+)=(\d+)$").unwrap();
    static ref DASH_RE: Regex = Regex::new(r"^(\w+)-$").unwrap();
}

fn hash(s: &str) -> u8 {
    assert!(s.is_ascii());
    s.chars()
        .map(|c| c as u8)
        .fold(0_u64, |hash, c| ((hash + c as u64) * 17) % 256) as u8
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let inputs = load(input)?;
    debug!("{:?}", inputs);
    let sum: u64 = inputs
        .iter()
        .map(|i| {
            let h = hash(i.as_str());
            debug!("  Hash of {i}: {h}");
            h as u64
        })
        .sum();
    Ok(sum.to_string())
}

#[derive(Debug, Hash, PartialOrd, Ord, Eq, PartialEq, Clone)]
struct LensKey {
    label: String,
    hash: u8,
}
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Lens {
    key: LensKey,
    focal_length: u8,
}
impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.key.label, self.focal_length)
    }
}

#[derive(Debug)]
enum Step {
    Equal(LensKey, u8), // last is focal length
    Dash(LensKey),
}
impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Equal(k, fl) => write!(f, "{}={}", k.label, fl),
            Step::Dash(k) => write!(f, "{}-", k.label),
        }
    }
}
impl From<&str> for Step {
    fn from(value: &str) -> Self {
        if let Some(cap) = EQUAL_RE.captures(value) {
            let label = cap.get(1).unwrap().as_str();
            let hash = hash(label);
            let focal_length = cap.get(2).unwrap().as_str().parse::<u8>().unwrap();
            return Step::Equal(
                LensKey {
                    label: String::from(label),
                    hash,
                },
                focal_length,
            );
        }
        if let Some(cap) = DASH_RE.captures(value) {
            let label = cap.get(1).unwrap().as_str();
            let hash = hash(label);
            return Step::Dash(LensKey {
                label: String::from(label),
                hash,
            });
        }
        panic!("Invalid format {}", value);
    }
}

struct Boxes(Vec<Vec<Lens>>);
impl Boxes {
    fn add_to_box(&mut self, lens_key: &LensKey, focal_length: u8) {
        let bx: &mut Vec<Lens> = self.0.get_mut(lens_key.hash as usize).unwrap();

        if let Some(index) = bx.iter().position(|l| l.key.label == lens_key.label) {
            bx.get_mut(index).unwrap().focal_length = focal_length;
        } else {
            bx.push(Lens {
                key: lens_key.clone(),
                focal_length,
            });
        }
    }
    fn remove_from_box(&mut self, lens_key: &LensKey) {
        let bx = self.0.get_mut(lens_key.hash as usize).unwrap();
        bx.retain(|lens| lens.key.label != lens_key.label);
    }
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let inputs = load(input)?;
    let steps = inputs.iter().map(|s| Step::from(s.as_str())).collect::<Vec<Step>>();

    // print!("Steps=");
    // for s in &steps {
    //     print!("{} ", s);
    // }
    // println!();

    let mut boxes = Boxes((0..256).map(|_| Vec::<Lens>::new()).collect::<Vec<_>>());

    for step in &steps {
        //println!(" Step {}", step);
        match step {
            Step::Equal(lens_key, focal_length) => boxes.add_to_box(lens_key, *focal_length),
            Step::Dash(lens_key) => boxes.remove_from_box(lens_key),
        };
        for (_index, bx) in boxes.0.iter().enumerate() {
            if bx.is_empty() {
                continue;
            }
            // print!("    Box {} ", index);
            // for l in bx {
            //     print!("{} ", l);
            // }
            // println!();
        }
    }

    let sum = boxes
        .0
        .iter()
        .enumerate()
        .map(|(index, bx)| {
            let box_num = index + 1;
            bx.iter()
                .enumerate()
                .map(|(index, lens)| {
                    let slot_num = index + 1;
                    lens.focal_length as usize * slot_num * box_num
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(sum.to_string())
}

pub(crate) fn load(input: DailyInput) -> Result<Vec<String>, AocError> {
    let inputs = input
        .get_input_as_single_string()?
        .split(',')
        .map(String::from)
        .collect::<Vec<_>>();
    Ok(inputs)
}

#[cfg(test)]
mod test {
    use crate::{
        day15::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 15,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "1320"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 15,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "494980"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 15,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "145"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 15,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "247933"
        );
    }
}
