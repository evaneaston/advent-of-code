use self::model::PartRanges;
use crate::{AocError, DailyInput};
use log::debug;

pub(crate) mod model;
mod parse;

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let input_string = input.get_input_as_single_string()?;
    let sys = parse::parse_system(input_string.as_str())?;
    debug!("{:?}", sys);

    let accepted = sys
        .parts
        .iter()
        .filter(|&part| {
            let mut next = "in".to_owned();
            loop {
                let workflow = sys.keyed_workflows.get(next.as_str()).unwrap();
                match workflow.run_to_action(part) {
                    model::Action::RunWorkflow(name) => next = name.clone(),
                    model::Action::Reject => break false,
                    model::Action::Accept => break true,
                }
            }
        })
        .collect::<Vec<_>>();

    debug!("Accepted: {:?}", accepted);
    let sum: i64 = accepted.iter().map(|&p| p.sum_value()).sum();
    Ok(sum.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let input_string = input.get_input_as_single_string()?;
    let sys = parse::parse_system(input_string.as_str())?;

    let mut combinations = 0_i64;

    let mut actions = vec![("in".to_owned(), PartRanges::default())];

    while let Some((workflow_name, mut ranges)) = actions.pop() {
        let workflow = sys.keyed_workflows.get(workflow_name.as_str()).unwrap();

        let mut inverse_ranges = ranges;
        for branch in &workflow.branches {
            match &branch.predicate {
                model::Predicate::Always => (),
                model::Predicate::LessThan { prop, value } => {
                    ranges.reduce_range_to_below(prop, *value);
                    inverse_ranges.reduce_range_to_above(prop, ranges.get(prop).1);
                }
                model::Predicate::GreaterThan { prop, value } => {
                    ranges.reduce_range_to_above(prop, *value);
                    inverse_ranges.reduce_range_to_below(prop, ranges.get(prop).0);
                }
            };
            // were the branch predicate to evaluate to true, we'd do this
            match &branch.action {
                model::Action::RunWorkflow(workflow_name) => actions.push((workflow_name.to_string(), ranges)),
                model::Action::Reject => (),
                model::Action::Accept => combinations += ranges.combinations(),
            }
            // and if the branch predicate evaluages to false, we'd continue down the predicate chain with the inverse ranges
            ranges = inverse_ranges;
        }
    }

    Ok(combinations.to_string())
}

#[cfg(test)]
mod test {
    use crate::{
        day19::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 19,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "19114"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 19,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "348378"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 19,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "167409079868000"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 19,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "121158073425385"
        );
    }
}
