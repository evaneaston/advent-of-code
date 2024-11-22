use log::debug;
use std::collections::HashMap;

use crate::{AocError, DailyInput};

struct Solver {}
impl Solver {
    fn run(row: &[u8], sizes: &[usize]) -> usize {
        let mut cache: HashMap<Vec<usize>, usize> = HashMap::new();
        Solver {}.count_valid_paths(&mut cache, 0, row, sizes)
    }
    fn count_valid_paths(
        &self,
        cache: &mut HashMap<Vec<usize>, usize>,
        indent: usize,
        row: &[u8],
        sizes: &[usize],
    ) -> usize {
        let key = row.iter().map(|&v| v as usize).chain(sizes.iter().copied()).collect::<Vec<_>>();
        if let Some(count) = cache.get(&key) {
            *count
        } else {
            let count = self.count_valid_paths_(cache, indent, row, sizes);
            cache.insert(key, count);
            count
        }
    }
    fn count_valid_paths_(
        &self,
        cache: &mut HashMap<Vec<usize>, usize>,
        indent: usize,
        row: &[u8],
        sizes: &[usize],
    ) -> usize {
        let prefix = format!(
            "{}row={:?} sizes={:?} ",
            "  ".repeat(indent),
            &String::from_utf8(row.to_vec()).unwrap(),
            sizes
        );

        if row.is_empty() {
            return if sizes.is_empty() {
                debug!("{prefix}A 1");
                1
            } else {
                debug!("{prefix}B 0");
                0
            };
        }
        if !sizes.is_empty() && sizes[0] > row.len() {
            debug!("{prefix}C2 0");
            return 0;
        }

        match row[0] {
            b'.' => {
                debug!("{prefix}D");
                let c = self.count_valid_paths(cache, indent + 1, &row[1..], sizes);
                debug!("{prefix}D.f {c}");
                c
            }
            b'?' => {
                debug!("{prefix}E");
                let mut new_row = Vec::from(row);

                new_row[0] = b'.';
                debug!("{prefix}E.1 for '.'");
                let count = self.count_valid_paths(cache, indent + 1, &new_row, sizes);
                debug!("{prefix}E.1 for '.' {count}");

                new_row[0] = b'#';
                debug!("{prefix}E.2 for '#'");
                let count2 = self.count_valid_paths(cache, indent + 1, &new_row, sizes);
                debug!("{prefix}E.2 for '#' {count2}");

                count + count2
            }
            b'#' => {
                if sizes.is_empty() {
                    debug!("{prefix}F-1 0");
                    return 0;
                }
                let size = sizes[0];
                // we know size >= row.len() from above
                if row[0..size].iter().all(|c| *c == b'#' || *c == b'?') {
                    if row.len() == size || row[size] == b'.' {
                        debug!("{prefix}F");
                        let new_row = &row[size..];
                        let new_sizes = &sizes[1..];
                        let c = self.count_valid_paths(cache, indent + 1, new_row, new_sizes);
                        debug!("{prefix}F.f {c}");
                        c
                    } else {
                        match row[size] {
                            b'#' => {
                                debug!("{prefix}G 0");
                                0
                            }
                            b'?' => {
                                debug!("{prefix}H");
                                let mut new_row = row[size..].to_vec();
                                new_row[0] = b'.';
                                let new_sizes = &sizes[1..];
                                let c = self.count_valid_paths(cache, indent + 1, &new_row, new_sizes);
                                debug!("{prefix}G.f {c}");
                                c
                            }
                            c => panic!("unknown char {}", c),
                        }
                    }
                } else {
                    debug!("{prefix}H 0");
                    0
                }
            }
            c => panic!("Should not get a {c}"),
        }
    }
}

fn parse(line: &str) -> (&str, Vec<usize>) {
    let p = line.split(' ').collect::<Vec<_>>();
    assert_eq!(p.len(), 2, "Expected 2 parts in line {line}");
    let row = p[0];
    let sizes = p[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    (row, sizes)
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let mut sum = 0_usize;
    for (i, line) in input.get_input_lines()?.iter().filter(|l| !l.is_empty()).enumerate() {
        debug!("i={i}");
        let (row, sizes) = parse(line);

        debug!("============= {i} {row} ==================================");
        sum += Solver::run(row.as_bytes(), &sizes);
    }
    Ok(sum.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let mut sum = 0_usize;
    for (i, line) in input.get_input_lines()?.iter().filter(|l| !l.is_empty()).enumerate() {
        debug!("i={i}");
        let (row, sizes) = parse(line);

        let row = (0..5).map(|_| row).collect::<Vec<&str>>().join("?");
        let sizes = sizes.repeat(5);

        debug!("============= {i} {row} {:?}", sizes);
        let c = Solver::run(row.as_bytes(), &sizes);
        debug!("   c={c}");
        sum += c;
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use crate::{
        day12::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 12,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "21"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 12,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "7939"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 12,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "525152"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 12,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "850504257483930"
        );
    }
}
