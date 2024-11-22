use crate::common::{AocError, InputType};
use log::debug;
use std::io::Error;

#[derive(Debug)]
struct Trees {
    data: Vec<String>,
    rows: usize,
    columns: usize,
}

impl Trees {
    fn get(&self, row: usize, column: usize) -> u8 {
        let a = self.data.get(row).unwrap();
        let b = &a[column..column + 1];
        b.parse().unwrap()
    }

    fn assert_valid_row_col(&self, row: usize, col: usize) {
        assert!(
            row < self.rows && col < self.columns,
            "Value row={}, col={} outside of range row∈[0..{}) and col∈[0..{})",
            row,
            col,
            self.rows,
            self.columns
        );
    }

    fn get_north_of(&self, row: usize, col: usize) -> Vec<u8> {
        self.assert_valid_row_col(row, col);

        let mut result = Vec::with_capacity(row);
        if row == 0 {
            return result;
        }

        for r in (0..row).rev() {
            result.push(self.get(r, col));
        }
        result
    }

    fn get_south_of(&self, row: usize, col: usize) -> Vec<u8> {
        self.assert_valid_row_col(row, col);

        let mut result = Vec::with_capacity(self.rows - row);

        if row == self.rows - 1 {
            return result;
        }

        for r in row + 1..self.rows {
            result.push(self.get(r, col));
        }
        result
    }

    fn get_east_of(&self, row: usize, col: usize) -> Vec<u8> {
        self.assert_valid_row_col(row, col);

        let mut result = Vec::with_capacity(self.columns - col);

        if col == self.columns - 1 {
            return result;
        }

        for c in col + 1..self.columns {
            result.push(self.get(row, c));
        }
        result
    }

    fn get_west_of(&self, row: usize, col: usize) -> Vec<u8> {
        self.assert_valid_row_col(row, col);

        let mut result = Vec::with_capacity(col);

        if col == 0 {
            return result;
        }

        for c in (0..col).rev() {
            result.push(self.get(row, c));
        }
        result
    }
}

fn load_trees() -> Result<Trees, Error> {
    let lines = InputType::Challenge.get_input_lines(8)?;
    let non_empty_lines: Vec<String> = lines.iter().filter(|l| !l.is_empty()).cloned().collect();
    let rows = non_empty_lines.len();
    assert!(rows > 1);

    let mut columns: Option<usize> = None;
    for line in &non_empty_lines {
        match columns {
            Some(cols) => assert!(cols == line.len()),
            None => {
                columns = Some(line.len());
            }
        };
    }

    Ok(Trees {
        data: non_empty_lines,
        rows: rows,
        columns: columns.unwrap(),
    })
}

pub fn part1() -> Result<String, AocError> {
    let trees = load_trees()?;
    Ok(part_one(&trees))
}

pub fn part2() -> Result<String, AocError> {
    let trees = load_trees()?;
    Ok(part_two(&trees))
}

fn part_one(trees: &Trees) -> String {
    let mut num_visible = 0;
    for row in 0..trees.rows {
        for col in 0..trees.columns {
            let height = trees.get(row, col);

            let less_than = |h: &u8| -> bool { *h < height };

            let is_visible = trees.get_north_of(row, col).iter().all(&less_than)
                || trees.get_south_of(row, col).iter().all(less_than)
                || trees.get_east_of(row, col).iter().all(less_than)
                || trees.get_west_of(row, col).iter().all(less_than);
            if is_visible {
                num_visible += 1;
            }
            // debug!(
            //     "{}x{} = {}",
            //     row,
            //     col,
            //     if is_visible { "visible" } else { "NOT visible" }
            // )
        }
    }
    format!("# visible ={}", num_visible)
}

fn viewing_distance(from_height: &u8, heights: &Vec<u8>) -> usize {
    let mut num_visible = 0;
    for h in heights {
        num_visible += 1;
        if h >= from_height {
            break;
        }
    }
    return num_visible;
}

