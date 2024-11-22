use crate::common::{AocError, InputType};
use std::collections::HashSet;

pub fn part1() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(6)?;
    let line = lines.get(0).unwrap();
    Ok(find_marker_of_length(&line, 4))
}

pub fn part2() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(6)?;
    let line = lines.get(0).unwrap();
    Ok(find_marker_of_length(&line, 14))
}

fn find_marker_of_length(line: &str, length: usize) -> String {
    for n in length..line.len() {
        let slice = &line[n - length..n];

        let mut set = HashSet::<char>::with_capacity(length);

        for c in slice.chars() {
            set.insert(c);
        }
        if set.len() == length {
            return format!("position={} slice={} length={}", n, slice, set.len());
            // break;
        }
    }
    return "".to_string();
}

#[cfg(test)]
mod tests {}
