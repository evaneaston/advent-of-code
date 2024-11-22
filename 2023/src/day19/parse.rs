use crate::{blank_line, AocError};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

use super::model::{Action, Branch, Day19Inputs, Part, Predicate, Prop, Workflow};

pub(super) fn parse_system(input: &str) -> Result<Day19Inputs, AocError> {
    match parse_system_(input) {
        Ok((input, day_19_inputs)) => {
            if input.trim().is_empty() {
                Ok(day_19_inputs)
            } else {
                Err(AocError::ParseNotComplete { remaining: input.to_string() })
            }
        }
        Err(e) => Err(AocError::ParseFailed { message: e.to_string() })
    }
}

fn parse_system_(input: &str) -> IResult<&str, Day19Inputs> {
    let (input, workflows) = parse_workflows(input)?;
    let (input, _) = blank_line(input)?;
    let (input, parts) = parse_parts(input)?;
    Ok((input, Day19Inputs::new(workflows, parts)))
}

fn parse_workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
    many1(parse_workflow)(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, (name, _, branches, _, _)) =
        tuple((alphanumeric1, tag("{"), parse_branches, tag("}"), line_ending))(input)?;
    Ok((
        input,
        Workflow {
            name: String::from(name),
            branches,
        },
    ))
}

fn parse_branches(input: &str) -> IResult<&str, Vec<Branch>> {
    let (input, (mut first_branches, _, last_branch)) = tuple((
        separated_list1(tag(","), parse_conditional_branch),
        tag(","),
        parse_unconditional_branch,
    ))(input)?;
    first_branches.push(last_branch);
    Ok((input, first_branches))
}

fn parse_conditional_branch(input: &str) -> IResult<&str, Branch> {
    let (input, (prop, comparison, number, _, action)) = tuple((
        alphanumeric1,
        alt((tag("<"), tag(">"))),
        digit1,
        tag(":"),
        alphanumeric1,
    ))(input)?;
    let predicate = match &comparison {
        &"<" => Predicate::LessThan {
            prop: Prop::from(prop),
            value: number.parse::<i64>().unwrap(),
        },
        &">" => Predicate::GreaterThan {
            prop: Prop::from(prop),
            value: number.parse::<i64>().unwrap(),
        },
        c => panic!("Invalid comparison operator: {c}"),
    };
    Ok((
        input,
        Branch {
            predicate,
            action: Action::from(action),
        },
    ))
}
fn parse_unconditional_branch(input: &str) -> IResult<&str, Branch> {
    let (input, s) = alphanumeric1(input)?;
    Ok((
        input,
        Branch {
            predicate: Predicate::Always,
            action: Action::from(s),
        },
    ))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    many1(parse_part)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, (_, x, _, m, _, a, _, s, _, _)) = tuple((
        tag("{x="),
        digit1,
        tag(",m="),
        digit1,
        tag(",a="),
        digit1,
        tag(",s="),
        digit1,
        tag("}"),
        opt(line_ending),
    ))(input)?;
    Ok((
        input,
        Part {
            x: to_i64(x),
            m: to_i64(m),
            a: to_i64(a),
            s: to_i64(s),
        },
    ))
}

fn to_i64(s: &str) -> i64 {
    s.parse::<i64>().expect("{s} is not an i64")
}
