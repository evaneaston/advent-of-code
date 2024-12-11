use crate::{AocError, DailyInput};
use std::iter::repeat;

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

    let answer: i64 = expanded.iter().filter(|c| **c != -1).enumerate().map(|(index, id)| index as i64 * id).sum();

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
                let id = next_id;
                next_id += 1;
                Content::File { id, len: *d as usize }
            } else {
                Content::Gap { len: *d as usize }
            }
        })
        .filter(|b| match b {
            Content::File { .. } => true,
            Content::Gap { len } => *len > 0,
        })
        .collect::<Vec<_>>();

    // let mut index = 0;

    let mut right = disk.len() - 1;
    loop {
        //eprintln!("{index}:{}", render_disk(&disk));
        // index += 1;

        match disk[right] {
            Content::File { id, len: file_len } => {
                //eprintln!("  @right={right} File #{id} (len={file_len})");
                let mut left = 0;
                while left < right {
                    match disk[left] {
                        Content::Gap { len: gap_len } => {
                            //eprintln!("  @left={left} = gap (len={gap_len})");
                            if gap_len >= file_len {
                                //eprintln!("    it's big enough,");
                                disk[left] = Content::File { id, len: file_len };
                                disk[right] = Content::Gap { len: file_len };

                                if gap_len > file_len {
                                    let mut new_gap_len = gap_len - file_len;
                                    disk.insert(left + 1, Content::Gap { len: new_gap_len });
                                    right += 1;

                                    //eprintln!("    swapped and padded: {}", render_disk(&disk));

                                    while let Some(Content::Gap { len }) = disk.get(left + 2) {
                                        new_gap_len += len;
                                        disk.remove(left + 2);
                                        right -= 1;
                                    }
                                    disk[left + 1] = Content::Gap { len: new_gap_len };
                                    //eprintln!("    combined padding  : {}", render_disk(&disk));
                                }
                                //eprintln!("    left={left} right={right}");
                                break;
                            }
                        }
                        Content::File { .. } => {
                            //Content::File { id, len: file_len } => {
                            //eprintln!("  @left={left} = File {id} (len={file_len})");
                        }
                    }
                    left += 1;
                }
            }
            Content::Gap { .. } => {
            //Content::Gap { len } => {
                //eprintln!("  @right={right} Gap (len={len})");
            }
        }

        right -= 1;
        if right == 0 {
            break;
        }
    }
    //eprintln!("Final:{}", render_disk(&disk));

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

    Ok(answer.to_string())
}

// fn render_disk(disk: &[Content]) -> String {
//     let mut t = disk.iter().flat_map(|c| match c {
//         Content::File { id, len } => repeat(format!("{id}")).take(*len),
//         Content::Gap { len } => repeat(".".to_string()).take(*len),
//     });
//     t.join("")
// }

#[cfg(test)]
mod test {
    use crate::day09::{part1, part2};
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