use log::{info, debug};

use crate::{
    common::{AocError, InputType, RowCol},
    grid::Grid,
};

use self::{
    model::{Move, Orientation},
    parser::load_inputs,
};
mod model;
mod parser;

#[cfg(test)]
mod tests;

pub fn part1() -> Result<String, AocError> {
    let (grid, moves) = load_inputs(InputType::Challenge)?;
    info!("{}", grid);
    let (final_position, final_orientation) = apply_moves(&grid, &moves);
    let answer = password(&final_position, &final_orientation);

    assert_eq!(answer, 57350);

    Ok(format!("{}", answer))
}

pub fn part2() -> Result<String, AocError> {
    Ok(format!("{}", ""))
}

fn apply_moves(grid: &Grid, moves: &Vec<Move>) -> (RowCol, Orientation) {
    let mut positions = vec![];

    let mut pos = find_start(&grid).expect("Couldn't find starting position");
    let mut orientation = Orientation::East;

    positions.push(pos);
    for m in moves {
        match m {
            Move::Left => orientation = orientation.left(),
            Move::Right => orientation = orientation.right(),
            Move::Forward(count) => pos = forward(&grid, &pos, orientation, *count, &mut positions),
        }
    }

    debug!("Moves: {:?}", positions);
    grid.log_moves_over_self(flexi_logger::Level::Info, &positions);

    (pos, orientation)
}

fn forward(
    grid: &Grid,
    from_pos: &RowCol,
    orientation: Orientation,
    count: u64,
    positions: &mut Vec<RowCol>,
) -> RowCol {
    let mut pos = *from_pos;
    for _ in 0..count {
        let next_pos = next_position(&grid, pos, orientation);
        if next_pos == pos {
            break;
        }
        pos = next_pos;
        positions.push(pos);
    }
    pos
}

fn next_position(grid: &Grid, pos: RowCol, orientation: Orientation) -> RowCol {
    let mut p = pos;

    loop {
        let possible_next = p.next_position(orientation);
        match grid.get(possible_next) {
            Some(c) => {
                if c == b'.' {
                    return possible_next;
                } else if c == b'#' {
                    return pos;
                } else if c == b' ' || c == 0 {
                    p = possible_next
                } else {
                    panic!("Unrecognized character {}", c);
                }
            }
            None => {
                if possible_next.col() < grid.min().col() {
                    // went off left
                    p = RowCol::new(possible_next.row(), grid.max().col() + 1);
                } else if possible_next.col() > grid.max().col() {
                    // went off right
                    p = RowCol::new(possible_next.row(), grid.min().col() - 1);
                } else if possible_next.row() < grid.min().row() {
                    // went off top
                    p = RowCol::new(grid.max().row() + 1, possible_next.col());
                } else if possible_next.row() > grid.max().row() {
                    // went off bottom
                    p = RowCol::new(grid.min().row() - 1, possible_next.col());
                } else {
                    panic!("Not sure how this happened")
                }
            }
        }
    }
}

fn find_start(grid: &Grid) -> Option<RowCol> {
    assert!(
        grid.row_count() > 0,
        "Bad input.  Grid doesn't have a first row"
    );
    find_first_on_row(&grid, 1, b'.')
}

fn find_first_on_row(grid: &Grid, row: i64, char: u8) -> Option<RowCol> {
    let first = grid.min().col();
    let last = grid.max().col();
    for col in first..=last {
        let candidate = RowCol::new(row, col);
        if grid.get(candidate) == Some(char) {
            return Some(candidate);
        }
    }
    None
}

// The final password is the sum of 1000 times the row, 4 times the column, and the facing.
fn password(position: &RowCol, orientation: &Orientation) -> i64 {
    let on: i64 = orientation.numeric_value().try_into().unwrap();
    1000 * position.row() + 4 * position.col() + on
}

fn get_cube_edge_size(grid: &Grid) -> usize {
    let width: usize = (grid.max().col() - grid.min().col() + 1).try_into().unwrap();
    let height: usize = (grid.max().row() - grid.min().row() + 1).try_into().unwrap();

    match ((width as f64) / (height as f64) * 1000.0).trunc() / 1000.0 {
        ar if ar == 0.750 || ar == 1.333 => width.min(height) / 3,
        ar => panic!("unsupported aspect ratio of {}", ar),
    }    
}