use std::iter::repeat;

use itertools::Itertools;

use crate::{AocError, DailyInput};

fn is_odd(v: &usize) -> bool {
    0 == v % 2
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let binding = input.get_input_as_single_string()?;
    let inputs = binding.trim();
    let inputs = inputs.chars().map(|c| (c as u8) - b'0').collect::<Vec<_>>();
    let mut v = 0_i64;
    let mut expanded = inputs
        .iter()
        .enumerate()
        .flat_map(|(i, d)| {
            let r = repeat(if is_odd(&i) { v } else { -1 }).take(*d as usize);
            if is_odd(&i) {
                v += 1;
            }
            r
        })
        .collect::<Vec<_>>();

    // eprintln!("{}", String::from_utf8(expanded.clone()).unwrap());

    let mut l = 0_usize;
    let mut r = expanded.len() - 1;
    while l < r {
        loop {
            if expanded[l] != -1 {
                l += 1;
            } else {
                break;
            }
        }
        loop {
            if expanded[r] == -1 {
                r -= 1;
            } else {
                break;
            }
        }
        if r > l {
            expanded[l] = expanded[r];
            expanded[r] = -1;
        }
    }
    // eprintln!("{}", String::from_utf8(expanded.clone()).unwrap());
    let answer: i64 = expanded
        .iter()
        .filter(|c| **c != -1)
        .enumerate()
        .map(|(index, id)| {
            let r = index as i64 * id;
            // eprint!("{index} x {id} = {r},");
            r
        })
        .sum();
    Ok(format!("{answer}"))
}

#[derive(Debug)]
enum Content {
    File { id: i64, len: usize },
    Gap { len: usize },
}
pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let binding = input.get_input_as_single_string()?;
    let inputs = binding.trim();
    let inputs = inputs.chars().map(|c| (c as u8) - b'0').collect::<Vec<_>>();
    let mut next_id = 0_i64;
    let mut disk = inputs
        .iter()
        .enumerate()
        .map(|(index, d)| {
            if index % 2 == 0 {
                let b = Content::File {
                    id: next_id,
                    len: *d as usize,
                };
                next_id += 1;
                b
            } else {
                Content::Gap { len: *d as usize }
            }
        })
        .filter(|b| match b {
            Content::File { .. } => true,
            Content::Gap { len } => *len > 0,
        })
        .collect::<Vec<_>>();

    // eprintln!("Start: {}", render_disk(&disk));

    let mut times = 0;

    let mut r = disk.len() - 1;
    loop {
        times += 1;

        if let Content::File { id, len: file_len } = disk[r] {
            let mut l = 0;
            while l < r {
                match disk[l] {
                    Content::File { .. } => {}
                    Content::Gap { len: gap_len } => {
                        if gap_len == file_len {
                            disk[l] = Content::File { id, len: file_len };
                            disk[r] = Content::Gap { len: file_len };
                            //eprintln!("Move {r} to {l}. {}", render_disk(&disk));
                            break;
                        } else if gap_len > file_len {
                            //eprintln!("Move {r} to {l} with extra gap of {}.", gap_len - file_len);
                            disk[l] = Content::File { id, len: file_len };
                            disk[r] = Content::Gap { len: file_len };
                            //eprintln!("     swap: {}", render_disk(&disk));
                            let mut new_gap_len = gap_len - file_len;
                            while let Content::Gap { len } = disk[l + 1] {
                                new_gap_len += len;
                                disk.remove(l + 1);
                                r -= 1;
                            }
                            //eprintln!(" coalesce: {}", render_disk(&disk));
                            disk.insert(l + 1, Content::Gap { len: new_gap_len });
                            r += 1;
                            //eprintln!(" final: {}", render_disk(&disk));
                            break;
                        }
                    }
                }
                l += 1;
            }
        }

        r -= 1;
        if r == 0 {
            break;
        }
    }
    //  eprint!("{}", render_disk(&disk));

    let answer = disk
        .iter()
        .flat_map(|c| match c {
            Content::File { id, len } => repeat(Some(*id)).take(*len),
            Content::Gap { len } => repeat(None).take(*len),
        })
        .enumerate()
        .map(|(i, v)| match v {
            Some(n) => n * i as i64,
            None => 0,
        })
        .sum::<i64>();

    Ok(format!("{answer}"))
}

fn render_disk(disk: &[Content]) -> String {
    disk.iter()
        .map(|c| match c {
            Content::File { id, len } => repeat(format!("{id}")).take(*len),
            Content::Gap { len } => repeat(format!(".")).take(*len),
        })
        .flatten()
        .join("")
}
#[cfg(test)]
mod test {
    use super::{part1, part2};
    use crate::{DailyInput, InputType};

    const DAY: usize = 9;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: DAY,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "1928"
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
            "6201130364722"
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
            "2858"
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
            "6221662795602"
        );
    }
}
