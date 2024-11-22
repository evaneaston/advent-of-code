use std::{
    collections::BTreeSet,
    env,
    io::{stdout, Write},
    time::{Duration, Instant},
};

use regex::Regex;

use aoc2024::{AocError, DailyInput, DayPartFn, enable_logging, get_day_parts, InputType};

/// cargo run -- day
/// cargo run -- day,1
/// cargo run -- day,2
fn main() -> Result<(), AocError> {
    enable_logging()?;

    let start_all_time = Instant::now();
    for DayPartFn { day, part, function } in find_parts_to_run() {
        print!("[Day {:2} Part {:2}]", day, part);
        stdout().flush()?;
        let start_time = Instant::now();

        let result = function(DailyInput {
            day,
            input_type: InputType::Challenge,
            number: None,
        })?;

        let day_part_duration: Duration = Instant::now() - start_time;
        println!(" in {} = {}", format_duration(&day_part_duration), result);
    }
    let total_duration: Duration = Instant::now() - start_all_time;
    println!("Total time: {}", format_duration(&total_duration));
    Ok(())
}

fn format_duration(duration: &Duration) -> String {
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    let microseconds = duration.subsec_micros();
    format!("{:2}m {:2}.{:06}s", minutes, seconds, microseconds)
}

fn find_parts_to_run() -> Vec<DayPartFn> {
    let all_day_parts = get_day_parts();

    let args = env::args().collect::<Vec<_>>();
    let args = args.split_first().unwrap().1;

    if args.iter().any(|a| a == "all") {
        all_day_parts.into_iter().collect()
    } else {
        if args.is_empty() {
            panic!("No arguments")
        }

        let re = Regex::new(r"^([1-9]|1[0-9]|2[0-5])(,([12]))?$").unwrap();
        let day_and_part_nums = args
            .iter()
            .enumerate()
            .flat_map(|(i, arg)| {
                match re.captures(arg) {
                    Some(c) => {
                        let day = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        if let Some(part) = c.get(3) {
                            vec![(day, part.as_str().parse::<usize>().unwrap())]
                        } else {
                            vec![(day, 1), (day, 2)]
                        }
                    }
                    None => panic!("Invalid arg {} at at position {}.  Expected to be either a day from 1-25 or a day from 1-25 followed by a command and a part from 1-2", arg, i),
                }
            })
            .collect::<BTreeSet<_>>();

        all_day_parts
            .into_iter()
            .filter(|dp| day_and_part_nums.contains(&(dp.day, dp.part)))
            .collect::<Vec<_>>()
    }
}
