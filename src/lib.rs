mod grid;
pub(crate) mod nums;

seq!(N in 1..=9 {
   mod day0~N;
});
seq!(N in 10..=25 {
   mod day~N;
});

use flexi_logger::Logger;
use log::{debug, info};
use nom::{bytes::complete::tag, character::complete::one_of, multi::many0, sequence::tuple, IResult};
use seq_macro::seq;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Error, Read},
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
            Some(number) => format!("src/{day}/inputs/{day}-{qualifier}{number}.txt"),
            None => format!("src/{day}/inputs/{day}-{qualifier}.txt"),
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
    pub fn plus_row(&self) -> Self {
        Self(self.row() + 1, self.col())
    }

    pub fn minus_row(&self) -> Self {
        Self(self.row() - 1, self.col())
    }

    pub fn plus_col(&self) -> Self {
        Self(self.row(), self.col() + 1)
    }

    pub fn minus_col(&self) -> Self {
        Self(self.row(), self.col() - 1)
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

pub(crate) fn count_distinct<T>(values: &[T]) -> HashMap<&T, usize>
where
    T: Eq + PartialEq + std::hash::Hash,
{
    values.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    })
}

// https://en.m.wikipedia.org/wiki/Shoelace_formula
// 1/2 Σ i->n (x[i]* y[i+1] - x[i+1]*y[i])
// this should work even if XY were changed to use non integers
pub(crate) fn shoelace_area(vertices: &[XY]) -> f64 {
    let mut sum = 0_f64;
    for i in 0..vertices.len() {
        let (v1, v2) = (
            vertices[i],
            if i == vertices.len() - 1 {
                vertices[0]
            } else {
                vertices[i + 1]
            },
        );

        sum += (v1.x() * v2.y() - v1.y() * v2.x()) as f64;
    }
    (sum / 2.).abs()
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem
//
// Pick's   A = i + b/2 - 1
//  A=area
//  i=iterior points
//  b=boundary points
//
// i= A+1-b/2
// This only works with integer vertices
pub(crate) fn get_num_interior_points(vertices: &[XY]) -> PicksResult {
    let vertices = &vertices;
    let area = shoelace_area(vertices);
    println!(" shoelace area={area}");

    let mut boundary_points_not_in_vertices = 0_usize;

    let mut looped: Vec<XY> = Vec::from(*vertices);
    looped.push(looped[0]);

    for p in looped.windows(2) {
        let a = p[0];
        let b = p[1];
        if a.x() == b.x() {
            let num_missing = ((b.y() - a.y()).abs() - 1).max(0);
            println!(" Between {:?} and {:?} there are {num_missing}", a, b);
            boundary_points_not_in_vertices += num_missing as usize;
        } else if a.y() == b.y() {
            let num_missing = ((b.x() - a.x()).abs() - 1).max(0);
            println!(" Between {:?} and {:?} there are {num_missing}", a, b);
            boundary_points_not_in_vertices += num_missing as usize;
        } else {
            // todo if we ever need angled ones, find integer intersections
            panic!("Assumed no angled edges");
        }
    }

    println!(" boundary_points_not_in_vertices={boundary_points_not_in_vertices}");

    let num_boundary_points = vertices.len() + boundary_points_not_in_vertices;
    let num_interior_points = area + 1_f64 - num_boundary_points as f64 / 2.;

    println!(" num_boundary_points={num_boundary_points}");
    println!(" num_interior_points={num_interior_points}");

    // num_interior_points.round() as i64
    PicksResult {
        shoelace_area: area,
        num_boundary_points,
        num_interior_points: num_interior_points.round() as usize,
    }
}

#[derive(Debug)]
struct PicksResult {
    shoelace_area: f64,
    num_boundary_points: usize,
    num_interior_points: usize,
}
impl Display for PicksResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "boundary_points: {}, interior_points: {}, shoelace_area: {}",
            self.num_boundary_points, self.num_interior_points, self.shoelace_area
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_num_interior_points, shoelace_area, RowCol, XY};

    #[test]
    fn row_col_conversions_test() {
        let rc = RowCol(1, 2);
        let (a, b): (i64, i64) = (rc.row(), rc.col());
        assert_eq!(a, 1);
        assert_eq!(b, 2);

        let new_rc: RowCol = (1, 2).into();
        assert_eq!(new_rc, rc);
    }

    #[test]
    fn test_shoelace_area() {
        assert_eq!(
            shoelace_area(&[
                XY::new(1, 6),
                XY::new(3, 1),
                XY::new(7, 2),
                XY::new(4, 4),
                XY::new(8, 5),
            ]),
            16.5
        );
    }

    #[test]
    fn test_picks() {
        // assert_eq!(
        //     get_num_interior_points(&[
        //         XY::new(1, 6),
        //         XY::new(3, 1),
        //         XY::new(7, 2),
        //         XY::new(4, 4),
        //         XY::new(8, 5),
        //     ]),
        //     15
        // );

        // day2 first example
        assert_eq!(
            get_num_interior_points(&[
                XY::new(1, 1),
                XY::new(9, 1),
                XY::new(9, 7),
                XY::new(6, 7),
                XY::new(6, 5),
                XY::new(8, 5),
                XY::new(8, 2),
                XY::new(2, 2),
                XY::new(2, 5),
                XY::new(4, 5),
                XY::new(4, 7),
                XY::new(1, 7)
            ]).num_interior_points,
            4
        );
    }
}
