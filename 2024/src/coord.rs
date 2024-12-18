use std::fmt::Display;
use strum_macros::EnumIter;

pub struct Offset(pub i64, pub i64);

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
pub const NSEW: [Direction;4] = [Direction::N, Direction::S , Direction::E, Direction::W];
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

    #[allow(dead_code)]
    pub fn cw_turn_difference(&self, other: Direction) -> u64 {
        let mut d = *self;
        let mut count = 0;
        while d != other {
            d = d.turn_cw_90();
            count += 1;
        }
        count
    }

    #[allow(dead_code)]
    pub fn ccw_turn_difference(&self, other: Direction) -> u64 {
        let mut d = *self;
        let mut count = 0;
        while d != other {
            d = d.turn_ccw_90();
            count += 1;
        }
        count
    }
}
impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::N => '^',
            Direction::NE => todo!(),
            Direction::E => '>',
            Direction::SE => todo!(),
            Direction::S => 'v',
            Direction::SW => todo!(),
            Direction::W => '<',
            Direction::NW => todo!(),
        }
    }
}
impl From<Direction> for u8 {
    fn from(val: Direction) -> Self {
        let c: char = val.into();
        c as u8
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        write!(f, "{}", c)
    }
}
#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn test_count_ccw_turns() {
        assert_eq!(Direction::N.ccw_turn_difference(Direction::N), 0);
        assert_eq!(Direction::N.ccw_turn_difference(Direction::W), 1);
        assert_eq!(Direction::N.ccw_turn_difference(Direction::S), 2);
        assert_eq!(Direction::N.ccw_turn_difference(Direction::E), 3);
    }

    #[test]
    fn test_count_cw_turns() {
        assert_eq!(Direction::N.cw_turn_difference(Direction::N), 0);
        assert_eq!(Direction::N.cw_turn_difference(Direction::W), 3);
        assert_eq!(Direction::N.cw_turn_difference(Direction::S), 2);
        assert_eq!(Direction::N.cw_turn_difference(Direction::E), 1);
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
    pub fn minus(&self, direction: &Direction) -> RowCol {
        self.plus_n(&direction.turn_180(), 1)
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
impl Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X={},Y={}", self.x(), self.y())
    }
}

