use crate::AocError;
use crate::DailyInput;

use super::{Inputs, Mappings};

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let inputs = Inputs::from(&lines);
    let mappings = Mappings::from(&inputs);
    let locations = mappings.map_many(&inputs.seeds);
    let min = locations.iter().min().unwrap();
    Ok(min.to_string())
}
