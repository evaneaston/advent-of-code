use crate::common::{InputType, AocError};
use super::model::{Assignment, Expression, Operation};
use log::debug;
use nom::{branch::alt, bytes::complete::tag, character::complete::alpha1, sequence::tuple, IResult};


impl From<&str> for Operation {
    fn from(op: &str) -> Operation {
        match op {
            " + " => Operation::Add,
            " - " => Operation::Subtract,
            " * " => Operation::Multiply,
            " / " => Operation::Divide,
            _ => panic!("Unsupported operation {}", op),
        }
    }
}

pub(super) fn load_inputs(input_type: InputType) -> Result<Vec<Assignment>, AocError> {
    let lines = input_type.get_input_lines(21)?;

    Ok(lines
        .iter()
        .map(|line| match parse_assignment(line) {
            Ok(a) => match a.0 {
                "" => a.1,
                left_over => panic!("Line: {}, Left over text: {:?}", line, left_over),
            },
            Err(e) => panic!("Line: {}, Error: {:?}", line, e),
        })
        .collect::<Vec<_>>())
}

pub(super) fn parse_assignment(input: &str) -> IResult<&str, Assignment> {
    tuple((parse_symbol, tag(": "), parse_expression))(input).map(|r| {
        debug!("parse_assignment: {:?}", r);
        (
            r.0,
            Assignment {
                symbol: r.1 .0,
                expression: r.1 .2,
            },
        )
    })
}

pub(super) fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((parse_number, parse_binary_operation))(input).map(|r| {
        debug!("parse_expression: {:?}", r);
        r
    })
}

pub(super) fn parse_symbol(input: &str) -> IResult<&str, String> {
    alpha1(input).map(|r| {
        debug!("parse_symbol: {:?}", r);
        (r.0, r.1.to_string())
    })
}

pub(super) fn parse_number(input: &str) -> IResult<&str, Expression> {
    nom::character::complete::i64(input).map(|r| {
        debug!("parse_number: {:?}", r);
        (r.0, Expression::Number(r.1))
    })
}

pub(super) fn parse_binary_operation(input: &str) -> IResult<&str, Expression> {
    tuple((
        parse_symbol,
        alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
        parse_symbol,
    ))(input)
    .map(|r| {
        debug!("parse_binary_operation: {:?}", r);
        (
            r.0,
            Expression::BinaryExpression(r.1 .0, Operation::from(r.1 .1), r.1 .2),
        )
    })
}
