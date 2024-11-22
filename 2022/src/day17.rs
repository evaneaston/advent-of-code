use std::{collections::VecDeque, fmt::Display};

use log::{debug, info, trace};

use crate::common::{AocError, InputType};

const SHAPE1: u32 = 0b0000000 << 24 | 0b0000000 << 16 | 0b0000000 << 8 | 0b0011110;
const SHAPE2: u32 = 0b0000000 << 24 | 0b0001000 << 16 | 0b0011100 << 8 | 0b0001000;
const SHAPE3: u32 = 0b0000000 << 24 | 0b0000100 << 16 | 0b0000100 << 8 | 0b0011100;
const SHAPE4: u32 = 0b0010000 << 24 | 0b0010000 << 16 | 0b0010000 << 8 | 0b0010000;
const SHAPE5: u32 = 0b0000000 << 24 | 0b0000000 << 16 | 0b0011000 << 8 | 0b0011000;

pub fn part1() -> Result<String, AocError> {
    let input = InputType::Challenge.get_input_as_single_string(17)?;

    let mut sim = Sim::new(input.chars().collect(), shapes_vec());

    for _ in 0..2022 {
        sim.drop_shape();
    }

    let top_row = sim.top_rock_row.unwrap() + 1;

    assert_eq!(top_row, 3188);

    Ok(format!("{top_row}"))
}

pub fn part2() -> Result<String, AocError> {
    let input = InputType::Challenge.get_input_as_single_string(17)?;
    let result = run_large_sim(&input, 1000000000000_usize);

    assert_eq!(result, 1591977077342);

    Ok(format!("{result}"))
}

fn shapes_vec() -> Vec<u32> {
    vec![SHAPE1, SHAPE2, SHAPE3, SHAPE4, SHAPE5]
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}
impl Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Jet::Left => "<",
                Jet::Right => ">",
            }
        )
    }
}

struct Sim {
    jets: VecDeque<Jet>,
    shapes: VecDeque<u32>,
    stack: VecDeque<u8>,
    shape: u32,
    shape_row: usize,
    top_rock_row: Option<usize>,
}
impl Sim {
    fn new(jets: Vec<char>, shapes: Vec<u32>) -> Sim {
        let mut my_jets = VecDeque::with_capacity(jets.len());
        for c in jets {
            my_jets.push_back(match c {
                '<' => Jet::Left,
                '>' => Jet::Right,
                other => panic!("Unsupported jet direction of {}", other),
            });
        }

        let mut my_shapes = VecDeque::with_capacity(shapes.len());
        for s in &shapes {
            my_shapes.push_back(s.clone());
        }

        Sim {
            jets: my_jets,
            shapes: my_shapes,
            stack: VecDeque::from([0; 100000]),
            shape: 0u32,
            shape_row: 0,
            top_rock_row: None,
        }
    }

    fn next_shape(&mut self) -> u32 {
        let next_shape = self.shapes.pop_front().unwrap();
        self.shapes.push_back(next_shape);
        next_shape
    }

    fn next_jet(&mut self) -> &Jet {
        let next_jet = self.jets.pop_front().unwrap();
        self.jets.push_back(next_jet);
        &self.jets[self.jets.len() - 1]
    }

    fn drop_shape(&mut self) {
        self.add_shape();

        loop {
            let jet = self.next_jet();

            trace!("jet={}", jet);
            match jet {
                Jet::Left => self.move_left_if_possible(),
                Jet::Right => self.move_right_if_possible(),
            }

            let mut place_shape = false;
            if self.shape_row == 0 {
                trace!("Can't drop.  At bottom.");
                place_shape = true;
            } else {
                self.shape_row -= 1;
                if self.intersects() {
                    trace!("Can't drop.  Rock in way.");
                    place_shape = true;
                    self.shape_row += 1;
                } else {
                    trace!("Drop row");
                }
            }
            if place_shape {
                trace!("Place shape.");
                self.place_shape();
                break;
            }
        }
    }

    fn add_shape(&mut self) {
        self.shape = self.next_shape();

        let insert_row = match self.top_rock_row {
            None => 3,
            Some(top_rock_row) => top_rock_row + 4,
        };
        if insert_row + 4 > self.stack.len() {
            self.stack.extend(vec![0_u8; 1000000].iter());
        }

        self.shape_row = insert_row;
    }

