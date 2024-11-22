use self::{eval::Evaluator, parser::load_inputs, reorg::reorganize_assignments};
use crate::common::{AocError, InputType};
mod eval;
mod model;
mod parser;
mod reorg;

#[cfg(test)]
mod tests;

pub fn part1() -> Result<String, AocError> {
    let assignments = load_inputs(InputType::Challenge)?;
    let answer = Evaluator::new(&assignments).evaluate("root");
    assert_eq!(answer, 268597611536314);
    Ok(format!("{}", answer))
}

pub fn part2() -> Result<String, AocError> {
    let assignments = load_inputs(InputType::Challenge)?;
    let restructured_assignments = reorganize_assignments(&assignments);
    let answer = Evaluator::new(&restructured_assignments).evaluate("humn");
    assert_eq!(answer, 3451534022348);
    Ok(format!("{}", answer))
}
