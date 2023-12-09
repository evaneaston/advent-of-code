use std::cmp::{max, min};

use crate::{AocError, DailyInput};

pub(crate) fn line_to_number_list(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap_or_else(|_| panic!("Invalid number {s}")))
        .collect()
}
pub(crate) fn history_to_difference_tree(history: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut tree: Vec<Vec<i64>> = vec![history.to_vec()];
    loop {
        // println!("     Tree:");
        // for row in &tree {
        //     println!("     {:?}", row);
        // }
        let next = tree
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| (pair[1] - pair[0]))
            .collect::<Vec<i64>>();
        tree.push(next.clone());

        if next.iter().all(|&value| value == 0) {
            // for row in &tree {
            //     println!("     {:?}", row);
            // }
            break;
        }
    }
    tree
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let sum: i64 = input
        .get_input_lines()?
        .iter()
        .enumerate()
        .map(|(line_num, line)| {
            let history = line_to_number_list(&line);
            // println!("Line ({}): {:?}", line_num, line);
            // println!("  History: {:?}", history);
            let final_tree = {
                let mut tree = history_to_difference_tree(&history);

                // println!("     Tree:");
                // for row in &tree {
                //     println!("     {:?}", row);
                // }

                tree.last_mut().unwrap().push(0);

                //println!("treelein range {:?}", tree.len() - 1..=1);

                for i in (1..tree.len()).rev() {
                    // println!("      i={}", i);
                    let i_minus_one_last: i64 =
                        tree.get(i).unwrap().last().unwrap() + tree.get(i - 1).unwrap().last().unwrap();
                    tree.get_mut(i - 1).unwrap().push(i_minus_one_last);
                }

                // println!("     After Tree:");
                // for row in &tree {
                //     println!("      {:?}", row);
                // }
                tree
            };
            let last = final_tree.first().unwrap().last().unwrap();
            *last
        })
        .sum();

    Ok(sum.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let sum: i64 = input
        .get_input_lines()?
        .iter()
        .enumerate()
        .map(|(line_num, line)| {
            let history = line_to_number_list(&line);
            // println!("Line ({}): {:?}", line_num, line);
            // println!("  History: {:?}", history);
            let final_tree = {
                let mut tree = history_to_difference_tree(&history);

                // println!("     Tree:");
                // for row in &tree {
                //     println!("     {:?}", row);
                // }

                tree.last_mut().unwrap().insert(0, 0);

                // println!("treelein range {:?}", (1..tree.len()));

                for i in (1..tree.len()).rev() {
                    // println!("      i={}", i);
                    let i_minus_one_first: i64 =
                        tree.get(i - 1).unwrap().first().unwrap() - tree.get(i).unwrap().first().unwrap();
                    tree.get_mut(i - 1).unwrap().insert(0, i_minus_one_first);
                }

                // println!("     After Tree:");
                // for row in &tree {
                //     println!("      {:?}", row);
                // }
                tree
            };
            let last = final_tree.first().unwrap().first().unwrap();
            *last
        })
        .sum();

    Ok(sum.to_string())
}
#[cfg(test)]
mod test {
    use crate::{
        day09::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 9,
                part: None,
                input_type: InputType::Example
            })
            .unwrap(),
            "114"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 9,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            "1884768153"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 9,
                part: None,
                input_type: InputType::Example
            })
            .unwrap(),
            "2"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 9,
                part: None,
                input_type: InputType::Challenge
            })
            .unwrap(),
            "1031"
        );
    }
}
