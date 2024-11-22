use crate::common::{AocError, InputType};
use log::debug;
use regex::Regex;
use std::{
    collections::VecDeque,
    io::{Error, ErrorKind},
};

fn str_to_usize(s: &str) -> Result<usize, Error> {
    match s.parse() {
        Ok(u) => Ok(u),
        Err(e) => Err(Error::new(
            ErrorKind::Other,
            format!("Unable to parse {} as usize.  Error: {:?}", s, e),
        )),
    }
}

#[derive(Debug)]

struct Move {
    quantity: usize,
    from_stack: usize,
    to_stack: usize,
}

struct Stacks(Vec<VecDeque<char>>);

impl Stacks {
    fn display(&self) {
        for stack_num in 0..self.0.len() {
            debug!(
                "Stack[{}]: BOTTOM>{:?}<TOP",
                stack_num + 1,
                self.0.get(stack_num).unwrap()
            );
        }
    }

    fn get_top_boxes(&self) -> String {
        for stack_num in 0..self.0.len() {
            debug!(
                "Stack[{}]: TOP BOX = {}",
                stack_num + 1,
                self.0.get(stack_num).unwrap().back().unwrap()
            );
        }

        let mut r: String = String::new();
        for stack_num in 0..self.0.len() {
            r.push(match self.0.get(stack_num).unwrap().back() {
                Some(b) => *b,
                None => ' ',
            });
        }
        r
    }

    fn apply_move_with_cm9000(&mut self, m: &Move) {
        for _count in 0..m.quantity {
            let bx = self
                .0
                .get_mut(m.from_stack - 1)
                .unwrap()
                .pop_back()
                .unwrap();
            self.0.get_mut(m.to_stack - 1).unwrap().push_back(bx);
        }
    }

    fn apply_move_with_cm9001(&mut self, m: &Move) {
        let from_stack = self.0.get_mut(m.from_stack - 1).unwrap();
        let mut bxes = from_stack.split_off(from_stack.len() - m.quantity);
        self.0.get_mut(m.to_stack - 1).unwrap().append(&mut bxes);
    }
}

pub fn part1() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(5)?;
    let mut stacks = parse_stacks(&lines);

    debug!("With the cm9000 crane algorithm");
    stacks.display();
    let moves = parse_moves(&lines);
    for m in &moves {
        debug!("Applying move {:?}", m);
        stacks.apply_move_with_cm9000(&m);
        //        stacks.display();
    }
    Ok(stacks.get_top_boxes())
}

pub fn part2() -> Result<String, AocError> {
    let lines = InputType::Challenge.get_input_lines(5)?;
    let mut stacks = parse_stacks(&lines);
    stacks.display();
    let moves = parse_moves(&lines);
    for m in &moves {
        debug!("Applying move {:?}", m);
        stacks.apply_move_with_cm9001(&m);
        //        stacks.display();
    }
    Ok(stacks.get_top_boxes())
}

fn parse_moves(lines: &Vec<String>) -> Vec<Move> {
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut moves = Vec::<Move>::with_capacity(lines.len());
    for line in lines.iter() {
        match move_re.captures(line) {
            Some(c) => {
                moves.push(Move {
                    quantity: str_to_usize(c.get(1).unwrap().as_str()).unwrap(),
                    from_stack: str_to_usize(c.get(2).unwrap().as_str()).unwrap(),
                    to_stack: str_to_usize(c.get(3).unwrap().as_str()).unwrap(),
                });
            }
            None => {}
        };
    }
    moves
}

fn parse_stacks(lines: &Vec<String>) -> Stacks {
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(20);

    for line in lines.iter() {
        if line.is_empty() {
            break;
        }

        if stacks.len() == 0 {
            // L: length of line
            // n: # of stacks
            // w: # chars width for for one stack
            // s: # chars providing space between each stack
            // L = w*n+(n-1)*s = 3n+(n-1)*1 = 3n+n-1 = 4n-1  ==> n = (L+1)/4
            let num_stacks = (line.len() + 1) / 4;
            //debug!("Num Stacks {}", num_stacks);

            for _n in 0..num_stacks {
                stacks.push(VecDeque::new());
            }
        }

        if stacks.len() == 0 {
            panic!("Shouldn't be here without an allocated set of stacks");
        }
        let mut chars = line.chars();
        for stack in 0..stacks.len() {
            let next_char = match stack {
                0 => chars.nth(1),
                _ => chars.nth(3),
            };

            match next_char {
                Some(next_char) => {
                    //                debug!(" {} {}", stack, next_char);
                    if next_char.is_alphabetic() {
                        stacks[stack].push_front(next_char)
                    }
                }
                None => break,
            }
        }
    }

    Stacks(stacks)
}

#[cfg(test)]
mod tests {}
