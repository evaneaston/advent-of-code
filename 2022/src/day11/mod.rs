pub mod model;
mod parse;
use self::{
    model::{DivisibleBy, Monkey, Operation, ThrowToMonkey},
    parse::load_input,
};
use crate::common::AocError;
use log::{debug, max_level, LevelFilter};
use std::ops::{Add, Mul};

pub fn part1() -> Result<String, AocError> {
    let mut monkeys = load_input()?;

    for _ in 0..20 {
        round(&mut monkeys, true, usize::MAX);
    }

    let monkey_business = get_monkey_business(&monkeys);
    assert_eq!(monkey_business, 121450);
    Ok(format!("{}", monkey_business))
}

pub fn part2() -> Result<String, AocError> {
    let mut monkeys = load_input()?;

    let common_multiple = monkeys
        .iter()
        .map(|m| m.test.0)
        .reduce(|a, b| a * b)
        .unwrap();

    show_monkeys("Before", &monkeys);
    for _r in 0..10000 {
        round(&mut monkeys, false, common_multiple);
    }
    show_monkeys("After", &monkeys);

    let monkey_business = get_monkey_business(&monkeys);

    assert_eq!(monkey_business, 28244037010);

    Ok(format!("{}", monkey_business))
}

impl Monkey {
    fn catch_item(&mut self, worry_level: usize) {
        self.item_worry_levels.push_back(worry_level);
    }

    fn pick_and_decide(
        &mut self,
        div_by_three: bool,
        modulus: usize,
    ) -> Option<(usize, ThrowToMonkey)> {
        match self.item_worry_levels.len() {
            0 => None,
            _ => {
                let item_worry_level = self.item_worry_levels.pop_front().unwrap();
                debug!(
                    "  Monkey {} inspects an item with a worry level of {}",
                    self.number, item_worry_level
                );

                let item_worry_level = match &self.operation {
                    Operation::Plus(operand) => match operand {
                        model::Operand::Number(number) => item_worry_level + number,
                        model::Operand::OldValue => item_worry_level.clone().add(item_worry_level),
                    },
                    Operation::Times(operand) => match operand {
                        model::Operand::Number(number) => item_worry_level * number,
                        model::Operand::OldValue => item_worry_level.clone().mul(item_worry_level),
                    },
                };

                debug!(
                    "    Worry level changes by {:?} to {}",
                    self.operation, item_worry_level
                );

                let denom = if div_by_three { 3_usize } else { 1_usize };
                let item_worry_level =
                    (item_worry_level as f64 / denom as f64).floor() as usize % modulus;

                let target_monkey_number = match self.test.is_true(item_worry_level) {
                    true => {
                        debug!("    Current worry level is divisible by {}", self.test.0);
                        self.if_true.0
                    }
                    false => {
                        debug!(
                            "    Current worry level is not divisible by {}",
                            self.test.0
                        );
                        self.if_false.0
                    }
                };

                self.inspect_count += 1;

                debug!(
                    "    Item with worry level {} will be thrown to monkey {}.",
                    item_worry_level, target_monkey_number
                );

                Some((item_worry_level, ThrowToMonkey(target_monkey_number)))
            }
        }
    }
}

impl DivisibleBy {
    fn is_true(&self, value: usize) -> bool {
        let quot = value as f64 / self.0 as f64;
        quot.floor() == quot
    }
}

fn get_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut sorted_inspect_counts = monkeys.iter().map(|r| r.inspect_count).collect::<Vec<_>>();
    sorted_inspect_counts.sort_by(|a, b| a.cmp(&b).reverse());
    sorted_inspect_counts[0] * sorted_inspect_counts[1]
}

fn show_monkeys(label: &str, monkeys: &Vec<Monkey>) {
    if max_level() < LevelFilter::Debug {
        return;
    }
    debug!("{}:", label);
    for m in monkeys {
        debug!(" - {:?}", m);
    }
}

fn round(monkeys: &mut Vec<Monkey>, div_by_three: bool, modulus: usize) {
    for i in 0..monkeys.len() {
        debug!("Monkey {}", monkeys[i].number);
        loop {
            let decision = monkeys[i].pick_and_decide(div_by_three, modulus);
            match decision {
                Some(x) => {
                    let (item_worry_level, throw_to) = x;
                    monkeys[throw_to.0].catch_item(item_worry_level)
                }
                None => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::model::DivisibleBy;

    #[test]
    fn test_divisible_by() {
        assert_eq!(DivisibleBy(3).is_true(3), true);
        assert_eq!(DivisibleBy(3).is_true(4), false);
        assert_eq!(DivisibleBy(3).is_true(5), false);
        assert_eq!(DivisibleBy(3).is_true(6), true);
        assert_eq!(DivisibleBy(23).is_true(500), false);
        assert_eq!(DivisibleBy(23).is_true(620), false);
        assert_eq!(DivisibleBy(19).is_true(20), false);
        assert_eq!(DivisibleBy(19).is_true(23), false);
        assert_eq!(DivisibleBy(19).is_true(27), false);
    }
}