    fn intersects(&self) -> bool {
        self.shape
            & ((self.stack[self.shape_row + 3] as u32) << 24
                | (self.stack[self.shape_row + 2] as u32) << 16
                | (self.stack[self.shape_row + 1] as u32) << 8
                | (self.stack[self.shape_row] as u32))
            != 0
    }

    fn place_shape(&mut self) {
        self.stack[self.shape_row] = (self.shape & 0b01111111) as u8 | self.stack[self.shape_row];
        self.stack[self.shape_row + 1] =
            ((self.shape >> 8) & 0b01111111) as u8 | self.stack[self.shape_row + 1];
        self.stack[self.shape_row + 2] =
            ((self.shape >> 16) & 0b01111111) as u8 | self.stack[self.shape_row + 2];
        self.stack[self.shape_row + 3] =
            ((self.shape >> 24) & 0b01111111) as u8 | self.stack[self.shape_row + 3];

        let mut r = self.shape_row;
        loop {
            r += 1;
            if self.stack[r] == 0 {
                self.top_rock_row = Some(r - 1);
                break;
            }
        }
    }

    #[cfg(test)]
    fn draw_stack(&self) {
        println!("");
        let start = match self.top_rock_row {
            Some(r) => r + 4,
            None => 3,
        };
        for i in 0..=start {
            let row = start - i;
            let a = format!("{:#09b}", self.stack[row]);
            println!("|{}|", &a[2..9].replace("0", "."));
        }
        println!("+-------+")
    }

    fn move_left_if_possible(&mut self) {
        if self.shape & 0b01000000010000000100000001000000 != 0 {
            debug!("Can't move left.  At edge");
            return;
        }

        self.shape = self.shape << 1;

        if self.intersects() {
            debug!("Can't move left.  Rock in way.");
            self.shape = self.shape >> 1;
        }
    }

    fn move_right_if_possible(&mut self) {
        if self.shape & 0b00000001000000010000000100000001 != 0 {
            debug!("Can't move right.  At edge");
            return;
        }

        self.shape = self.shape >> 1;
        if self.intersects() {
            debug!("Can't move right.  Rock in way.");
            self.shape = self.shape << 1;
        }
    }
}

struct GenerationRepetition {
    generation_iterations: usize,
    iteration_heights: Vec<usize>,
    cycle_start_iteration: usize,
    cycle_iteration_length: usize,
}