fn part_two(trees: &Trees) -> String {
    let mut max_scenic_score = 0;
    for row in 0..trees.rows {
        for col in 0..trees.columns {
            let height = trees.get(row, col);

            let north_viewing_distance = viewing_distance(&height, &trees.get_north_of(row, col));
            let south_viewing_distance = viewing_distance(&height, &trees.get_south_of(row, col));
            let east_viewing_distance = viewing_distance(&height, &trees.get_east_of(row, col));
            let west_viewing_distance = viewing_distance(&height, &trees.get_west_of(row, col));

            let scenic_score = north_viewing_distance.max(1)
                * south_viewing_distance.max(1)
                * east_viewing_distance.max(1)
                * west_viewing_distance.max(1);

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
                debug!(
                    "New Max of {} at {}x{} ({}x{}x{}x{})",
                    max_scenic_score,
                    row,
                    col,
                    north_viewing_distance.max(1),
                    south_viewing_distance.max(1),
                    east_viewing_distance.max(1),
                    west_viewing_distance.max(1)
                );
            }
        }
    }
    format!("max scenic score={}", max_scenic_score)
}

#[cfg(test)]
mod tests {
    use crate::day08::{load_trees, viewing_distance};

    #[test]
    fn test_trees_load() {
        let trees = load_trees().unwrap();
        assert!(trees.rows == 99);
        assert!(trees.columns == 99);
        assert!(trees.data.len() == 99);
    }

    #[test]
    fn test_get() {
        let trees = load_trees().unwrap();
        assert!(trees.get(0, 0) == 2);
        assert!(trees.get(0, 98) == 0);
        assert!(trees.get(98, 0) == 2);
        assert!(trees.get(98, 98) == 0);
    }

    #[test]
    fn test_get_north_of() {
        let trees = load_trees().unwrap();
        assert!(trees.get_north_of(0, 0).len() == 0);
        assert!(trees.get_north_of(1, 1) == vec![1]);
        assert!(trees.get_north_of(3, 8) == vec![1, 2, 2]);
        assert!(trees.get_north_of(15, 56) == vec![4, 6, 5, 5, 2, 2, 4, 2, 5, 4, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_get_south_of() {
        let trees = load_trees().unwrap();
        assert!(trees.get_south_of(96, 2) == vec![2, 0]);
        assert!(trees.get_south_of(98, 15) == vec![]);
        assert!(trees.get_south_of(90, 31) == vec![5, 2, 2, 4, 5, 1, 1, 2]);
    }

    #[test]
    fn test_get_east_of() {
        let trees = load_trees().unwrap();
        assert!(trees.get_east_of(0, 0).len() == 98);
        assert!(trees.get_east_of(20, 40).len() == 58);
        assert!(trees.get_east_of(65, 92) == vec![1, 5, 3, 3, 3, 1]);
        assert!(trees.get_east_of(50, 98) == vec![]);
    }

    #[test]
    fn test_get_west_of() {
        let trees = load_trees().unwrap();
        assert!(trees.get_west_of(0, 0).len() == 0);
        assert!(trees.get_west_of(20, 40).len() == 40);
        assert!(trees.get_west_of(66, 10) == vec![4, 3, 5, 1, 3, 3, 4, 2, 2, 4]);
        assert!(trees.get_west_of(50, 0) == vec![]);
        assert!(trees.get_west_of(50, 1) == vec![2]);
    }

    #[test]
    fn test_viewing_distance() {
        assert!(viewing_distance(&5, &vec![5, 5, 5, 4, 3, 2]) == 1);
        assert!(viewing_distance(&5, &vec![6, 6, 5, 4, 3, 2]) == 1);
        assert!(viewing_distance(&5, &vec![5, 3, 5, 4, 3, 2]) == 1);
        assert!(viewing_distance(&5, &vec![3, 5, 4, 3, 2]) == 2);
        assert!(viewing_distance(&5, &vec![5, 4, 3, 2]) == 1);
        assert!(viewing_distance(&5, &vec![4, 3, 2]) == 3);
        assert!(viewing_distance(&5, &vec![]) == 0);
    }
}
