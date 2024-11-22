use log::debug;

use crate::common::{AocError, InputType};
use std::collections::HashMap;

use self::input::Valve;

mod input;

pub fn part1() -> Result<String, AocError> {
    let input = input::get_input(InputType::Challenge)?;
    let answer = get_max_flow(&IndexedInput::from(input));

    assert_eq!(1737, answer);

    Ok(format!("{:?}", answer))
}

pub fn part2() -> Result<String, AocError> {
    let input = input::get_input(InputType::Challenge)?;
    let indexed = IndexedInput::from(input);

    let num_flowable_valves = indexed
        .valve_flows
        .iter()
        .filter(|flow_rate| **flow_rate > 0)
        .count();
    let permutation_count = 1 << num_flowable_valves;

    let state = simulate_all_permutations(26, permutation_count, &indexed);

    let mut max = 0_usize;

    let starting_valve_index = indexed.valve_indexes[&"AA".to_string()];
    let all_ones_permutation = permutation_count - 1;
    for valve_set_a in 0..(permutation_count / 2) {
        let valve_set_b = all_ones_permutation - valve_set_a;

        let released_a = state[index_of(
            NOfM(25, 26),
            NOfM(valve_set_a, permutation_count),
            NOfM(starting_valve_index, indexed.valve_indexes.len()),
        )];
        let released_b = state[index_of(
            NOfM(25, 26),
            NOfM(valve_set_b, permutation_count),
            NOfM(starting_valve_index, indexed.valve_indexes.len()),
        )];

        max = max.max(released_a + released_b);
    }

    assert_eq!(max, 2216);

    Ok(format!("{max}"))
}

struct IndexedInput {
    valve_indexes: HashMap<String, usize>,
    valve_flows: Vec<u32>,
    valve_tunnel_indexes: Vec<Vec<usize>>,
}

impl IndexedInput {
    fn from(mut input: Vec<Valve>) -> Self {
        input.sort_by(|v1, v2| v2.flow_rate.cmp(&v1.flow_rate));
        debug!("valves: {:?}", input);

        let valve_indexes = input
            .iter()
            .enumerate()
            .map(|(i, v)| (v.id.to_owned(), i))
            .collect::<HashMap<_, _>>();
        let valve_flows = input.iter().map(|v| v.flow_rate).collect::<Vec<_>>();

        let valve_tunnel_indexes = input
            .iter()
            .map(|v| {
                v.tunnels
                    .iter()
                    .map(|valve_id| valve_indexes[valve_id])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        IndexedInput {
            valve_indexes,
            valve_flows,
            valve_tunnel_indexes,
        }
    }
}

fn get_max_flow(input: &IndexedInput) -> usize {
    let num_flowable_valves = input
        .valve_flows
        .iter()
        .filter(|flow_rate| **flow_rate > 0)
        .count();

    let num_permutations: usize = 1 << num_flowable_valves;

    debug!("valve_indexes: {:?}", input.valve_indexes);
    debug!("valve_flows: {:?}", input.valve_flows);
    debug!("valve_tunnel_indexes: {:?}", input.valve_tunnel_indexes);
    debug!("num_flowable_valves: {}", num_flowable_valves);
    debug!("num_permutations: {}", num_permutations);

    let num_minutes = 30;

    let state = simulate_all_permutations(num_minutes, num_permutations, input);

    let i = index_of(
        NOfM(num_minutes - 1, num_minutes),
        NOfM(num_permutations - 1, num_permutations),
        NOfM(
            input.valve_indexes[&"AA".to_string()],
            input.valve_indexes.len(),
        ),
    );
    let answer = state[i];
    answer
}

/**
 * Build up the max releasable pressure for the number of minutes for every permutation of
 * opening valves that can flow.
 *
 * Use the index_of function to generate an index into the state for a a simulation minute, a permutation
 * (flowable valve bitmask) and a starting valve index.
 */
fn simulate_all_permutations(
    num_minutes: usize,
    num_permutations: usize,
    input: &IndexedInput,
) -> Vec<usize> {
    let mut state = vec![0; num_minutes * input.valve_indexes.len() * num_permutations];

    for minute in 1..num_minutes {
        for current_valve_index in 0..input.valve_flows.len() {
            for permutation in 0..num_permutations {
                let open_valve_bits = permutation;
                let current_valve_bit = 1 << current_valve_index;

                let mut released = 0;

                if open_valve_bits & current_valve_bit != 0 {
                    let this_valves_contribution_for_this_minute =
                        minute * input.valve_flows[current_valve_index] as usize;
                    let i = index_of(
                        NOfM(minute - 1, 30),
                        NOfM(open_valve_bits - current_valve_bit, num_permutations),
                        NOfM(current_valve_index, input.valve_indexes.len()),
                    );
                    let previous_round_without_this_valve = state[i];

                    let this_plus_prev = previous_round_without_this_valve
                        + this_valves_contribution_for_this_minute;

                    released = released.max(this_plus_prev);
                }
                for preceding_valve_index in
                    input.valve_tunnel_indexes.get(current_valve_index).unwrap()
                {
                    let i = index_of(
                        NOfM(minute - 1, 30),
                        NOfM(open_valve_bits, num_permutations),
                        NOfM(*preceding_valve_index, input.valve_indexes.len()),
                    );
                    let preceeding_projected_release = state[i];

                    released = released.max(preceeding_projected_release)
                }

                let i = index_of(
                    NOfM(minute, 30),
                    NOfM(open_valve_bits, num_permutations),
                    NOfM(current_valve_index, input.valve_indexes.len()),
                );

                state[i] = released;
            }
        }
    }
    state
}

/***
 *  |  minute n | minute n-1 | ... | minute 1 | minute 0 |
 *  |           \
 *  |            \____________________________
 *  |                                          \
 *  | perm m | perm m-1 | ... | perm 1 | perm 0 |
 *  |        \
 *  |         \
 *  |          \___________________________________
 *  |                                              \
 *  | valve p | valve p-1 | ... | valve 1 | valve 0 |
 *
 */
fn index_of(minute: NOfM<usize>, permutation: NOfM<usize>, valve_index: NOfM<usize>) -> usize {
    let size_of_minute = valve_index.count() * permutation.count();
    let size_of_permutation = valve_index.count();
    let mut index = minute.i() * size_of_minute;
    index += permutation.i() * size_of_permutation;
    index += valve_index.i();
    index
}

struct NOfM<T>(T, T);
impl<T> NOfM<T> {
    fn i(&self) -> &T {
        &self.0
    }
    fn count(&self) -> &T {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::InputType,
        day16::{get_max_flow, input, IndexedInput},
    };

    #[test]
    fn test_part1_example_sim() {
        let input = input::get_input(InputType::Example).unwrap();

        let answer = get_max_flow(&IndexedInput::from(input));

        assert_eq!(1651, answer); // example
    }
}
