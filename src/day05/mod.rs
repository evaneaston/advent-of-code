pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests;

pub use part1::part1;
//pub use part2::part2_ as part2;
pub use part2::part2;

use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    ops::RangeInclusive,
};

#[derive(Clone, Debug)]
pub(crate) struct Inputs {
    pub(crate) seeds: Vec<i64>,
    pub(crate) maps: HashMap<StageMapping, Vec<NumericMapping>>,
}

impl Inputs {
    pub(crate) fn get_mapper_for<'a>(&self, from: &'a str, to: &'a str) -> Mapper {
        let stages = StageMapping {
            from: String::from(from),
            to: String::from(to),
        };
        let mappings = self
            .maps
            .get(&stages)
            .expect(format!("Expect to have a mapping from '{}' to '{}'", from, to).as_str());
        Mapper {
            stages,
            numeric_mappings: mappings.to_vec(),
        }
    }
}

impl From<&Vec<String>> for Inputs {
    fn from(lines: &Vec<String>) -> Self {
        assert!(lines[0].starts_with("seeds:"));
        let list_re = Regex::new(r"(\d+)").unwrap();
        let seeds = list_re.find_iter(&lines[0]).map(|n| n.as_str().parse::<i64>().unwrap()).collect::<Vec<_>>();

        let map_key_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
        let map_values_re = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();

        let mut maps = HashMap::<StageMapping, Vec<NumericMapping>>::new();
        let mut current_mapping_key: Option<StageMapping> = None;
        let mut current_numeric_mappings: Option<Vec<NumericMapping>> = Some(vec![]);

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
                    Some(v) => v.push(NumericMapping::new(
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

        Self { seeds, maps }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct NumericMapping {
    inlet_range: RangeInclusive<i64>,
    outlet_range: RangeInclusive<i64>,
    offset: i64,
}

impl NumericMapping {
    pub fn new(to: i64, from: i64, size: i64) -> Self {
        let offset = to - from;
        Self {
            inlet_range: from..=(from + size - 1),
            outlet_range: (from + offset)..=(from + size - 1 + offset),
            offset,
        }
    }
    pub fn map(&self, value: i64) -> Option<i64> {
        if self.inlet_range.contains(&value) {
            Some(value + self.offset)
        } else {
            None
        }
    }
    pub fn map_reverse(&self, value: i64) -> Option<i64> {
        if self.outlet_range.contains(&value) {
            Some(value - self.offset)
        } else {
            None
        }
    }
    pub fn inlet_edge_points(&self) -> Vec<i64> {
        vec![*self.inlet_range.start(), *self.inlet_range.end()]
    }
    pub fn outlet_edge_points(&self) -> Vec<i64> {
        vec![*self.outlet_range.start(), *self.outlet_range.end()]
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct StageMapping {
    pub from: String,
    pub to: String,
}

pub(crate) struct Mapper {
    #[allow(dead_code)]
    stages: StageMapping,
    numeric_mappings: Vec<NumericMapping>,
}

impl Mapper {
    pub(crate) fn new(stages: StageMapping, numeric_mappings: Vec<NumericMapping>) -> Self {
        Self {
            stages,
            numeric_mappings,
        }
    }
    pub(crate) fn map(&self, input: i64) -> i64 {
        for mapping in &self.numeric_mappings {
            let mapped = mapping.map(input);
            if let Some(result) = mapped {
                return result;
            }
        }
        input
    }
    pub(crate) fn map_reverse(&self, input: i64) -> i64 {
        for mapping in &self.numeric_mappings {
            let mapped = mapping.map_reverse(input);
            if let Some(result) = mapped {
                return result;
            }
        }
        input
    }

    pub(crate) fn inlet_edge_points(&self) -> BTreeSet<i64> {
        self.numeric_mappings.iter().flat_map(|nm| nm.inlet_edge_points()).collect()
    }
    pub(crate) fn outlet_edge_points(&self) -> BTreeSet<i64> {
        self.numeric_mappings.iter().flat_map(|nm| nm.outlet_edge_points()).collect()
    }

    fn outlet_ranges(&self) -> Vec<RangeInclusive<i64>> {
        self.numeric_mappings.iter().map(|nr| nr.outlet_range.clone()).collect()
    }
}

pub(crate) struct Mappings {
    mappers: Vec<Mapper>,
}

impl Mappings {
    pub(crate) fn map_many(&self, input: &[i64]) -> Vec<i64> {
        input.iter().map(|v| self.map(*v)).collect()
    }

    pub(crate) fn map(&self, value: i64) -> i64 {
        self.mappers.iter().fold(value, |value, mapper| mapper.map(value))
    }

    pub(crate) fn map_reverse(&self, value: i64) -> i64 {
        self.mappers.iter().rev().fold(value, |value, mapper| mapper.map_reverse(value))
    }

    pub(crate) fn inlet_edge_points(&self) -> BTreeSet<i64> {
        self.mappers.iter().flat_map(|nms| nms.inlet_edge_points()).collect()
    }

    pub(crate) fn outlet_edge_points(&self) -> BTreeSet<i64> {
        self.mappers.iter().flat_map(|nms| nms.outlet_edge_points()).collect()
    }

    pub(crate) fn outlet_ranges(&self) -> Vec<RangeInclusive<i64>> {
        self.mappers.iter().flat_map(|nms| nms.outlet_ranges()).collect()
    }

}

impl From<&Inputs> for Mappings {
    fn from(inputs: &Inputs) -> Self {
        let stages = [
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ];
        Self {
            mappers: stages.windows(2).map(|pair| inputs.get_mapper_for(pair[0], pair[1])).collect(),
        }
    }
}
