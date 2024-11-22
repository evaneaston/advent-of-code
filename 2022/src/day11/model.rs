use std::collections::VecDeque;


#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Plus(Operand),
    Times(Operand),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Number(usize),
    OldValue,
}

#[derive(Debug, Clone)]
pub struct DivisibleBy(pub usize);

#[derive(Debug, Clone)]
pub struct ThrowToMonkey(pub usize);

#[derive(Debug, Clone)]
pub struct Monkey {
    pub number: u64,
    pub item_worry_levels: VecDeque<usize>,
    pub operation: Operation,
    pub test: DivisibleBy,
    pub if_true: ThrowToMonkey,
    pub if_false: ThrowToMonkey,
    pub inspect_count: usize
}
