use crate::common::{AocError, InputType};
use log::debug;
use std::{
    io::{Error, ErrorKind},
    ops::RangeInclusive,
};

fn str_to_u32(s: &str) -> Result<u32, Error> {
    match s.parse() {
        Ok(u) => Ok(u),
        Err(e) => Err(Error::new(
            ErrorKind::Other,
            format!("Unable to parse {} as u32.  Error: {:?}", s, e),
        )),
    }
}

fn to_range(s: &str) -> Result<RangeInclusive<u32>, Error> {
    let parts = s.split("-").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);

    let start: u32 = str_to_u32(parts[0])?;
    let end: u32 = str_to_u32(parts[1])?;

    Ok(start..=end)
}

pub fn is_full_overlap(r0: &RangeInclusive<u32>, r1: &RangeInclusive<u32>) -> bool {
    (r0.contains(r1.start()) && r0.contains(r1.end()))
        || (r1.contains(r0.start()) && r1.contains(r0.end()))
}

pub fn is_any_overlap(r0: &RangeInclusive<u32>, r1: &RangeInclusive<u32>) -> bool {
    r0.contains(r1.start())
        || r0.contains(r1.end())
        || r1.contains(r0.start())
        || r1.contains(r0.end())
}

fn get_results() -> Result<(i32, i32), AocError> {
    let mut line_number = 0;
    let mut count_full_overlap = 0;
    let mut count_any_overlap = 0;
    for line in InputType::Challenge.get_input_lines(4)?.iter() {
        line_number += 1;

        let parts = line.split(",").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2, "Error on line {}", line_number);

        let range0 = to_range(parts[0])?;
        let range1 = to_range(parts[1])?;
        let full_overlap = is_full_overlap(&range0, &range1);
        let any_overlap = is_any_overlap(&range0, &range1);
        debug!(
            "R0={:?} R1={:?}    full_overlap={} / any_overlap={}",
            range0, range1, full_overlap, any_overlap
        );
        if full_overlap {
            count_full_overlap += 1;
        }
        if any_overlap {
            count_any_overlap += 1;
        }
    }
    debug!("Number of complete overlaps = {}", count_full_overlap);
    debug!("Number of overlaps = {}", count_any_overlap);
    Ok((count_full_overlap, count_any_overlap))
}

pub fn part1() -> Result<String, AocError> {
    Ok(format!("{}", get_results()?.0))
}

pub fn part2() -> Result<String, AocError> {
    Ok(format!("{}", get_results()?.1))
}

#[cfg(test)]
mod tests {
    use crate::day04::{is_any_overlap, is_full_overlap};

    #[test]
    fn test_is_full_overlap() {
        assert!(is_full_overlap(&(0..=10), &(0..=0)));
        assert!(is_full_overlap(&(0..=10), &(10..=10)));
        assert!(is_full_overlap(&(0..=0), &(0..=10)));
        assert!(is_full_overlap(&(0..=0), &(0..=0)));

        assert!(!is_full_overlap(&(0..=10), &(5..=11)));
        assert!(!is_full_overlap(&(0..=10), &(10..=11)));

        assert!(!is_full_overlap(&(0..=10), &(25..=25)));
    }

    #[test]
    fn test_is_any_overlap() {
        assert!(is_any_overlap(&(0..=10), &(0..=0)));
        assert!(is_any_overlap(&(0..=10), &(10..=10)));
        assert!(is_any_overlap(&(0..=0), &(0..=10)));
        assert!(is_any_overlap(&(0..=0), &(0..=0)));

        assert!(is_any_overlap(&(1..=10), &(0..=1)));
        assert!(is_any_overlap(&(1..=10), &(5..=11)));
        assert!(is_any_overlap(&(5..=11), &(1..=10)));
        assert!(is_any_overlap(&(0..=10), &(10..=11)));

        assert!(!is_any_overlap(&(0..=10), &(25..=25)));
        assert!(!is_any_overlap(&(0..=10), &(25..=25)));
    }
}
