use flexi_logger::Level;
use log::{log, log_enabled};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    iter::repeat,
    ops::Range,
};

use crate::RowCol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    rows: usize,
    cols: usize,
    min: RowCol,
    max: RowCol,
    data: Vec<u8>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new_repeating(min: RowCol, max: RowCol, fill: u8) -> Grid {
        let rows: usize = (max.row() - min.row() + 1).try_into().unwrap();
        let cols: usize = (max.col() - min.col() + 1).try_into().unwrap();

        let size = rows.checked_mul(cols).unwrap(); // panics if too big
        let mut data = Vec::<u8>::with_capacity(size);
        data.extend(repeat(fill).take(size));

        Grid {
            rows,
            cols,
            min,
            max,
            data,
        }
    }

    /**
     *  Create new instance from Vec of rows, assuming all row data is ascii.
     * If rows have differing lines grid is filled as
     */
    pub fn new(lines: &Vec<String>) -> Self {
        Self::new_offset(RowCol::new(0, 0), lines)
    }

    /**
     *  Create new instance from Vec of rows, assuming all row data is ascii.
     * If rows have differing lines grid is filled as
     */
    pub fn new_offset(min: RowCol, lines: &Vec<String>) -> Self {
        let rows = lines.len();
        if rows == 0 {
            panic!("Cannot support zero rows")
        }
        let max_cols: usize = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        if max_cols == 0 {
            panic!("Cannot support zero cols")
        }
        let cols = max_cols;
        let size = rows.checked_mul(cols).unwrap();
        let mut data = Vec::with_capacity(size);

        for line in lines {
            let row_data = line.as_bytes();
            data.extend_from_slice(row_data);
            if row_data.len() < cols {
                data.extend(repeat(0u8).take(cols - row_data.len()));
            }
        }

        Grid {
            rows,
            cols,
            min,
            max: RowCol::new(min.row() + rows as i64 - 1, min.col() + cols as i64 - 1),
            data,
        }
    }

    pub fn get(&self, rc: RowCol) -> Option<u8> {
        if rc.row() < self.min.row() {
            return None;
        }
        if rc.row() > self.max.row() {
            return None;
        }
        if rc.col() < self.min.col() {
            return None;
        }
        if rc.col() > self.max.col() {
            return None;
        }

        self._get(rc)
    }

    pub fn set(&mut self, rc: RowCol, value: u8) {
        if !self.is_in_window(rc) {
            panic!("Set at {:?} outside of range {:?}-{:?}", rc, self.min, self.max);
        }

        self._set(rc, value);
    }

    // pub fn try_set(&mut self, rc: RowCol, value: u8) -> Result<(), AocError> {
    //     if !self.is_in_window(rc) {
    //         return Err(AocError::OutOfRange(format!(
    //             "Set at {:?} outside of range {:?}..={:?}",
    //             rc, self.min, self.max
    //         )));
    //     }

    //     self._set(rc, value);
    //     Ok(())
    // }

    fn is_in_window(&self, rc: RowCol) -> bool {
        !(rc.row() < self.min.row()
            && rc.row() > self.max.row()
            && rc.col() < self.min.col()
            && rc.col() > self.max.col())
    }

    fn _get(&self, rc: RowCol) -> Option<u8> {
        let zero_based_rc = self.to_zero_based(rc);
        let index = self.index_of(zero_based_rc);
        self.data.get(index).copied()
    }

    fn _set(&mut self, rc: RowCol, value: u8) {
        let zero_based_rc = self.to_zero_based(rc);
        let index = self.index_of(zero_based_rc);
        if let Some(rv) = self.data.get_mut(index) {
            *rv = value;
        }
    }

    fn to_zero_based(&self, rc: RowCol) -> RowCol {
        let result = RowCol::new(self.to_zero_based_row(rc.row()), self.to_zero_based_col(rc.col()));
        if result.row() < 0 || result.col() < 0 {
            panic!(
                "Conversion of {:?} to zero-based {:?} failed.  Somethings negative when it shouldn't be.  self={:?}",
                rc, result, self
            );
        }
        result
    }

    fn to_zero_based_row(&self, row: i64) -> i64 {
        row - self.min.row()
    }

    fn to_zero_based_col(&self, col: i64) -> i64 {
        col - self.min.col()
    }

    fn index_of(&self, zero_based_rc: RowCol) -> usize {
        (zero_based_rc.row() * self.cols as i64 + zero_based_rc.col())
            .try_into()
            .unwrap()
    }

    pub fn fill_horizontal(&mut self, row: i64, col_range: Range<i64>, fill_with: u8) {
        let from = col_range.start.min(col_range.end);
        let through = col_range.start.max(col_range.end);

        for col in from..=through {
            self.set((row, col).into(), fill_with);
        }
    }

    pub fn fill_vertical(&mut self, row_range: Range<i64>, col: i64, fill_with: u8) {
        let from = row_range.start.min(row_range.end);
        let through = row_range.start.max(row_range.end);

        for row in from..=through {
            self.set((row, col).into(), fill_with);
        }
    }

    /**
     * Search for all occurrences of any of the chars, returning them all in a HashMap with values containing all of their [RowCol] (no specific ordering).
     */
    pub fn find(&self, chars: HashSet<u8>) -> HashMap<u8, Vec<RowCol>> {
        let mut result: HashMap<u8, Vec<RowCol>> = HashMap::new();

        for index in 0..self.data.len() {
            let b = self.data[index];
            if chars.contains(&b) {
                let col = index % self.cols;
                let row = (index - col) / self.cols;
                let pos = RowCol::new(row as i64, col as i64);
                if let Some(vec) = result.get_mut(&b) {
                    vec.push(pos);
                } else {
                    result.insert(b, Vec::from([pos]));
                }
            }
        }

        result
    }

    pub fn row_count(&self) -> usize {
        self.rows
    }

    pub fn col_count(&self) -> usize {
        self.cols
    }

    pub fn min(&self) -> RowCol {
        self.min
    }

    pub fn min_row(&self) -> i64 {
        self.min.row()
    }

    pub fn min_col(&self) -> i64 {
        self.min.col()
    }

    pub fn max(&self) -> RowCol {
        self.max
    }

    pub fn max_row(&self) -> i64 {
        self.max.row()
    }

    pub fn max_col(&self) -> i64 {
        self.max.col()
    }

    pub fn log_moves_over_self(&self, level: Level, path: &Vec<RowCol>) {
        if log_enabled!(level) {
            let mut log_grid = self.clone();
            log_grid.log_moves(level, path);
        }
    }

    pub fn log_moves_over_dots(&self, level: Level, path: &Vec<RowCol>) {
        if log_enabled!(level) {
            let mut log_grid = Grid::new_repeating(self.min(), self.max(), b'.');
            log_grid.log_moves(level, path);
        }
    }

    fn log_moves(&mut self, level: Level, path: &Vec<RowCol>) {
        if log_enabled!(level) {
            for i in 0..(path.len() - 1) {
                let from = path[i];
                let to = path[i + 1];

                if from.row() < to.row() {
                    self.set(from, b'v');
                } else if from.row() > to.row() {
                    self.set(from, b'^');
                } else if from.col() < to.col() {
                    self.set(from, b'>');
                } else if from.col() > to.col() {
                    self.set(from, b'<');
                }
            }

            log!(level, "Moves: \n{}", &self);
            log!(level, "Path Length={}", path.len() - 1);
        }
    }

    pub fn transpose(&self) -> Self {
        let mut new_data = vec![0; self.data.len()];
        self.data.iter().enumerate().for_each(|(index, v)| {
            let row_index = index / self.col_count();
            let col_index = index % self.col_count();
            let new_index = col_index * self.row_count() + row_index;
            *new_data.get_mut(new_index).unwrap() = *v;
        });
        Self {
            rows: self.cols,
            cols: self.rows,
            min: RowCol::new(self.min.col(), self.min.row()),
            max: RowCol::new(self.max.col(), self.max.row()),
            data: new_data,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{}, {}]-[{}, {}]:",
            self.min.row(),
            self.min.col(),
            self.max.row(),
            self.max.col()
        )?;
        for row in 0..self.rows {
            let start = row * self.cols;
            writeln!(
                f,
                "{}",
                self.data[start..(start + self.cols)]
                    .iter()
                    .map(|n| *n as char)
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    #[test]
    fn test_raw() {
        let grid = Grid {
            min: (0, 0).into(),
            max: (1, 2).into(),
            rows: 2,
            cols: 3,
            data: vec![b'a', b'b', b'c', b'd', b'e', b'f'],
        };

        assert_eq!(grid.get((0, 0).into()), Some(b'a'));
        assert_eq!(grid.get((0, 1).into()), Some(b'b'));
        assert_eq!(grid.get((0, 2).into()), Some(b'c'));
        assert_eq!(grid.get((1, 0).into()), Some(b'd'));
        assert_eq!(grid.get((1, 1).into()), Some(b'e'));
        assert_eq!(grid.get((1, 2).into()), Some(b'f'));

        assert_eq!(grid.get((0, 3).into()), None);
        assert_eq!(grid.get((1, 3).into()), None);
        assert_eq!(grid.get((2, 0).into()), None);
    }

    #[test]
    fn test_new() {
        let grid = Grid::new(&vec!["ABC".to_string(), "DEF".to_string()]);
        println!("grid={}", grid);

        assert_eq!(grid.get((0, 0).into()), Some(b'A'));
        assert_eq!(grid.get((0, 1).into()), Some(b'B'));
        assert_eq!(grid.get((0, 2).into()), Some(b'C'));
        assert_eq!(grid.get((1, 0).into()), Some(b'D'));
        assert_eq!(grid.get((1, 1).into()), Some(b'E'));
        assert_eq!(grid.get((1, 2).into()), Some(b'F'));

        assert_eq!(grid.get((0, 3).into()), None);
        assert_eq!(grid.get((1, 3).into()), None);
        assert_eq!(grid.get((2, 0).into()), None);
    }

    #[test]
    fn test_new_with() {
        let grid = Grid::new_repeating((0, 0).into(), (2, 1).into(), b'.');
        println!("grid: {:?}", grid);

        assert_eq!(grid.get((0, 0).into()), Some(b'.'));
        assert_eq!(grid.get((0, 1).into()), Some(b'.'));
        assert_eq!(grid.get((0, 2).into()), None);
        assert_eq!(grid.get((1, 0).into()), Some(b'.'));
        assert_eq!(grid.get((1, 1).into()), Some(b'.'));
        assert_eq!(grid.get((1, 2).into()), None);
        assert_eq!(grid.get((2, 0).into()), Some(b'.'));
        assert_eq!(grid.get((2, 1).into()), Some(b'.'));
        assert_eq!(grid.get((2, 2).into()), None);
    }

    #[test]
    fn test_set() {
        let mut grid = Grid::new_repeating((0, 0).into(), (9, 9).into(), b'.');
        grid.set((0, 0).into(), 100);
        grid.set((0, 1).into(), 101);
        grid.set((0, 9).into(), b'^');

        grid.set((1, 0).into(), 102);
        grid.set((1, 1).into(), 103);
        grid.set((1, 9).into(), b'^');

        grid.set((9, 0).into(), 100);
        println!("{}", grid);
    }

    #[test]
    fn test_grid() {
        let mut wg = Grid::new_repeating((5, 10).into(), (20, 60).into(), b'X');
        assert_eq!(wg.get((5, 10).into()), Some(b'X'));

        wg.set((6, 11).into(), b'Y');

        assert_eq!(wg.get((6, 11).into()), Some(b'Y'));

        wg.set((5, 10).into(), b'1');
        wg.set((5, 60).into(), b'2');
        wg.set((20, 60).into(), b'3');
        wg.set((20, 10).into(), b'4');

        println!("{}", wg);
    }

    #[test]
    fn test_transpose() {
        let lines = "abcdef\n\
        ghijkl\n\
        mnopqr";

        let expected_transposed = "agm\n\
            bhn\n\
            cio\n\
            djp\n\
            ekq\n\
            flr";

        let grid = Grid::new(&lines.split('\n').map(|s| s.trim().to_string()).collect());
        let expected_grid = Grid::new(&expected_transposed.split('\n').map(|s| s.trim().to_string()).collect());

        assert_eq!(grid.transpose(), expected_grid);
    }
}
