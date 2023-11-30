use aoc2023::{enable_logging, get_day_parts, AocError, DayPartFn};
use regex::Regex;
use std::{collections::BTreeSet, env};

fn main() -> Result<(), AocError> {
    enable_logging()?;

    let day_parts = get_day_parts();

    for DayPartFn {
        day,
        part,
        function,
    } in find_parts_to_run(&day_parts)
    {
        // println!(
        //     "[Day {} Part {}]: {}",
        //     day,
        //     part,
        //     function(DailyInput {
        //         day: *day,
        //         input_type: InputType::Challenge
        //     })?
        // );

        println!("[Day {} Part {}]", day, part,);
    }

    Ok(())
}

fn find_parts_to_run(day_parts: &[DayPartFn]) -> Vec<&DayPartFn> {
    let args = env::args().collect::<Vec<_>>();
    let args = args.split_first().unwrap().1;

    if args.iter().any(|a| a == "all") {
        day_parts.iter().collect()
    } else {
        if args.is_empty() {
            panic!("No arguments")
        }

        let re = Regex::new(r"^([1-9]|1[0-9]|2[0-5])(,(1|2))?$").unwrap();
        let day_and_part_nums = args
            .iter()
            .enumerate()
            .flat_map(|(i, arg)| {
                println!("arg {},m {:#?}", arg, re.captures(arg));
                match re.captures(arg) {
                    Some(c) => {
                        println!("c.len {}", c.len());
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

        day_parts
            .iter()
            .filter(|dp| day_and_part_nums.contains(&(dp.day, dp.part)))
            .collect::<Vec<_>>()
    }
}
