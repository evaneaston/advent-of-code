use super::model::{Assignment, Expression, Operation};
use std::collections::HashMap;

pub(super) struct Evaluator {
    pub(super) values: HashMap<String, i64>,
    pub(super) keyed_assignments: HashMap<String, Expression>,
}

impl Evaluator {
    pub(super) fn new(assignments: &Vec<Assignment>) -> Self {
        Self {
            values: HashMap::with_capacity(assignments.len()),
            keyed_assignments: assignments
                .iter()
                .map(|a| (a.symbol.clone(), a.expression.clone()))
                .collect::<HashMap<String, Expression>>(),
        }
    }

    pub(super) fn evaluate(&mut self, symbol: &str) -> i64 {
        self.evaluate_recursively(symbol.to_string())
    }

    pub(super) fn evaluate_recursively(&mut self, symbol: String) -> i64 {
        if let Some(value) = self.values.get(&symbol) {
            return *value;
        }
        let value: i64 = match self.keyed_assignments[&symbol].clone() {
            Expression::Number(number) => number,
            Expression::BinaryExpression(l, o, r) => {
                let left = self.evaluate_recursively(l.to_string());
                let right = self.evaluate_recursively(r.to_string());
                match o {
                    Operation::Add => left + right,
                    Operation::Subtract => left - right,
                    Operation::Multiply => left * right,
                    Operation::Divide => left / right,
                }
            }
        };
        self.values.insert(symbol, value);
        value
    }
}
