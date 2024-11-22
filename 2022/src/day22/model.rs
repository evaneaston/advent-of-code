use crate::common::RowCol;

#[derive(Debug, Clone)]
pub(super) enum Move {
    Forward(u64),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Orientation {
    North,
    South,
    East,
    West,
}
impl Orientation {
    pub(super) fn numeric_value(&self) -> u64 {
        match self {
            Self::East => 0,
            Self::South => 1,
            Self::West => 2,
            Self::North => 3,
        }
    }
    pub(super) fn left(&self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }
    pub(super) fn right(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
}

impl RowCol {
    pub(super) fn next_position(&self, orientation: Orientation) -> RowCol {
        match orientation {
            Orientation::North => RowCol::new(self.row() - 1, self.col()),
            Orientation::South => RowCol::new(self.row() + 1, self.col()),
            Orientation::East => RowCol::new(self.row(), self.col() + 1),
            Orientation::West => RowCol::new(self.row(), self.col() - 1),
        }
    }
}