impl GenerationRepetition {
    fn find_generation_repetition(sim: &mut Sim) -> GenerationRepetition {
        let generation_iterations = sim.shapes.len() * sim.jets.len();

        info!(
            "Will repeat every {} iterations ( {} * {} )",
            generation_iterations,
            sim.shapes.len(),
            sim.jets.len()
        );

        let mut iteration_heights = Vec::<usize>::new();
        let mut generation_top_values = Vec::<u8>::new();

        loop {
            for _ in 0..generation_iterations {
                sim.drop_shape();
                iteration_heights.push(sim.top_rock_row.unwrap());
            }
            generation_top_values.push(sim.stack[sim.top_rock_row.unwrap()]);

            debug!(
                "generation = {} repeat iter = {} height: {} : top row = {}",
                iteration_heights.len() / generation_iterations,
                iteration_heights.len(),
                iteration_heights.last().unwrap(),
                sim.stack[sim.top_rock_row.unwrap()]
            );

            if repeats(&generation_top_values) {
                return GenerationRepetition {
                    generation_iterations,
                    iteration_heights,
                    cycle_start_iteration: 1 * generation_iterations,
                    cycle_iteration_length: (generation_top_values.len()) / 2
                        * generation_iterations,
                };
            }
        }
    }
}
fn run_large_sim(input: &String, count: usize) -> usize {
    let mut sim = Sim::new(input.chars().collect(), shapes_vec());

    match GenerationRepetition::find_generation_repetition(&mut sim) {
        GenerationRepetition {
            cycle_start_iteration,
            cycle_iteration_length,
            generation_iterations,
            iteration_heights,
        } => {
            let num_cycles = (count - cycle_start_iteration) / cycle_iteration_length;
            let iterations_spent_in_cycles = cycle_iteration_length * num_cycles;
            let remaining_iters = count - cycle_start_iteration - iterations_spent_in_cycles;

            let height_of_first_iteration = iteration_heights[generation_iterations - 1];
            let height_of_full_cycle = iteration_heights
                [cycle_start_iteration + cycle_iteration_length]
                - iteration_heights[cycle_start_iteration];
            let remain_height = iteration_heights[cycle_start_iteration + remaining_iters]
                - iteration_heights[cycle_start_iteration];

            info!("Cycle start iteration={}", cycle_start_iteration);
            info!(
                "Cycle length (in iterations)={} (from iteration {} to {})",
                cycle_iteration_length,
                cycle_start_iteration,
                cycle_start_iteration + cycle_iteration_length
            );
            info!(
                "Num full cycles= ({}-{})/{} = {}",
                count, cycle_start_iteration, cycle_iteration_length, num_cycles
            );
            info!(
                "Iterations in cycles= {}*{}= {}",
                cycle_iteration_length, num_cycles, iterations_spent_in_cycles
            );
            info!(
                "Remaining iterations = {}-{}-{}={}",
                count, cycle_start_iteration, iterations_spent_in_cycles, remaining_iters
            );
            info!("Height of 1st iteration = {}", height_of_first_iteration);
            info!("Height of full iteration cycle = {}", height_of_full_cycle);
            info!(
                "Height of {}*{}={}",
                num_cycles,
                height_of_full_cycle,
                num_cycles + height_of_full_cycle
            );
            debug!("Height of final iterations = {}", remain_height);

            height_of_first_iteration + num_cycles * height_of_full_cycle + remain_height + 1
        }
    }
}

fn repeats(values: &[u8]) -> bool {
    if values.len() & 1 == 1 {
        return false;
    }
    let half = values.len() / 2;
    &values[0..half] == &values[half..values.len()]
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{enable_logging, InputType},
        day17::{repeats, run_large_sim, shapes_vec, Sim},
    };

    #[test]
    fn test_repeats() {
        assert_eq!(
            repeats(&[28, 56, 64, 4, 4, 4, 48, 28, 56, 64, 4, 4, 4, 48]),
            true
        );
        assert_eq!(repeats(&vec![0, 1, 2, 0, 1, 2]), true);
        assert_eq!(
            repeats(&vec![0, 0, 0, 1, 2, 1, 2, 0, 0, 0, 1, 2, 1, 2]),
            true
        );
        assert_eq!(repeats(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 7]), false);
        assert_eq!(repeats(&vec![0, 0, 1, 0, 1, 2, 3, 1, 2, 3]), false);
        assert_eq!(
            repeats(&vec![9, 2, 3, 1, 0, 0, 5, 6, 1, 2, 3, 0, 0, 5, 6, 1, 2, 3],),
            false
        );
        assert_eq!(
            repeats(&vec![1, 2, 2, 4, 4, 2, 2, 4, 4, 1, 2, 2, 4, 4, 2, 2, 4, 4],),
            true
        );
    }

    #[test]
    fn visualize_example() {
        let input = InputType::Example.get_input_as_single_string(17).unwrap();

        let mut sim = Sim::new(input.chars().collect(), shapes_vec());

        for _ in 0..10 {
            sim.drop_shape();
            sim.draw_stack();
        }
    }

    #[test]
    fn test_part1_example() {
        let input = InputType::Example.get_input_as_single_string(17).unwrap();

        let mut sim = Sim::new(input.chars().collect(), shapes_vec());

        for _ in 0..2022 {
            sim.drop_shape();
        }

        let top_row = sim.top_rock_row.unwrap() + 1;
        assert_eq!(top_row, 3068);
    }

    #[test]
    fn test_part2_example() {
        enable_logging().unwrap();

        let input = InputType::Example.get_input_as_single_string(17).unwrap();
        let result = run_large_sim(&input, 1000000000000_usize);
        assert_eq!(result, 1514285714288_usize);
    }
}
