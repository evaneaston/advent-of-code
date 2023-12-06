use regex::Regex;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct Inputs {
    pub(crate) seeds: Vec<i64>,
    pub(crate) maps: HashMap<StageMapping, Vec<NumericMappings>>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct StageMapping {
    pub from: String,
    pub to: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct NumericMappings {
    pub(crate) range: core::ops::Range<i64>,
    pub(crate) offset: i64,
}

impl NumericMappings {
    pub fn new(to: i64, from: i64, size: i64) -> Self {
        Self {
            range: from..(from + size),
            offset: to - from,
        }
    }
    pub fn map(&self, value: i64) -> Option<i64> {
        if self.range.contains(&value) {
            Some(value + self.offset)
        } else {
            None
        }
    }
}

pub(crate) fn lines_to_inputs(lines: &[String]) -> Inputs {
    assert!(lines[0].starts_with("seeds:"));
    let list_re = Regex::new(r"(\d+)").unwrap();
    let seeds = list_re
        .find_iter(&lines[0])
        .map(|n| n.as_str().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let map_key_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let map_values_re = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();

    let mut maps = HashMap::<StageMapping, Vec<NumericMappings>>::new();
    let mut current_mapping_key: Option<StageMapping> = None;
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
            current_mapping_key.replace(StageMapping {
                from: from.to_string(),
                to: to.to_string(),
            });
        } else if let Some(captures) = map_values_re.captures(line) {
            match current_numeric_mappings.as_mut() {
                Some(v) => v.push(NumericMappings::new(
                    captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                    captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                )),
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

    Inputs { seeds, maps }
}
