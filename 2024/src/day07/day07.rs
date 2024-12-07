use regex::Regex;

use crate::{AocError, DailyInput};

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

fn parse(input: &DailyInput) -> Result<Vec<(i64, Vec<i64>)>, AocError> {
    let re = Regex::new(r"^(\d+): (.*)$").unwrap();
    Ok(input
        .get_input_lines()?
        .iter()
        .map(|line| {
            let t = re
                .captures(line)
                .expect("line matches pattern")
                .iter()
                .skip(1)
                .map(|sm| sm.unwrap().as_str())
                .collect::<Vec<_>>();
            let answer = t[0].parse::<i64>().unwrap();
            let operands = t[1].trim().split(' ').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
            (answer, operands)
        })
        .collect::<Vec<_>>())
}

fn generate_permutations(n: usize) -> Vec<Vec<Operator>> {
    let mut permutations = Vec::new();
    let total_combinations = 1 << n; // 2^n combinations

    for i in 0..total_combinations {
        let mut permutation = Vec::new();
        for bit in 0..n {
            // Check if the bit at position `bit` is set
            if (i & (1 << bit)) != 0 {
                permutation.push(Operator::Add);
            } else {
                permutation.push(Operator::Multiply);
            }
        }
        permutations.push(permutation);
    }

    permutations
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let inputs = parse(&input)?;
    eprintln!("{:?}", inputs);
    let answer = inputs
        .iter()
        .filter_map(|(answer, operands)| {

            let compute = |operators:&Vec<Operator>|  -> i64 {
                let mut x = *operands.first().unwrap();
                for (i, op) in operators.iter().enumerate() {
                    x = match op {
                        Operator::Add => x + operands[i + 1],
                        Operator::Multiply => x * operands[i + 1],
                    };
                }
                x
            };

            let operation_permutations = generate_permutations(operands.len() - 1);
            operation_permutations.iter().map(compute).find(|&a| a == *answer)
        }).sum::<i64>();
    Ok(format!("{answer}"))
}

#[derive(Debug)]
enum Operator2 {
    Add,
    Multiply,
    Concatenate,
}



fn generate_permutations2(n: usize) -> Vec<Vec<Operator2>> {
    let mut permutations = Vec::new();
    let total_combinations = usize::pow(3, n as u32); // 3^n combinations

    for i in 0..total_combinations {
        let mut permutation = Vec::new();
        let mut value = i;

        for _ in 0..n {
            // Map the current base-3 digit to an enum variant
            let digit = value % 3; // Get the remainder (base-3 digit)
            let enum_variant = match digit {
                0 => Operator2::Add,
                1 => Operator2::Multiply,
                2 => Operator2::Concatenate,
                _ => unreachable!(),
            };
            permutation.push(enum_variant);
            value /= 3; // Move to the next base-3 digit
        }

        permutations.push(permutation);
    }

    permutations
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let inputs = parse(&input)?;
    eprintln!("{:?}", inputs);
    let answer = inputs
        .iter()
        .filter_map(|(answer, operands)| {

            let compute = |operators:&Vec<Operator2>|  -> i64 {
                let mut x = *operands.first().unwrap();
                for (i, op) in operators.iter().enumerate() {
                    x = match op {
                        Operator2::Add => x + operands[i + 1],
                        Operator2::Multiply => x * operands[i + 1],
                        Operator2::Concatenate => format!("{x}{}", operands[i+1]).parse::<i64>().unwrap(),
                    };
                }
                x
            };

            let operation_permutations = generate_permutations2(operands.len() - 1);
            operation_permutations.iter().map(compute).find(|&a| a == *answer)
        }).sum::<i64>();
    Ok(format!("{answer}"))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 7;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "3749"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "7579994664753"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "11387"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: DAY,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "438027111276610"
        );
    }
}
