mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod grid;

use common::{AocError, Part, enable_logging};
use std::{collections::BTreeMap, env};

fn main() -> Result<(), AocError> {
    enable_logging()?;

    let day_parts: BTreeMap<(usize, usize), Part> = BTreeMap::from([
        ((1, 1), day01::part1 as Part),
        ((1, 2), day01::part2 as Part),
        ((2, 1), day02::part1 as Part),
        ((2, 2), day02::part2 as Part),
        ((3, 1), day03::part1 as Part),
        ((3, 2), day03::part2 as Part),
        ((4, 1), day04::part1 as Part),
        ((4, 2), day04::part2 as Part),
        ((5, 1), day05::part1 as Part),
        ((5, 2), day05::part2 as Part),
        ((6, 1), day06::part1 as Part),
        ((6, 2), day06::part2 as Part),
        ((7, 1), day07::part1 as Part),
        ((7, 2), day07::part2 as Part),
        ((8, 1), day08::part1 as Part),
        ((8, 2), day08::part2 as Part),
        ((9, 1), day09::part1 as Part),
        ((9, 2), day09::part2 as Part),
        ((10, 1), day10::part1 as Part),
        ((10, 2), day10::part2 as Part),
        ((11, 1), day11::part1 as Part),
        ((11, 2), day11::part2 as Part),
        ((12, 1), day12::part1 as Part),
        ((12, 2), day12::part2 as Part),
        ((13, 1), day13::part1 as Part),
        ((13, 2), day13::part2 as Part),
        ((14, 1), day14::part1 as Part),
        ((14, 2), day14::part2 as Part),
        ((15, 1), day15::part1 as Part),
        ((15, 2), day15::part2 as Part),
        ((16, 1), day16::part1 as Part),
        ((16, 2), day16::part2 as Part),
        ((17, 1), day17::part1 as Part),
        ((17, 2), day17::part2 as Part),
        ((18, 1), day18::part1 as Part),
        ((18, 2), day18::part2 as Part),
        ((19, 1), day19::part1 as Part),
        ((19, 2), day19::part2 as Part),
        ((20, 1), day20::part1 as Part),
        ((20, 2), day20::part2 as Part),
        ((21, 1), day21::part1 as Part),
        ((21, 2), day21::part2 as Part),
        ((22, 1), day22::part1 as Part),
        ((22, 2), day22::part2 as Part),
    ]);

    let mut to_run: Vec<(&(usize, usize), &Part)> = Vec::new();

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "all") {
        to_run.extend(day_parts.iter().map(|d| d));
    } else {
        let mut days = args
            .iter()
            .map(|a| a.parse::<usize>())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();

        if days.is_empty() {
            let latest_day = day_parts.iter().map(|e| (e.0).0).max().unwrap();

            // run latest day part(s)
            days.push(latest_day)
        }

        for e in &day_parts {
            if days.contains(&e.0 .0) {
                to_run.push(e);
            }
        }
    }

    for (key, f) in to_run {
        println!("[Day {} Part {}]: {}", key.0, key.1, f()?);
    }

    Ok(())
}
