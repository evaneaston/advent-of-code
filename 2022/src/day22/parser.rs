use crate::{
    common::{AocError, InputType, RowCol},
    grid::Grid,
};
use log::debug;
use nom::{branch::alt, bytes::complete::tag, multi::many1, IResult};

use super::model::Move;

pub(super) fn load_inputs(input_type: InputType) -> Result<(Grid, Vec<Move>), AocError> {
    let mut lines = input_type.get_input_lines(22)?;
    let move_line = lines.remove(lines.len() - 1);
    lines.remove(lines.len() - 1);

    let moves = match parse_moves(&move_line) {
            Ok(a) => match a.0 {
                "" => a.1,
                left_over => panic!("Line: {}, Left over text: {:?}", move_line, left_over),
            },
            Err(e) => panic!("Line: {}, Error: {:?}", move_line, e),
        };

    Ok((Grid::new_offset(RowCol::new(1, 1), &lines), moves))
}

pub(super) fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((parse_move_forward, parse_move_right_or_left)))(input).map(|r| {
        debug!("parse_moves: {:?}", r);
        (r.0, r.1)
    })
}

fn parse_move_forward(input: &str) -> IResult<&str, Move> {
    nom::character::complete::u64(input).map(|r| (r.0, Move::Forward(r.1)))
}

fn parse_move_right_or_left(input: &str) -> IResult<&str, Move> {
    alt((tag("R"), tag("L")))(input)
        .map(|r| (r.0, if r.1 == "R" { Move::Right } else { Move::Left }))
}
