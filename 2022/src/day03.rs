use crate::common::{AocError, InputType};
use log::debug;
use std::collections::HashSet;

fn char_to_priority(c: char) -> usize {
    if c.is_lowercase() {
        return c as usize - 'a' as usize + 1;
    }
    return c as usize - 'A' as usize + 27;
}

#[cfg(test)]
mod tests {
    use crate::day03::char_to_priority;

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority('a'), 1);
        assert_eq!(char_to_priority('b'), 2);
        assert_eq!(char_to_priority('c'), 3);
        assert_eq!(char_to_priority('d'), 4);
        assert_eq!(char_to_priority('e'), 5);
        assert_eq!(char_to_priority('z'), 26);
        assert_eq!(char_to_priority('A'), 27);
        assert_eq!(char_to_priority('Z'), 52);
    }
}

fn to_u8_set(str: &str) -> HashSet<char> {
    let mut set = HashSet::<char>::new();
    str.as_bytes().iter().for_each(|b| {
        set.insert(*b as char);
    });
    set
}

pub fn part1() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(3)?;

    let mut priority_sum = 0;

    let mut line_number = 0;
    for line in lines {
        line_number += 1;

        let parts = line.split_at(line.len() / 2);

        assert!(
            parts.0.len() == parts.1.len(),
            "Line #{}) expected even-lengthed line",
            line_number
        );

        let compartment0 = to_u8_set(parts.0);
        let compartment1 = to_u8_set(parts.1);
        let intersection: HashSet<&char> = compartment0.intersection(&compartment1).collect();

        assert!(
            intersection.len() == 1,
            "(Line #{}) expected intersection of size one",
            line_number
        );

        let priority = char_to_priority(**intersection.iter().nth(0).unwrap());
        debug!(
            "{:?} ∩ {:?} = {:?}   with priority {}",
            compartment0, compartment1, intersection, priority
        );

        priority_sum += priority;
    }

    debug!("Sum of all priorities={}", priority_sum);

    Ok(format!("{}", priority_sum))
}

pub fn part2() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(3)?;

    let group_count_f = ((lines.len() as u32) as f32) / 3_f32;
    assert_eq!(group_count_f.floor(), group_count_f);

    let mut priority_sum = 0;

    for group_start in (0..lines.len()).step_by(3) {
        let mut group = Vec::<&str>::with_capacity(3);

        let a = &lines[group_start..(group_start + 3)];
        for i in (0..3).step_by(1) {
            group.push(a[i].as_str());
        }

        let elf0 = to_u8_set(group.get(0).unwrap());
        let elf1 = to_u8_set(group.get(1).unwrap());
        let elf2 = to_u8_set(group.get(2).unwrap());

        let intersection = elf0
            .intersection(&elf1)
            .map(|c| *c)
            .collect::<HashSet<char>>()
            .intersection(&elf2)
            .map(|c| *c)
            .collect::<HashSet<char>>();

        assert!(
            intersection.len() == 1,
            "(Group starting at line #{}) expected intersection of size one",
            group_start
        );

        let priority = char_to_priority(*intersection.iter().nth(0).unwrap());

        debug!(
            "Intersection of sets {:?} = {:?} with priority = {}",
            group, intersection, priority
        );

        priority_sum += priority;
    }

    debug!("Sum of all priorities={}", priority_sum);

    Ok(format!("{}", priority_sum))
}
