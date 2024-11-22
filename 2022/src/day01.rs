use crate::common::{AocError, InputType};
use log::debug;

#[derive(Debug, Clone)]
struct Elf {
    calories: Vec<usize>,
    total_calories: usize,
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            calories: Vec::new(),
            total_calories: 0,
        }
    }

    pub fn add_calories(&mut self, calories: usize) {
        self.calories.push(calories);
        self.total_calories += calories;
    }
}

pub fn part1() -> Result<String, AocError> {
    let elves = get_elves_sorted_descending()?;
    let result = elves.first().unwrap().total_calories;
    debug!("Top elf calories = {}", result);

    Ok(format!("{}", result))
}

pub fn part2() -> Result<String, AocError> {
    let elves = get_elves_sorted_descending()?;

    let top_three = &elves[0..3];
    for elf in top_three {
        debug!("{:?} => {}", elf, elf.total_calories);
    }

    let result = top_three.iter().map(|e| e.total_calories).sum::<usize>();
    debug!("Top 3 elf calories calories = {}", result);

    Ok(format!("{}", result))
}

fn get_elves_sorted_descending() -> Result<Vec<Elf>, std::io::Error> {
    let lines = InputType::Challenge.get_input_lines(1)?;
    let mut elves = Vec::<Elf>::new();
    let mut create_new_elf = true;
    for line in lines {
        if line.is_empty() {
            create_new_elf = true;
        } else {
            let calories = line.parse::<usize>().unwrap();

            if create_new_elf {
                elves.push(Elf::new());
                create_new_elf = false;
            }

            let elf = elves.last_mut().unwrap();
            elf.add_calories(calories);
        }
    }
    elves.sort_by(|a, b| a.total_calories.cmp(&b.total_calories));
    elves.reverse();
    Ok(elves)
}
