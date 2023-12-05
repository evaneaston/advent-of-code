use regex::Regex;
use std::collections::HashMap;

use crate::{AocError, DailyInput};

struct Model {
    seeds: Vec<u64>,
    maps: HashMap<TypeMapping, Vec<NumericMappings>>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct TypeMapping {
    pub from: String,
    pub to: String,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct NumericMappings {
    pub to: u64,
    pub from: u64,
    pub size: u64,
}
impl NumericMappings {
    pub fn convert(&self, value: u64) -> Option<u64> {
        if self.from <= value && value < self.from + self.size {
            Some(value - self.from + self.to)
        } else {
            None
        }
    }
}

fn lines_to_model(lines: &[String]) -> Model {
    assert!(lines[0].starts_with("seeds:"));
    let list_re = Regex::new(r"(\d+)").unwrap();
    let seeds = list_re
        .find_iter(&lines[0])
        .map(|n| n.as_str().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let map_key_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let map_values_re = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();

    let mut maps = HashMap::<TypeMapping, Vec<NumericMappings>>::new();
    let mut current_mapping_key: Option<TypeMapping> = None;
    let mut current_numeric_mappings: Option<Vec<NumericMappings>> = Some(vec![]);

    for line in lines[1..].iter().filter(|l| !l.is_empty()) {
        if let Some(captures) = map_key_re.captures(line) {
            let from = captures.get(1).unwrap().as_str();
            let to = captures.get(2).unwrap().as_str();

            // key has changed
            if current_mapping_key.is_some() {
                let ce = current_numeric_mappings.replace(vec![]).unwrap();
                maps.insert(current_mapping_key.take().unwrap(), ce);
            }
            current_mapping_key.replace(TypeMapping {
                from: from.to_string(),
                to: to.to_string(),
            });
        } else if let Some(captures) = map_values_re.captures(line) {
            match current_numeric_mappings.as_mut() {
                Some(v) => v.push(NumericMappings {
                    from: captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                    to: captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    size: captures.get(3).unwrap().as_str().parse::<u64>().unwrap(),
                }),
                None => panic!("there should always be a current vec"),
            }
        }
    }
    if current_mapping_key.is_some() {
        maps.insert(
            current_mapping_key.take().unwrap(),
            current_numeric_mappings.take().unwrap().clone(),
        );
    }

    Model { seeds, maps }
}

fn map(input: &[u64], from: &str, to: &str, maps: &HashMap<TypeMapping, Vec<NumericMappings>>) -> Vec<u64> {
    let mappings = maps
        .get(&TypeMapping {
            from: String::from(from),
            to: String::from(to),
        })
        .unwrap();

    let mut mapped = vec![0; input.len()];
    for i in 0..mapped.len() {
        let old_val = input.get(i).unwrap();
        let new_val = mappings.iter().find_map(|m| m.convert(*old_val)).unwrap_or(*old_val);

        match mapped.get_mut(i) {
            Some(v) => *v = new_val,
            None => panic!(),
        }
    }

    mapped
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let model = lines_to_model(&lines);

    let soil = map(&model.seeds, "seed", "soil", &model.maps);
    let fertilizer = map(&soil, "soil", "fertilizer", &model.maps);
    let water = map(&fertilizer, "fertilizer", "water", &model.maps);
    let light = map(&water, "water", "light", &model.maps);
    let temperature = map(&light, "light", "temperature", &model.maps);
    let humidity = map(&temperature, "temperature", "humidity", &model.maps);
    let location = map(&humidity, "humidity", "location", &model.maps);

    let min = location.iter().min().unwrap();
    Ok(min.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    input.get_input_as_single_string()?;
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        day05::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 5,
                part: None,
                input_type: InputType::Example
            })
            .unwrap(),
            "35"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 5,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            "251346198"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 5,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            "unw0979"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 5,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            ""
        );
    }
}
