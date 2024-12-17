use std::fmt::Display;
use strum_macros::{Display, EnumIter};

pub struct Offset(pub i64, pub i64);

#[derive(EnumIter, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Direction {
    #[allow(dead_code)]
    pub fn turn_cw_90(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::NE => Direction::SE,
            Direction::E => Direction::S,
            Direction::SE => Direction::SW,
            Direction::S => Direction::W,
            Direction::SW => Direction::NW,
            Direction::W => Direction::N,
            Direction::NW => Direction::NE,
        }
    }
    #[allow(dead_code)]
    pub fn turn_ccw_90(&self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::NE => Direction::NW,
            Direction::E => Direction::N,
            Direction::SE => Direction::NE,
            Direction::S => Direction::E,
            Direction::SW => Direction::SE,
            Direction::W => Direction::S,
            Direction::NW => Direction::SW,
        }
    }
    #[allow(dead_code)]
    pub fn turn_180(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::NE => Direction::SW,
            Direction::E => Direction::W,
            Direction::SE => Direction::NW,
            Direction::S => Direction::N,
            Direction::SW => Direction::NE,
            Direction::W => Direction::E,
            Direction::NW => Direction::SE,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RowCol(pub i64, pub i64);

#[allow(dead_code)]
pub(crate) const fn rc(row: i64, col: i64) -> RowCol {
    RowCol(row, col)
}

///  2D coordinate: row ↕ , col ↔
#[allow(dead_code)]
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
    pub fn offset(&self, row_offset: i64, col_offset: i64) -> Self {
        Self(self.row() + row_offset, self.col() + col_offset)
    }
    pub fn offset_row(&self, row_offset: i64) -> Self {
        Self(self.row() + row_offset, self.col())
    }
    pub fn offset_col(&self, col_offset: i64) -> Self {
        Self(self.row(), self.col() + col_offset)
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
    pub fn plus(&self, direction: &Direction) -> RowCol {
        self.plus_n(direction, 1)
    }
    pub fn plus_n(&self, direction: &Direction, n: i64) -> RowCol {
        match direction {
            Direction::N => self.offset_row(-n),
            Direction::NE => self.offset(-n, n),
            Direction::E => self.offset_col(n),
            Direction::SE => self.offset(n, n),
            Direction::S => self.offset_row(n),
            Direction::SW => self.offset(n, -n),
            Direction::W => self.offset_col(-n),
            Direction::NW => self.offset(-n, -n),
        }
    }
    pub fn plus_offset(&self, offset: &Offset) -> Self {
        Self(self.0 + offset.0, self.1 + offset.1)
    }
    pub fn minus_offset(&self, offset: &Offset) -> Self {
        Self(self.0 - offset.0, self.1 - offset.1)
    }
    pub fn diff(&self, other: &Self) -> Offset {
        Offset(self.0 - other.0, self.1 - other.1)
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

/// 2D Coordinate - Cartesian:  x ↔ , y ↕
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct XY(pub i64, pub i64);

#[allow(dead_code)]
pub(crate) const fn xy(x: i64, y: i64) -> XY {
    XY(x, y)
}

#[allow(dead_code)]
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
