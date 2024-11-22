use crate::common::AocError;
use crate::common::InputType;
use crate::common::RowCol;
use log::debug;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub(crate) struct PolyLine(pub Vec<RowCol>);

pub(crate) fn poly_lines(input: &str) -> IResult<&str, Vec<PolyLine>> {
    separated_list0(tag("\n"), poly_line)(input)
}

pub(crate) fn poly_line(input: &str) -> IResult<&str, PolyLine> {
    separated_list0(tag(" -> "), coordinate_pair)(input)
        .map(|(input, value)| (input, PolyLine(value)))
}

pub(crate) fn coordinate_pair(input: &str) -> IResult<&str, RowCol> {
    tuple((
        nom::character::complete::i64,
        tag(","),
        nom::character::complete::i64,
    ))(input)
    .map(|(input, parts)| (input, RowCol::new(parts.2, parts.0))) // note XY->YX swap
}

pub(crate) fn parse_input() -> Result<Vec<PolyLine>, AocError> {
    let all_input = InputType::Challenge.get_input_as_single_string(14)?;
    let lines = poly_lines(&all_input);
    debug!("Line definitions: {:?}", lines);
    let vec_lines = match lines {
        Ok((remaining, parsed)) => {
            if !remaining.is_empty() {
                panic!(
                    "Couldn't parse entire input.\nRemaining: {:?}\nParsed: {:?}",
                    remaining, parsed
                )
            } else {
                parsed
            }
        }
        Err(_) => todo!(),
    };
    Ok(vec_lines)
}
