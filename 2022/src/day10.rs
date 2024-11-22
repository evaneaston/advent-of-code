use crate::common::{AocError, InputType};
use std::io::Error;

#[derive(Debug)]
enum Op {
    NOOP,
    ADDX(i32),
}

impl Op {
    fn register_adjustment_instructions(&self) -> Vec<Option<i32>> {
        match &self {
            Op::NOOP => vec![None],
            Op::ADDX(v) => vec![None, Some(*v)],
        }
    }
}

fn get_cycle_start_and_end() -> Result<Vec<(i32, i32)>, Error> {
    let instructions = get_instructions()?;

    let mut x = 1_i32;
    let xs: Vec<(i32, i32)> = instructions
        .iter()
        .flat_map(|op| op.register_adjustment_instructions())
        .map(|v| {
            let start = x;
            x += match v {
                Some(v) => v,
                None => 0,
            };
            (start, x)
        })
        .collect();

    Ok(xs)
}

fn get_signal_strengths() -> Result<Vec<i32>, Error> {
    let se = get_cycle_start_and_end()?;

    let mut values_as_product = vec![1_i32; se.len()];
    for i in 0_usize..se.len() {
        let ii = i as i32;
        values_as_product[i] = (ii + 1) * se[i].0;
    }
    Ok(values_as_product)
}

pub fn part1() -> Result<String, AocError> {
    let values = get_signal_strengths()?;

    let mut sum: i32 = 0;
    for i in (20..=220).step_by(40) {
        sum += values[i - 1];
    }

    assert_eq!(sum, 13220);

    Ok(format!("{}", sum))
}

pub fn part2() -> Result<String, AocError> {
    let xs = get_cycle_start_and_end()?
        .iter()
        .map(|p| p.0)
        .collect::<Vec<i32>>();

    let mut pixels = vec!['.'; xs.len()];
    let mut cycle = 0;
    for x in xs {
        let crt_x = cycle % 40;

        pixels[cycle] = if crt_x as i32 >= (x - 1) && crt_x as i32 <= x + 1 {
            '#'
        } else {
            '.'
        };
        cycle += 1;
    }

    let mut result = String::new();
    result.push('\n');

    let str: String = pixels.into_iter().collect();
    for i in (0..str.len()).step_by(40) {
        result.push_str(&str[i..i + 40]);
        result.push('\n');
    }
    Ok(result)
}

fn get_instructions() -> Result<Vec<Op>, Error> {
    let lines = InputType::Challenge.get_input_lines(10)?;
    let mut cycle_ops: Vec<Op> = Vec::with_capacity(240);
    for line in &lines {
        let mut splitter = line.split(" ");

        match splitter.next() {
            Some(e) if e == "noop" => {
                cycle_ops.push(Op::NOOP);
            }
            Some(e) if e == "addx" => {
                let to_add = splitter.next().unwrap().parse::<i32>().unwrap();
                cycle_ops.push(Op::ADDX(to_add));
            }
            Some(_) => panic!(),
            None => panic!(),
        }
    }
    Ok(cycle_ops)
}
