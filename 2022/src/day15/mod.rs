use crate::common::{AocError, InputType, XY};
use log::{debug, info};
use nom::{bytes::complete::tag, multi::separated_list1, sequence::tuple, IResult};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Beacon(XY);
impl Beacon {
    fn pos(&self) -> XY {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Sensor(XY);
impl Sensor {
    fn pos(&self) -> XY {
        self.0
    }
}

fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    tuple((
        tag("Sensor at x="),
        nom::character::complete::i64,
        tag(", y="),
        nom::character::complete::i64,
        tag(": closest beacon is at x="),
        nom::character::complete::i64,
        tag(", y="),
        nom::character::complete::i64,
    ))(input)
    .map(|(input, parts)| {
        (
            input,
            (
                Sensor(XY::new(parts.1, parts.3)),
                Beacon(XY::new(parts.5, parts.7)),
            ),
        )
    })
}

pub fn get_input(input_type: InputType) -> Result<Vec<(Sensor, Beacon)>, AocError> {
    let input = input_type.get_input_as_single_string(15)?;
    let result = match separated_list1(tag("\n"), parse_line)(&input) {
        Ok((remaining, parsed)) => {
            if !remaining.is_empty() {
                panic!(
                    "Unable to parse entire input.\nremaining={}\nparsed={:?}",
                    remaining, parsed
                );
            }
            parsed
        }
        Err(e) => panic!("Unable to parse entire {:?}", e),
    };

    debug!("Input={:?}", input);
    for (sensor, beacon) in &result {
        let distance = manhattan_distance(sensor.0, beacon.0);
        debug!(
            "Distance from {:?}  to {:?} is {}",
            sensor.0, beacon.0, distance
        );
    }

    Ok(result)
}

pub fn manhattan_distance(from: XY, to: XY) -> i64 {
    ((to.x() - from.x()).abs() + (to.y() - from.y()).abs())
        .try_into()
        .unwrap()
}

pub fn get_sensor_range_line_intersections(
    line_y: i64,
    input: &Vec<(Sensor, Beacon)>,
) -> HashMap<Sensor, (i64, i64)> {
    let mut intersections = HashMap::new();

    info!("Line Y: {}", line_y);

    for (sensor, beacon) in input {
        debug!("Sensor {:?}, Beacon {:?}", sensor, beacon);

        let sensor_beacon_distance = manhattan_distance(sensor.pos(), beacon.pos());
        debug!("  sensor beacon distance: {}", sensor_beacon_distance);

        let y_distance_from_line = (sensor.pos().y() - line_y).abs();

        if sensor_beacon_distance < y_distance_from_line {
            debug!("  Sensor too far from line to matter");
            continue;
        }

        let beacon_x_offset_if_on_line = sensor_beacon_distance - y_distance_from_line;
        let min_visible_x_on_line = sensor.pos().x() - beacon_x_offset_if_on_line;
        let max_visible_x_on_line = sensor.pos().x() + beacon_x_offset_if_on_line;

        debug!(
            "  Sensor can see from {} to {} on line at y={}",
            min_visible_x_on_line, max_visible_x_on_line, line_y
        );

        intersections.insert(
            sensor.clone(),
            (min_visible_x_on_line, max_visible_x_on_line),
        );
    }
    intersections
}

pub fn get_xs_of_beacons_on_line(line_y: i64, input: &Vec<(Sensor, Beacon)>) -> BTreeSet<i64> {
    input
        .iter()
        .map(|e| &e.1)
        .filter(|beacon| beacon.pos().y() == line_y)
        .map(|beacon| beacon.pos().x())
        .collect()
}

pub fn get_xs_of_sensors_on_line(line_y: i64, input: &Vec<(Sensor, Beacon)>) -> BTreeSet<i64> {
    input
        .iter()
        .map(|e| &e.0)
        .filter(|sensor| sensor.pos().y() == line_y)
        .map(|sensor| sensor.pos().x())
        .collect()
}

fn count_impossible_beacon_positions_on_line_y(
    line_y: i64,
    input: &Vec<(Sensor, Beacon)>,
) -> usize {
    let intersections = get_sensor_range_line_intersections(line_y, &input);

    let mut result = 0_usize;

    if intersections.is_empty() {
        debug!("No intersections on line y={}", line_y);
    } else {
        debug!("Intersections on line y={}: {:?}", line_y, intersections);

        let beacon_x_on_line = get_xs_of_beacons_on_line(line_y, &input);
        let sensor_x_on_line = get_xs_of_sensors_on_line(line_y, &input);

        let min_x = intersections
            .iter()
            .map(|(_sensor, range)| range.0)
            .min()
            .unwrap();
        let max_x = intersections
            .iter()
            .map(|(_sensor, range)| range.1)
            .max()
            .unwrap();

        debug!("Range to scan on line y={}: {}..={}", line_y, min_x, max_x);

        for x in min_x..=max_x {
            let is_x_inside_some_intersection =
                intersections.iter().any(|e| e.1 .0 <= x && x <= e.1 .1);

            if is_x_inside_some_intersection
                && !beacon_x_on_line.contains(&x)
                && !sensor_x_on_line.contains(&x)
            {
                result += 1;
            }
        }
    }
    result
}

fn slope_based_part2_solution(input: &Vec<(Sensor, Beacon)>, limit: i64) -> Option<XY> {
    let sensor_ranges: Vec<(XY, i64)> = input
        .iter()
        .map(|(sensor, beacon)| (sensor.pos(), manhattan_distance(sensor.pos(), beacon.pos())))
        .collect();

    let mut x_axis_shadows_from_plus_plus = Vec::<(i64, i64)>::new();
    for (sensor, beacon) in input {
        let distance = manhattan_distance(sensor.pos(), beacon.pos());
        let plus_plus_x_intercept = sensor.pos().x() - sensor.pos().y();
        x_axis_shadows_from_plus_plus.push((
            plus_plus_x_intercept - distance,
            plus_plus_x_intercept + distance,
        ));
    }

    x_axis_shadows_from_plus_plus.sort();

    debug!("intercepts: {:?}", x_axis_shadows_from_plus_plus);

    let mut strips = BTreeSet::<i64>::new();
    for (left, right) in x_axis_shadows_from_plus_plus {
        strips.insert(left - 1);
        strips.insert(right + 1);
    }

    for start_x in strips {
        debug!("Scan diagonally (slope 1) from {},0", start_x);
        let mut x = start_x;
        let mut y = 0;
        'inner: loop {
            if x >= 0 && x <= limit {
                if sensor_ranges.iter().all(|(sensor_xy, range)| {
                    manhattan_distance(XY::new(x, y), *sensor_xy) > *range
                }) {
                    info!("Found spot at {},{}", x, y);
                    return Some(XY::new(x, y));
                }
            }
            x += 1;
            y += 1;
            if x > limit || y > limit {
                break 'inner;
            }
        }
    }
    None
}

