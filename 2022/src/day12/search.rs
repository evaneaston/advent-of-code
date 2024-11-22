use super::{get_candidates, RowCol};
use crate::grid::Grid;
use log::debug;
use std::collections::HashMap;

pub struct Search<'a> {
    pub grid: &'a Grid,
    pub from: RowCol,
    pub to: RowCol,
    candidate_test: fn(u8, u8) -> bool,
}

pub struct SearchState {
    path: Vec<RowCol>,
    smallest_observed_length: HashMap<RowCol, usize>,
    pub shortest: Option<Vec<RowCol>>,
}
impl SearchState {
    fn smallest_observed_path_length_at(&self, location: &RowCol) -> usize {
        self.smallest_observed_length
            .get(location)
            .map(|s| *s)
            .unwrap_or(usize::MAX)
    }

    fn set_smallest_observed_path_length_at(&mut self, location: RowCol) {
        let mut insert = true;
        if let Some(min_len) = self.smallest_observed_length.get(&location) {
            if self.path.len() >= *min_len {
                insert = false;
            }
        }
        if insert {
            self.smallest_observed_length
                .insert(location, self.path.len());
        }
    }
}

impl<'a> Search<'a> {
    pub fn new_uphill_search(grid: &'a Grid, from: RowCol, to: RowCol) -> Search<'a> {
        Self {
            grid,
            from,
            to,
            candidate_test: Self::uphill_test,
        }
    }

    pub(crate) fn uphill_test(from_elevation: u8, to_elevation: u8) -> bool {
        to_elevation <= (from_elevation + 1)
    }

    pub fn find_shortest_path(&self, search_state: &Option<SearchState>) -> SearchState {
        let mut state = if let Some(existing) = search_state {
            SearchState {
                path: Vec::with_capacity(500),
                shortest: None, // existing.shortest.clone(),
                smallest_observed_length: existing.smallest_observed_length.clone(),
            }
        } else {
            SearchState {
                path: Vec::with_capacity(500),
                shortest: None,
                smallest_observed_length: HashMap::with_capacity(500),
            }
        };

        state.path.push(self.from);

        self.shortest_path_scan(&mut state);

        state
    }

    fn shortest_path_scan(&self, state: &mut SearchState) {
        if let Some(s) = &state.shortest {
            if s.len() < state.path.len() {
                // don't bother, we already found a shorter path
                return;
            }
        }
        //print!("\r{}", state.path.len());
        let candidates =
            get_candidates(self.grid, *state.path.last().unwrap(), self.candidate_test);

        for next in &candidates {
            if state.smallest_observed_path_length_at(&next) > state.path.len() + 1 {
                state.path.push(*next);
                state.set_smallest_observed_path_length_at(*next);
                if *next == self.to {
                    debug!(
                        "got to the end ({:?}) one way!: Path len={:?}",
                        self.to,
                        state.path.len()
                    );
                    let replace = match &state.shortest {
                        Some(shortest) => shortest.len() > state.path.len(),
                        None => true,
                    };
                    if replace {
                        state.shortest.replace(state.path.clone());
                    }
                    state.path.pop();
                    return;
                }

                self.shortest_path_scan(state);

                state.path.pop();
            }
        }
    }
}
