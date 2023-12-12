use log::{debug, log_enabled, Level};
use crate::{AocError, DailyInput};

pub(crate) fn line_to_number_list(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap_or_else(|_| panic!("Invalid number {s}")))
        .collect()
}
pub(crate) fn history_to_difference_tree(history: &[i64]) -> Vec<Vec<i64>> {
    let mut tree: Vec<Vec<i64>> = vec![history.to_vec()];
    loop {
        let next = tree
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| (pair[1] - pair[0]))
            .collect::<Vec<i64>>();
        tree.push(next.clone());

        if next.iter().all(|&value| value == 0) {
            break;
        }
    }
    tree
}
pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let sum: i64 = input
        .get_input_lines()?
        .iter()
        .map(|line| {
            let history = line_to_number_list(line);
            debug!("  History: {:?}", history);
            let final_tree = {
                let mut tree = history_to_difference_tree(&history);

                if log_enabled!(Level::Debug) {
                    debug!("     Tree:");
                    for row in &tree {
                        debug!("     {:?}", row);
                    }
                }

                tree.last_mut().unwrap().push(0);

                debug!("treelein range {:?}", tree.len() - 1..=1);

                for i in (1..tree.len()).rev() {
                    let i_minus_one_last: i64 =
                        tree.get(i).unwrap().last().unwrap() + tree.get(i - 1).unwrap().last().unwrap();
                    tree.get_mut(i - 1).unwrap().push(i_minus_one_last);
                }

                if log_enabled!(Level::Debug) {
                    debug!("     After Tree:");
                    for row in &tree {
                        debug!("      {:?}", row);
                    }
                }
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
        .map(|line| {
            let history = line_to_number_list(line);
            debug!("  History: {:?}", history);

            let final_tree = {
                let mut tree = history_to_difference_tree(&history);

                if log_enabled!(Level::Debug) {
                    debug!("     Tree:");
                    for row in &tree {
                        debug!("     {:?}", row);
                    }
                }

                tree.last_mut().unwrap().insert(0, 0);

                debug!("treelein range {:?}", (1..tree.len()));

                for i in (1..tree.len()).rev() {
                    let i_minus_one_first: i64 =
                        tree.get(i - 1).unwrap().first().unwrap() - tree.get(i).unwrap().first().unwrap();
                    tree.get_mut(i - 1).unwrap().insert(0, i_minus_one_first);
                }

                if log_enabled!(Level::Debug) {
                    debug!("     After Tree:");
                    for row in &tree {
                        debug!("      {:?}", row);
                    }
                }
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
                input_type: InputType::Example,
                number: None,
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
                input_type: InputType::Challenge,
                number: None,
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
                input_type: InputType::Example,
                number: None,
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
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "1031"
        );
    }
}
