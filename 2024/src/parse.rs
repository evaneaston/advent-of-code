use nom::{bytes::complete::tag, character::complete::one_of, multi::many0, sequence::tuple, IResult};

#[allow(dead_code)]
pub fn blank_line(input: &str) -> IResult<&str, ()> {
    tuple((many0(one_of(" \t")), tag("\n")))(input).map(|(input, _)| (input, ()))
}
