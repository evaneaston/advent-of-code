use flexi_logger::Logger;
use nom::{
    bytes::complete::tag, character::complete::one_of, multi::many0, sequence::tuple, IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};
use thiserror::Error;

pub type Part = fn() -> Result<String, AocError>;

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
impl InputType {
    fn get_input_file(&self, day: u8) -> Result<File, Error> {
        self.open_input_file(&mut match self {
            InputType::Example => format!("inputs/day{:02}-example-input.txt", day),
            InputType::Challenge => format!("inputs/day{:02}-input.txt", day),
        })
    }

    fn open_input_file(&self, file_name: &str) -> Result<File, Error> {
        match File::open(&file_name) {
            Ok(f) => Ok(f),
            Err(e) => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Unable to find file {}: {:?}", &file_name, e),
                ))
            }
        }
    }

    pub fn get_input_lines(&self, day: u8) -> Result<Vec<String>, Error> {
        Ok(BufReader::new(self.get_input_file(day)?)
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap())
    }

    pub fn get_input_as_single_string(&self, day: u8) -> Result<String, Error> {
        let mut file = self.get_input_file(day)?;
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
    Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .start()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::common::RowCol;

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

