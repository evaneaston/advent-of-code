use crate::{get_num_interior_points, AocError, DailyInput, RowCol, XY};

#[derive(Debug)]
enum Direction {
    R,
    L,
    D,
    U,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::R,
            "D" => Self::D,
            "L" => Self::L,
            "U" => Self::U,
            s => panic!("Unknown direction {s}"),
        }
    }
}
fn parse(line: &str) -> (Direction, usize, String) {
    let parts = line.split(' ').collect::<Vec<_>>();
    assert_eq!(parts.len(), 3, "Line should have three parts: {:?}", parts);
    (
        Direction::from(parts[0]),
        parts[1].parse::<usize>().unwrap(),
        parts[2].to_owned(),
    )
}

fn build_trench(i: impl Iterator<Item = (Direction, usize)>) -> Vec<RowCol> {
    let mut coords = i.fold(vec![RowCol::new(0, 0)], |mut acc, (direction, distance)| {
        let closure: fn(RowCol) -> RowCol = match direction {
            Direction::R => |rc| rc.plus_col(),
            Direction::L => |rc| rc.minus_col(),
            Direction::D => |rc| rc.plus_row(),
            Direction::U => |rc| rc.minus_row(),
        };
        (0..distance).for_each(|_| {
            let current = acc.last().unwrap();
            let next = closure(*current);
            acc.push(next)
        });
        acc
    });
    if coords.last() == coords.first() {
        coords.remove(coords.len() - 1);
    }
    coords
}

pub fn part1(input: DailyInput) -> Result<String, AocError> {
    let trench_rcs = build_trench2(
        input
            .get_input_lines()?
            .iter()
            .map(|line| parse(line))
            .map(|(direction, distance, _color)| (direction, distance)),
    );
    let trench_xys = trench_rcs.iter().map(|&rc| XY::from(rc)).collect::<Vec<_>>();
    let picks = get_num_interior_points(&trench_xys);
    println!(" {picks}");
    let total = picks.num_interior_points + picks.num_boundary_points;
    Ok(total.to_string())
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let lines = input.get_input_lines()?;
    let trench_rcs = build_trench2(
        lines.iter().map(|line| parse(line)).map(|(_direction, _distance, color)| {
            assert_eq!(color.len(), 9);
            let distance = i64::from_str_radix(&color[2..7], 16).unwrap() as usize;
            let direction: Direction = match &color[7..8] {
                // 0 means R, 1 means D, 2 means L, and 3 means U.
                "0" => Direction::R,
                "1" => Direction::D,
                "2" => Direction::L,
                "3" => Direction::U,
                c => panic!("Unexpected direction {c}"),
            };
            (direction, distance)
        }),
    );
    println!("trench len ={}", trench_rcs.len());
    println!("trench ={:?}", trench_rcs);
    let trench_xys = trench_rcs.iter().map(|&rc| XY::from(rc)).collect::<Vec<_>>();
    let picks = get_num_interior_points(&trench_xys);
    println!(" {picks}");
    let total = picks.num_interior_points + picks.num_boundary_points;
    Ok(total.to_string())
}

fn stepper(direction: Direction) -> fn(RowCol) -> RowCol {
    match direction {
        Direction::R => |rc| rc.plus_col(),
        Direction::L => |rc| rc.minus_col(),
        Direction::D => |rc| rc.plus_row(),
        Direction::U => |rc| rc.minus_row(),
    }
}
fn build_trench2(i: impl Iterator<Item = (Direction, usize)>) -> Vec<RowCol> {
    let mut coords = i.fold(vec![RowCol::new(0, 0)], |mut acc, (direction, distance)| {
        let steppr = stepper(direction);
        let next_vertex = (0..distance).fold(*acc.last().unwrap(), |current, _| steppr(current));
        acc.push(next_vertex);
        acc
    });
    if coords.last() == coords.first() {
        coords.remove(coords.len() - 1);
    }
    coords
}

#[cfg(test)]
mod test {
    use crate::{
        day18::{part1, part2},
        DailyInput, InputType,
    };

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1(DailyInput {
                day: 18,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "62"
        );
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(
            part1(DailyInput {
                day: 18,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            ""
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(DailyInput {
                day: 18,
                input_type: InputType::Example,
                number: None,
            })
            .unwrap(),
            "952408144115"
        );
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(
            part2(DailyInput {
                day: 18,
                input_type: InputType::Challenge,
                number: None,
            })
            .unwrap(),
            "129849166997110"
        );
    }
}
