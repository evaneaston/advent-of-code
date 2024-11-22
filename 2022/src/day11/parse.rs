use super::model::{DivisibleBy, Monkey, Operand, Operation, ThrowToMonkey};
use crate::common::InputType;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, one_of},
    error::context,
    multi::{many0, many_m_n, separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};
use std::collections::VecDeque;

fn monkey(input: &str) -> IResult<&str, Monkey> {
    context(
        "Monkey",
        tuple((
            monkey_number,
            starting_items,
            operation,
            divisible_by_test,
            if_true_branch,
            if_false_branch,
        )),
    )(input)
    .map(|(input, parsed)| {
        let (number, items, operation, test, if_true, if_false) = parsed;
        let mut item_queue: VecDeque<usize> = VecDeque::with_capacity(50); // big enough for any 1 monkey to hold all items so resizing will not occur
        item_queue.extend(items.iter().map(|ui| *ui as usize));
        (
            input,
            Monkey {
                number,
                item_worry_levels: item_queue,
                operation,
                test,
                if_true,
                if_false,
                inspect_count: 0,
            },
        )
    })
}

fn blank_line(input: &str) -> IResult<&str, ()> {
    context("blank_line", tuple((many0(one_of(" \t")), tag("\n"))))(input)
        .map(|(input, _)| (input, ()))
}

fn monkey_number(input: &str) -> IResult<&str, u64> {
    context(
        "monkey_number",
        tuple((
            tag("Monkey"),
            multispace1,
            nom::character::streaming::u64,
            tag(":"),
            tag("\n"),
        )),
    )(input)
    .map(|(input, parts)| (input, parts.2))
}

fn starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    context(
        "starting_items",
        tuple((
            multispace0,
            tag("Starting items: "),
            separated_list0(tag(", "), nom::character::complete::u64),
            tag("\n"),
        )),
    )(input)
    .map(|(input, parts)| (input, parts.2))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    context(
        "operation",
        tuple((
            multispace0,
            tag("Operation: new = old "),
            alt((tag("*"), tag("+"))),
            multispace0,
            operand,
            tag("\n"),
        )),
    )(input)
    .map(|(input, parts)| {
        (
            input,
            match parts.2 {
                "+" => Operation::Plus(parts.4),
                "*" => Operation::Times(parts.4),
                _ => panic!("Invalid op"),
            },
        )
    })
}

fn operand(input: &str) -> IResult<&str, Operand> {
    context("operand", alt((tag("old"), digit1)))(input).map(|(input, alternative)| {
        (
            input,
            match alternative {
                "old" => Operand::OldValue,
                num => Operand::Number(num.parse::<usize>().unwrap()),
            },
        )
    })
}

fn divisible_by_test(input: &str) -> IResult<&str, DivisibleBy> {
    context(
        "test",
        tuple((
            multispace0,
            tag("Test: divisible by "),
            nom::character::complete::u64,
            tag("\n"),
        )),
    )(input)
    .map(|(input, parts)| (input, DivisibleBy(parts.2 as usize)))
}

fn if_true_branch(input: &str) -> IResult<&str, ThrowToMonkey> {
    context(
        "throw_branch",
        tuple((
            multispace0,
            tag("If true: throw to monkey "),
            nom::character::complete::u64,
            tag("\n"),
        )),
    )(input)
    .map(|(input, parts)| (input, ThrowToMonkey(parts.2 as usize)))
}

fn if_false_branch(input: &str) -> IResult<&str, ThrowToMonkey> {
    context(
        "throw_branch",
        tuple((
            multispace0,
            tag("If false: throw to monkey "),
            nom::character::complete::u64,
            many_m_n(0, 1, tag("\n")),
        )),
    )(input)
    .map(|(input, parts)| (input, ThrowToMonkey(parts.2 as usize)))
}

#[cfg(test)]
mod tests {
    use super::{blank_line, operation, starting_items, Operand, Operation};

    #[test]
    fn blank_line_test() {
        assert_eq!(blank_line("  \n"), Ok(("", ())))
    }

    #[test]
    fn starting_items_test() {
        assert_eq!(
            starting_items("  Starting items: 57\n"),
            Ok(("", vec![57_u64]))
        )
    }

    #[test]
    fn operation_test() {
        assert_eq!(
            operation("  Operation: new = old + 3\n"),
            Ok(("", Operation::Plus(Operand::Number(3))))
        )
    }
}

pub fn load_input() -> Result<Vec<Monkey>, std::io::Error> {
    let input = InputType::Challenge.get_input_as_single_string(11)?;

    match separated_list1(blank_line, monkey)(&input) {
        Ok((remaining, monkeys)) => {
            if remaining.len() > 0 {
                panic!(
                    "Unable to parse all monkeys.  Remaining: {:?}\nParsed Monkeys: {:?}",
                    remaining, monkeys
                );
            }

            return Ok(monkeys);
        }
        Err(e) => panic!("Error parsing: {:?}", e),
    };
}
