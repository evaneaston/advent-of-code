mod parse;
mod sim;
use self::parse::PolyLine;
use crate::{
    common::{AocError, RowCol},
    grid::Grid,
};
use log::{debug, info};

const SAND_SOURCE: u8 = b'+';
const ROCK: u8 = b'#';
const SAND: u8 = b'o';
const EMPTY: u8 = b'.';

fn get_range(sand_source_location: RowCol, vec_lines: &Vec<PolyLine>) -> (RowCol, RowCol) {
    let mut min_row = sand_source_location.row();
    let mut min_col = sand_source_location.col();
    let mut max_row = sand_source_location.row();
    let mut max_col = sand_source_location.col();
    for line_def in vec_lines {
        for vertex in &line_def.0 {
            if vertex.row() < min_row {
                min_row = vertex.row();
            }
            if vertex.row() > max_row {
                max_row = vertex.row();
            }
            if vertex.col() < min_col {
                min_col = vertex.col();
            }
            if vertex.col() > max_col {
                max_col = vertex.col();
            }
        }
        debug!("{:?}", line_def);
    }
    ((min_row, min_col).into(), (max_row, max_col).into())
}

fn add_rock(grid: &mut Grid, rock_lines: &Vec<PolyLine>) {
    for rock_line in rock_lines {
        for i in 0..rock_line.0.len() - 1 {
            let from = rock_line.0[i];
            let to = rock_line.0[i + 1];

            if from.row() == to.row() {
                grid.fill_horizontal(from.row(), from.col()..to.col(), ROCK);
            } else if from.col() == to.col() {
                grid.fill_vertical(from.row()..to.row(), from.col(), ROCK);
            } else {
                panic!(
                    "Cannot support non-vertical/horizontal segments {:?}-{:?}",
                    from, to
                );
            }
        }
    }
}

pub fn part1() -> Result<String, AocError> {
    let vec_lines = parse::parse_input()?;

    let sand_source_position = RowCol::new(0, 500);
    let (min, max) = get_range(sand_source_position, &vec_lines);

    info!("Range for grid window: ({:?}) to ({:?})", min, max);

    let mut grid = Grid::new_repeating(min, max, EMPTY);
    grid.set(sand_source_position, SAND_SOURCE);
    add_rock(&mut grid, &vec_lines);

    info!("Grid Before:");
    info!("\n{}", grid);

    let mut sim = sim::SandSim {
        sand_source_position,
        pos: None,
        grid: &mut grid,
    };

    let mut count_come_to_rest = 0_usize;
    loop {
        match sim.drop_sand() {
            sim::SandDropResult::Jammed => break,
            sim::SandDropResult::CameToRestAt(_) => count_come_to_rest += 1,
            sim::SandDropResult::FellIntoAbyss => break,
        }
    }

    info!("Grid After:");
    info!("\n{}", grid);

    assert_eq!(count_come_to_rest, 578);

    Ok(format!("{}", count_come_to_rest))
}

pub fn part2() -> Result<String, AocError> {
    let vec_lines = parse::parse_input()?;

    let sand_source_position = RowCol::new(0, 500);
    let (min, max) = get_range(sand_source_position, &vec_lines);

    let base_width_extension = max.row() - min.row();
    let min = RowCol::new(min.row(), min.col() - base_width_extension);
    let max = RowCol::new(max.row() + 2, max.col() + base_width_extension);

    info!("Range for grid window: ({:?}) to ({:?})", min, max);

    let mut grid = Grid::new_repeating(min, max, EMPTY);
    grid.set(sand_source_position, SAND_SOURCE);
    add_rock(&mut grid, &vec_lines);
    grid.fill_horizontal(max.row(), min.col()..max.col() + 1, ROCK);

    info!("Grid Before:");
    info!("\n{}", grid);

    let mut sim = sim::SandSim {
        sand_source_position,
        pos: None,
        grid: &mut grid,
    };

    let mut count_come_to_rest = 0_usize;
    loop {
        match sim.drop_sand() {
            sim::SandDropResult::Jammed => {
                count_come_to_rest += 1;
                break;
            }
            sim::SandDropResult::CameToRestAt(_) => count_come_to_rest += 1,
            sim::SandDropResult::FellIntoAbyss => break,
        }
    }

    info!("Grid After:");
    info!("\n{}", grid);

    assert_eq!(count_come_to_rest, 24377);

    Ok(format!("{}", count_come_to_rest))
}
