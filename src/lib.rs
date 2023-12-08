mod grid;
seq!(N in 1..=9 {
   mod day0~N;
});
seq!(N in 10..=25 {
   mod day~N;
});

use flexi_logger::Logger;
use nom::{bytes::complete::tag, character::complete::one_of, multi::many0, sequence::tuple, IResult};
use seq_macro::seq;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Error, Read}, collections::HashMap,
};
use thiserror::Error;

#[allow(clippy::vec_init_then_push)]
pub fn get_day_parts() -> Vec<DayPartFn> {
    let mut day_parts: Vec<DayPartFn> = vec![];
    seq!(D in 1..=9 {
          day_parts.push(DayPartFn {
              day: D,
              part: 1,
              function: day0~D::part1
          });
          day_parts.push(DayPartFn {
           day: D,
           part: 2,
           function: day0~D::part2
       });
    });
    seq!(D in 10..=25 {
        day_parts.push(DayPartFn {
            day: D,
            part: 1,
            function: day~D::part1
        });
        day_parts.push(DayPartFn {
            day: D,
            part: 2,
            function: day~D::part2
        });
    });
    day_parts
}

pub type PartFn = fn(DailyInput) -> Result<String, AocError>;

pub struct DayPartFn {
    pub day: usize,
    pub part: usize,
    pub function: PartFn,
}
impl DayPartFn {
    pub fn new(day: usize, part: usize, function: PartFn) -> Self {
        Self { day, part, function }
    }
}

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    Parse {
        #[from]
        source: nom::error::Error<&'static str>,
    },
    #[error(transparent)]
    Log {
        #[from]
        source: flexi_logger::FlexiLoggerError,
    },
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error,
    },
    // #[error("Out of bounds")]
    // OutOfRange(String),
}

pub enum InputType {
    #[allow(dead_code)]
    Example,
    Challenge,
}
pub struct DailyInput {
    pub day: usize,
    pub part: Option<usize>,
    pub input_type: InputType,
}
impl DailyInput {
    fn get_input_file(&self) -> Result<File, Error> {
        let day = format!("day{:02}", self.day);
        let qualifier = match self.input_type {
            InputType::Example => "example",
            InputType::Challenge => "challenge",
        };
        let file_name = match self.part {
            Some(part) => format!("src/{day}/challenge/{day}-part{part}-{qualifier}-input.txt"),
            None => format!("src/{day}/challenge/{day}-{qualifier}-input.txt"),
        };
        self.open_input_file(&file_name)
    }

    fn open_input_file(&self, file_name: &str) -> Result<File, Error> {
        match File::open(file_name) {
            Ok(f) => Ok(f),
            Err(e) => Err(Error::new(
                std::io::ErrorKind::NotFound,
                format!("Unable to find file {}: {:?}", &file_name, e),
            )),
        }
    }

    pub fn get_input_lines(&self) -> Result<Vec<String>, Error> {
        Ok(BufReader::new(self.get_input_file()?)
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap())
    }

    pub fn get_input_as_single_string(&self) -> Result<String, Error> {
        let mut file = self.get_input_file()?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RowCol(i64, i64);
impl RowCol {
    pub fn new(row: i64, col: i64) -> RowCol {
        RowCol(row, col)
    }
    pub fn row(&self) -> i64 {
        self.0
    }
    pub fn col(&self) -> i64 {
        self.1
    }
}

impl Display for RowCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R{}C{}", self.row(), self.col())
    }
}
impl From<(i64, i64)> for RowCol {
    fn from(pair: (i64, i64)) -> Self {
        RowCol(pair.0, pair.1)
    }
}
impl From<XY> for RowCol {
    fn from(xy: XY) -> Self {
        RowCol(xy.y(), xy.x())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct XY(i64, i64);
impl XY {
    pub fn new(x: i64, y: i64) -> XY {
        XY(x, y)
    }
    pub fn x(&self) -> i64 {
        self.0
    }
    pub fn y(&self) -> i64 {
        self.1
    }
}

impl From<(i64, i64)> for XY {
    fn from(pair: (i64, i64)) -> Self {
        XY(pair.0, pair.1)
    }
}
impl From<RowCol> for XY {
    fn from(rc: RowCol) -> Self {
        XY(rc.col(), rc.row())
    }
}

pub fn blank_line(input: &str) -> IResult<&str, ()> {
    tuple((many0(one_of(" \t")), tag("\n")))(input).map(|(input, _)| (input, ()))
}

pub fn enable_logging() -> Result<(), AocError> {
    Logger::try_with_env_or_str("info")?.log_to_stdout().start()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::RowCol;

    #[test]
    fn row_col_conversions_test() {
        let rc = RowCol(1, 2);
        let (a, b): (i64, i64) = (rc.row(), rc.col());
        assert_eq!(a, 1);
        assert_eq!(b, 2);

        let new_rc: RowCol = (1, 2).into();
        assert_eq!(new_rc, rc);
    }
}

mod nums {

    pub(crate) fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    pub(crate) fn lcm_of_two(a: u64, b: u64) -> u64 {
        if a == 0 || b == 0 {
            0
        } else {
            (a * b) / gcd(a, b)
        }
    }

    pub(crate) fn lcm_of_multiple(numbers: &[u64]) -> u64 {
        numbers.iter().cloned().fold(1, lcm_of_two)
    }
}


pub(crate) fn count_distinct<T>(values: &[T]) -> HashMap<&T, usize>
where
    T: Eq + PartialEq + std::hash::Hash,
{
    values.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    })
}
