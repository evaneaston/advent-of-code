use super::Race;
use crate::{AocError, DailyInput};
use nom::character::complete::{space1, u64 as nom_u64};
use nom::{bytes::complete::tag, character::complete::space0, multi::separated_list1, sequence::tuple, IResult};

fn parse_line<'a>(expected_tag: &str, input: &'a str) -> IResult<&'a str, Vec<u64>> {
    let (remaining, (_, _, _, v)) =
        tuple((tag(expected_tag), tag(":"), space0, separated_list1(space1, nom_u64)))(input)?;
    Ok((remaining, v))
}

pub(crate) fn parse_day06_input(input: DailyInput) -> Result<Vec<Race>, AocError> {
    let lines = input.get_input_lines()?;

    let times = match parse_line("Time", &lines[0]) {
        Ok((_, times)) => times,
        Err(e) => panic!("Error parsing {:?}", e),
    };
    let distances = match parse_line("Distance", &lines[1]) {
        Ok((_, times)) => times,
        Err(e) => panic!("Error parsing {:?}", e),
    };

    Ok(std::iter::zip(times, distances)
        .map(|(time, distance)| Race { race_duration: time, record_distance: distance })
        .collect::<Vec<_>>())
}