pub fn part1() -> Result<String, AocError> {
    let input = get_input(InputType::Challenge)?;

    let result = count_impossible_beacon_positions_on_line_y(2000000, &input);

    assert_eq!(result, 5166077);

    Ok(format!("{:?}", result))
}

pub fn part2() -> Result<String, AocError> {
    let input = get_input(InputType::Challenge)?;

    let limit = 4000000;
    let result = slope_based_part2_solution(&input, limit);

    assert_eq!(result, Some(XY::new(3267801, 2703981)));

    let result = result.unwrap();
    let result = result.x() * limit + result.y();

    assert_eq!(result, 13071206703981);

    Ok(format!("{:?}", result))
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{AocError, InputType},
        day15::{count_impossible_beacon_positions_on_line_y, get_input, manhattan_distance},
    };

    use super::slope_based_part2_solution;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0).into(), (0, 0).into()), 0);
        assert_eq!(manhattan_distance((1, 1).into(), (1, 1).into()), 0);
        assert_eq!(manhattan_distance((-1, -1).into(), (-1, -1).into()), 0);

        assert_eq!(manhattan_distance((0, 0).into(), (1, 1).into()), 2);
        assert_eq!(manhattan_distance((1, 1).into(), (0, 0).into()), 2);
        assert_eq!(manhattan_distance((-1, -1).into(), (0, 0).into()), 2);
        assert_eq!(manhattan_distance((0, 0).into(), (-1, -1).into()), 2);
        assert_eq!(manhattan_distance((-1, -1).into(), (1, 1).into()), 4);

        assert_eq!(manhattan_distance((-5, -2).into(), (1, -1).into()), 7);
    }

    #[test]
    fn test_example_part_1() -> Result<(), AocError> {
        let input = get_input(InputType::Example)?;

        assert_eq!(count_impossible_beacon_positions_on_line_y(9, &input), 25);
        assert_eq!(count_impossible_beacon_positions_on_line_y(10, &input), 26);
        assert_eq!(count_impossible_beacon_positions_on_line_y(11, &input), 27);

        Ok(())
    }

    #[test]
    fn test_example_part_2() -> Result<(), AocError> {
        let input = get_input(InputType::Example)?;
        slope_based_part2_solution(&input, 20);
        Ok(())
    }
}
