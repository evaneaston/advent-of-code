use crate::{day06::parse::parse_day06_input, AocError, DailyInput};

mod parse;
#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Race {
    race_duration: u64,
    record_distance: u64,
}

pub(crate) fn comp_dist(race_duration: u64, button_hold_millis: u64) -> u64 {
    button_hold_millis * (race_duration - button_hold_millis)
}

fn num_ways_to_beat_record(race: &Race) -> u64 {
    (0_u64..=race.race_duration)
        .map(|button_hold_millis| comp_dist(race.race_duration, button_hold_millis))
        .filter(|distance| *distance > race.record_distance)
        .count() as u64
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let races: Vec<Race> = parse_day06_input(input)?;

    let n = races
        .iter()
        .map(num_ways_to_beat_record)
        .filter(|n| *n > 0)
        .collect::<Vec<_>>();
    let num_ways_to_win_product: u64 = n.iter().product();
    Ok(num_ways_to_win_product.to_string())
}

fn parse_part2_line(expected_tag: &str, line: &str) -> u64 {
    let binding = line.to_string().replace(' ', "");
    let v: Vec<_> = binding.split(':').collect();
    if v[0] != expected_tag {
        panic!("First line expected to be Time ({})", line);
    }
    v[1].parse::<u64>().unwrap()
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;

    let race = Race {
        race_duration: parse_part2_line("Time", &lines[0]),
        record_distance: parse_part2_line("Distance", &lines[1]),
    };

    let answer = num_ways_to_beat_record(&race);

    Ok(answer.to_string())
}
