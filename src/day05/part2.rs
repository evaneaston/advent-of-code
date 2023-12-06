use super::input::NumericMappings;
use super::input::StageMapping;
use super::input::lines_to_inputs;
use super::part1::Mapper;
use super::part1::get_mapper_for;
use crate::AocError;
use crate::DailyInput;
use std::collections::HashMap;

pub(crate) struct SeedIterator {
    pub(crate) seed_to_soil: Mapper,
    pub(crate) soil_to_fertilizer: Mapper,
    pub(crate) fertilizer_to_water: Mapper,
    pub(crate) water_to_light: Mapper,
    pub(crate) light_to_temperature: Mapper,
    pub(crate) temperature_to_humidity: Mapper,
    pub(crate) humidity_to_location: Mapper,
}

impl SeedIterator {
    pub fn new(maps: &HashMap<StageMapping, Vec<NumericMappings>>) -> Self {
        SeedIterator {
            seed_to_soil: get_mapper_for(maps, "seed", "soil"),
            soil_to_fertilizer: get_mapper_for(maps, "soil", "fertilizer"),
            fertilizer_to_water: get_mapper_for(maps, "fertilizer", "water"),
            water_to_light: get_mapper_for(maps, "water", "light"),
            light_to_temperature: get_mapper_for(maps, "light", "temperature"),
            temperature_to_humidity: get_mapper_for(maps, "temperature", "humidity"),
            humidity_to_location: get_mapper_for(maps, "humidity", "location"),
        }
    }

    pub fn map(&self, value: i64) -> i64 {
        let mapped = self.seed_to_soil.map(value);
        let mapped = self.soil_to_fertilizer.map(mapped);
        let mapped = self.fertilizer_to_water.map(mapped);
        let mapped = self.water_to_light.map(mapped);
        let mapped = self.light_to_temperature.map(mapped);
        let mapped = self.temperature_to_humidity.map(mapped);
        self.humidity_to_location.map(mapped)
    }
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let model = lines_to_inputs(&lines);

    //let num_seeds = model.seeds.chunks(2).map(|vals| vals[1]).sum()
    let seeds = model.seeds.chunks(2).flat_map(|vals| vals[0]..(vals[0] + vals[1]));

    let mut min: i64 = i64::MAX;
    let the_mapper = SeedIterator::new(&model.maps);
    for seed in seeds {
        let location = the_mapper.map(seed);
        if location < min {
            min = location;
        }
    }
    Ok(min.to_string())
}
