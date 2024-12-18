use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

use flexi_logger::{Logger, LoggerHandle};
use seq_macro::seq;
use thiserror::Error;

mod algo;
mod coord;
mod grid;
mod parse;

// I wanted to have my modules be named dayXX.rs, but I didn't want them all in ./src.  If I put them into sub dirs for
// each day, then by convention, I must name them dayXX/mod.rs.  I could use the attribute #[path=...] to supply an alternate
// path/file name for each module.  But this doesn't work inside the seq! macro.
// This weirdly manufactures the equivalent of dayXX/mod.js that pub uses the sibling dayXX.js from that same directory making
// it possible to access as dayXX::part1, etc.
seq!(D in 01..=25 {
    #[allow(clippy::module_inception)]
    mod day~D { mod day~D; pub use day~D::*; }
});

#[allow(clippy::vec_init_then_push)]
pub fn get_day_parts() -> Vec<DayPartFn> {
    let mut day_parts = Vec::with_capacity(50);
    seq!(D in 01..=25 {
        seq!(P in 1..=2 {
            day_parts.push(DayPartFn::new(D, P, day~D::part~P));
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

#[derive(Clone)]
pub enum InputType {
    #[allow(dead_code)]
    Example,
    Challenge,
}
#[derive(Clone)]
pub struct DailyInput {
    pub day: usize,
    pub input_type: InputType,
    pub number: Option<usize>,
}
impl DailyInput {
    fn get_input_file(&self) -> Result<File, Error> {
        let day = format!("day{:02}", self.day);
        let qualifier = match self.input_type {
            InputType::Example => "example",
            InputType::Challenge => "challenge",
        };
        let file_name = match self.number {
            Some(number) => format!("src/{day}/.input-{qualifier}-{day}-{number}.txt"),
            None => format!("src/{day}/.input-{qualifier}-{day}.txt"),
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
        Ok(BufReader::new(self.get_input_file()?).lines().collect::<Result<Vec<String>, _>>().unwrap())
    }

    pub fn get_input_as_single_string(&self) -> Result<String, Error> {
        let mut file = self.get_input_file()?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Parse didn't succeed: {message} ")]
    ParseFailed { message: String },

    #[error("Parse didn't read all input: {remaining}")]
    ParseNotComplete { remaining: String },

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
}

pub fn enable_logging() -> Result<LoggerHandle, AocError> {
    Logger::try_with_env_or_str("info")?.log_to_stdout().start().map_err(|e| e.into())
}
