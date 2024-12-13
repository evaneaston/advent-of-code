use crate::coord::XY;
use log::debug;
use std::collections::HashMap;
use std::fmt::Display;

// find intersection of two lines as specified by
//    a1 x + b1 y + c1 = 0
//    a2 x + b2 y + c2 = 0
pub(crate) fn line_intersection(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<XY> {
    let x: f64 = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
    if x.is_nan() {
        return None;
    }
    if x.trunc() != x {
        return None;
    }
    let x: i64 = num::cast(x)?;

    let y: f64 = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);
    // eprintln!("{} => {}, {} => {}", x, x.trunc(), y, y.trunc());
    if y.is_nan() {
        return None;
    }
    if y.trunc() != y {
        return None;
    }
    let y: i64 = num::cast(y)?;
    Some(XY(x, y))
}

#[allow(dead_code)]
pub(crate) fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(dead_code)]
pub(crate) fn lcm_of_two(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[allow(dead_code)]
pub(crate) fn lcm_of_multiple(numbers: &[u64]) -> u64 {
    numbers.iter().cloned().fold(1, lcm_of_two)
}

#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub(crate) fn get_num_interior_points(vertices: &[XY]) -> PicksResult {
    let vertices = &vertices;
    let area = shoelace_area(vertices);
    debug!(" shoelace area={area}");

    let mut boundary_points_not_in_vertices = 0_usize;

    let mut looped: Vec<XY> = Vec::from(*vertices);
    looped.push(looped[0]);

    for p in looped.windows(2) {
        let a = p[0];
        let b = p[1];
        if a.x() == b.x() {
            let num_missing = ((b.y() - a.y()).abs() - 1).max(0);
            debug!(" Between {:?} and {:?} there are {num_missing}", a, b);
            boundary_points_not_in_vertices += num_missing as usize;
        } else if a.y() == b.y() {
            let num_missing = ((b.x() - a.x()).abs() - 1).max(0);
            debug!(" Between {:?} and {:?} there are {num_missing}", a, b);
            boundary_points_not_in_vertices += num_missing as usize;
        } else {
            // todo if we ever need angled ones, find integer intersections
            panic!("Assumed no angled edges");
        }
    }

    debug!(" boundary_points_not_in_vertices={boundary_points_not_in_vertices}");

    let num_boundary_points = vertices.len() + boundary_points_not_in_vertices;
    let num_interior_points = area + 1_f64 - num_boundary_points as f64 / 2.;

    debug!(" num_boundary_points={num_boundary_points}");
    debug!(" num_interior_points={num_interior_points}");

    // num_interior_points.round() as i64
    PicksResult {
        shoelace_area: area,
        num_boundary_points,
        num_interior_points: num_interior_points.round() as usize,
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct PicksResult {
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
    use crate::algo::{get_num_interior_points, line_intersection, shoelace_area};
    use crate::coord::{RowCol, XY};

    #[test]
    fn test_line_intersection() {
        assert_eq!(
            line_intersection(
                94.into(),
                22.into(),
                (-8400).into(),
                34.into(),
                67.into(),
                (-5400).into()
            ),
            Some(XY(80, 40))
        );
        assert_eq!(
            line_intersection(3.into(), 4.into(), 5.into(), 2.into(), 5.into(), 7.into()),
            None
        );
    }

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
            ])
            .num_interior_points,
            4
        );
    }
}
