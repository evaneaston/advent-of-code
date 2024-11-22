use std::collections::HashMap;

use super::{model::Orientation, parser::load_inputs};
use crate::{
    common::{InputType, RowCol},
    day22::{apply_moves, find_start, get_cube_edge_size, password},
};

#[test]
fn test_part1() {
    let (grid, moves) = load_inputs(InputType::Example).unwrap();

    println!("Grid:\n{}", grid);
    println!("Moves: {:?}", moves);

    assert_eq!(find_start(&grid), Some(RowCol::new(1, 9)));

    let (final_position, final_orientation) = apply_moves(&grid, &moves);
    println!(
        "Final posltion & orientiation: {:?} {:?}",
        final_position, final_orientation
    );

    assert_eq!(final_position, RowCol::new(6, 8));

    assert_eq!(password(&final_position, &final_orientation), 6032);
}

#[test]
fn test_part2() {
    let (grid, moves) = load_inputs(InputType::Example).unwrap();

    println!("Grid:\n{}", grid);
    println!("Moves: {:?}", moves);

    let cube_edge_length = get_cube_edge_size(&grid);
    println!("Cube Edge Length: {:?}", cube_edge_length);

    assert_eq!(cube_edge_length, 4);
    
    let cube_faces = vec![
        Face::new(RowCol::new(1, 9), cube_edge_length),
        Face::new(RowCol::new(5, 9), cube_edge_length),
        Face::new(RowCol::new(5, 5), cube_edge_length),
        Face::new(RowCol::new(5, 1), cube_edge_length),
        Face::new(RowCol::new(9, 9), cube_edge_length),
        Face::new(RowCol::new(13, 9), cube_edge_length),
    ];

    let cube = Cube{
        faces:cube_faces,
        face_edges: [
            (Edge::new(0, Orientation::North), Edge::new(3, Orientation::North)),
            (Edge::new(0, Orientation::West), Edge::new(2, Orientation::North)),
            (Edge::new(0, Orientation::South), Edge::new(1, Orientation::North)),
            (Edge::new(0, Orientation::East), Edge::new(5, Orientation::East)),

            (Edge::new(1, Orientation::West), Edge::new(2, Orientation::East)),
            (Edge::new(1, Orientation::South), Edge::new(4, Orientation::North)),
            (Edge::new(1, Orientation::East), Edge::new(5, Orientation::North)),

        ].iter().map(|e| *e).collect()
    };
}

struct Cube {
    faces: Vec<Face>,
    face_edges: HashMap<Edge, Edge>,
}

#[derive(Debug,Clone,Copy,Hash)]
struct Edge {
    face_index: usize,
    side: Orientation
}
impl Edge {
    fn new(face_index:usize, side:Orientation) -> Self {
        Self { face_index,side}
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.face_index == other.face_index && self.side == other.side
    }
}
impl Eq for Edge {

}
struct Face {
    top_left: RowCol,
    bottom_right: RowCol,
}
impl Face {
    fn new(top_left: RowCol, cube_edge_length: usize) -> Self {
        let cube_edge_length: i64 = cube_edge_length.try_into().unwrap();
        Self {
            top_left,
            bottom_right: RowCol::new(
                top_left.row() + cube_edge_length - 1,
                top_left.col() + cube_edge_length - 1,
            ),
        }
    }
    fn is_in(&self, rc: &RowCol) -> bool {
        rc.row() >= self.top_left.row()
            && rc.col() >= self.top_left.col()
            && rc.row() <= self.bottom_right.row()
            && rc.col() <= self.bottom_right.col()
    }
}
