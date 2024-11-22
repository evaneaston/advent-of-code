use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

use flexi_logger::Logger;
use seq_macro::seq;
use thiserror::Error;

mod algo;
mod coord;
#[path = "day01/day01.rs"]
mod day01;
#[path = "day02/day02.rs"]
mod day02;
#[path = "day03/day03.rs"]
mod day03;
#[path = "day04/day04.rs"]
mod day04;
#[path = "day05/day05.rs"]
mod day05;
#[path = "day06/day06.rs"]
mod day06;
#[path = "day07/day07.rs"]
mod day07;
#[path = "day08/day08.rs"]
mod day08;
#[path = "day09/day09.rs"]
mod day09;
#[path = "day10/day10.rs"]
mod day10;
#[path = "day11/day11.rs"]
mod day11;
#[path = "day12/day12.rs"]
mod day12;
#[path = "day13/day13.rs"]
mod day13;
#[path = "day14/day14.rs"]
mod day14;
#[path = "day15/day15.rs"]
mod day15;
#[path = "day16/day16.rs"]
mod day16;
#[path = "day17/day17.rs"]
mod day17;
#[path = "day18/day18.rs"]
mod day18;
#[path = "day19/day19.rs"]
mod day19;
#[path = "day20/day20.rs"]
mod day20;
#[path = "day21/day21.rs"]
mod day21;
#[path = "day22/day22.rs"]
mod day22;
#[path = "day23/day23.rs"]
mod day23;
#[path = "day24/day24.rs"]
mod day24;
#[path = "day25/day25.rs"]
mod day25;
mod grid;
mod parse;

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
            Some(number) => format!("src/{day}/.input-{qualifier}-{day}{number}.txt"),
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
    #[error(transparent)]
    Parse {
        #[from]
        source: nom::error::Error<&'static str>,
    },
    #[error("Parse didn't read all input: {remaining}")]
    ParseNotComplete { remaining: String },
    #[error("Parse didn't succeed: {message} ")]
    ParseFailed { message: String },
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

pub fn enable_logging() -> Result<(), AocError> {
    Logger::try_with_env_or_str("info")?.log_to_stdout().start()?;
    Ok(())
}
