use std::collections::HashMap;

use crate::AocError;
use crate::DailyInput;
use super::input;

pub(crate) struct Mapper {
    pub(crate) numeric_mappings: Vec<input::NumericMappings>,
}

impl Mapper {
    pub(crate) fn map(&self, input: i64) -> i64 {
        for mapping in &self.numeric_mappings {
            let mapped = mapping.map(input);
            if let Some(result) = mapped {
                return result;
            }
        }
        input
    }
}

pub(crate) fn map(input: &[i64], from: &str, to: &str, maps: &HashMap<input::StageMapping, Vec<input::NumericMappings>>) -> Vec<i64> {
    let mapper = get_mapper_for(maps, from, to);

    let mut mapped = vec![0; input.len()];
    for i in 0..mapped.len() {
        match mapped.get_mut(i) {
            Some(v) => *v = mapper.map(*input.get(i).unwrap()),
            None => panic!(),
        }
    }

    mapped
}

pub(crate) fn get_mapper_for<'a>(maps: &'a HashMap<input::StageMapping, Vec<input::NumericMappings>>, from: &'a str, to: &'a str) -> Mapper {
    let mappings = maps
        .get(&input::StageMapping {
            from: String::from(from),
            to: String::from(to),
        })
        .unwrap();
    Mapper {
        numeric_mappings: mappings.to_vec(),
    }
}

pub(crate) fn apply_all_mappings(seeds: &[i64], maps: &HashMap<input::StageMapping, Vec<input::NumericMappings>>) -> Vec<i64> {
    let soil = map(seeds, "seed", "soil", maps);
    let fertilizer = map(&soil, "soil", "fertilizer", maps);
    let water = map(&fertilizer, "fertilizer", "water", maps);
    let light = map(&water, "water", "light", maps);
    let temperature = map(&light, "light", "temperature", maps);
    let humidity = map(&temperature, "temperature", "humidity", maps);
    map(&humidity, "humidity", "location", maps)
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let model = input::lines_to_inputs(&lines);
    let locations = apply_all_mappings(&model.seeds, &model.maps);
    let min = locations.iter().min().unwrap();
    Ok(min.to_string())
}
