use super::{EMPTY, SAND};
use crate::common::RowCol;
use crate::grid::Grid;
use log::trace;

#[derive(Debug)]
pub(crate) enum StepResult {
    CameToRestAt(RowCol),
    DroppedTo(RowCol),
    Jammed,
    FellIntoAbyss,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SandDropResult {
    CameToRestAt(RowCol),
    Jammed,
    FellIntoAbyss,
}

pub(crate) struct SandSim<'a> {
    pub(crate) sand_source_position: RowCol,
    pub(crate) pos: Option<RowCol>,
    pub(crate) grid: &'a mut Grid,
}

impl SandSim<'_> {
    pub(crate) fn is_open(&self, pos: RowCol) -> bool {
        let material = self.grid.get(pos);
        material == Some(EMPTY)
    }

    pub(crate) fn is_void(&self, pos: RowCol) -> bool {
        let material = self.grid.get(pos);
        material == None
    }

    pub(crate) fn step(&mut self) -> StepResult {
        let pos = self.pos.unwrap();
        let south_pos = RowCol::new(pos.row() + 1, pos.col());
        let southwest_pos = RowCol::new(pos.row() + 1, pos.col() - 1);
        let southeast_pos = RowCol::new(pos.row() + 1, pos.col() + 1);

        if self.is_void(south_pos) {
            return StepResult::FellIntoAbyss;
        }
        if self.is_open(south_pos) {
            self.pos.replace(south_pos);
            return StepResult::DroppedTo(south_pos);
        }

        if self.is_void(southwest_pos) {
            return StepResult::FellIntoAbyss;
        }
        if self.is_open(southwest_pos) {
            self.pos.replace(southwest_pos);
            return StepResult::DroppedTo(southwest_pos);
        }

        if self.is_void(southeast_pos) {
            return StepResult::FellIntoAbyss;
        }
        if self.is_open(southeast_pos) {
            self.pos.replace(southeast_pos);
            return StepResult::DroppedTo(southeast_pos);
        }

        if pos == self.sand_source_position {
            return StepResult::Jammed;
        }

        return StepResult::CameToRestAt(pos);
    }

    pub(crate) fn drop_sand(&mut self) -> SandDropResult {
        self.pos = Some(self.sand_source_position);
        loop {
            match self.step() {
                StepResult::DroppedTo(_) => {}
                StepResult::CameToRestAt(resting_pos) => {
                    self.grid.set(resting_pos, SAND);
                    trace!("Sand came to rest at {:?}", resting_pos);
                    self.pos = None;
                    return SandDropResult::CameToRestAt(resting_pos);
                }
                StepResult::Jammed => {
                    self.grid.set(self.sand_source_position, SAND);
                    trace!("Sand came to rest at {:?}", self.sand_source_position);
                    self.pos = None;
                    return SandDropResult::Jammed;
                }
                StepResult::FellIntoAbyss => return SandDropResult::FellIntoAbyss,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::RowCol,
        day14::{sim::SandDropResult, EMPTY, SAND},
        grid::Grid,
    };

    use super::SandSim;

    #[test]
    fn test_sim() {
        let grid_data = vec![
            "......+...",
            "..........",
            "..........",
            "..........",
            "....#...##",
            "....#...#.",
            "..###...#.",
            "........#.",
            "........#.",
            "#########.",
        ];

        let mut grid = Grid::new_offset(
            RowCol::new(0, 494),
            &grid_data.iter().map(|s| s.to_string()).collect(),
        );
        assert_eq!(grid.row_count(), 10);
        assert_eq!(grid.col_count(), 10);

        let mut sim = SandSim {
            sand_source_position: RowCol::new(0, 500),
            pos: None,
            grid: &mut grid,
        };

        println!("{:?}", sim.drop_sand());
        println!("{}", sim.grid);
        assert_eq!(sim.grid.get((8, 500).into()), Some(SAND));
        assert_eq!(sim.grid.get((8, 499).into()), Some(EMPTY));
        assert_eq!(sim.grid.get((8, 501).into()), Some(EMPTY));
        assert_eq!(sim.grid.get((7, 500).into()), Some(EMPTY));

        println!("{:?}", sim.drop_sand());
        println!("{}", sim.grid);
        assert_eq!(sim.grid.get((8, 500).into()), Some(SAND));
        assert_eq!(sim.grid.get((8, 499).into()), Some(SAND));
        assert_eq!(sim.grid.get((8, 501).into()), Some(EMPTY));
        assert_eq!(sim.grid.get((7, 500).into()), Some(EMPTY));

        println!("{:?}", sim.drop_sand());
        println!("{}", sim.grid);
        assert_eq!(sim.grid.get((8, 500).into()), Some(SAND));
        assert_eq!(sim.grid.get((8, 499).into()), Some(SAND));
        assert_eq!(sim.grid.get((8, 501).into()), Some(SAND));
        assert_eq!(sim.grid.get((7, 500).into()), Some(EMPTY));

        println!("{:?}", sim.drop_sand());
        println!("{}", sim.grid);
        assert_eq!(sim.grid.get((8, 500).into()), Some(SAND));
        assert_eq!(sim.grid.get((8, 499).into()), Some(SAND));
        assert_eq!(sim.grid.get((8, 501).into()), Some(SAND));
        assert_eq!(sim.grid.get((7, 500).into()), Some(SAND));

        for _ in 0..20 {
            println!("{:?}", sim.drop_sand());
        }
        println!("{}", sim.grid);
    }

    #[test]
    fn test_sim2() {
        let grid_data = vec![
            "...............+...............",
            "...............................",
            "...............................",
            "...............................",
            ".............#...##............",
            ".............#...#.............",
            "...........###...#.............",
            ".................#.............",
            ".................#.............",
            ".........#########.............",
            "...............................",
            "###############################",
        ];

        let mut grid = Grid::new_offset(
            RowCol::new(0, 485),
            &grid_data.iter().map(|s| s.to_string()).collect(),
        );

        assert_eq!(grid.row_count(), 12);
        assert_eq!(grid.col_count(), 31);

        let mut sim = SandSim {
            sand_source_position: RowCol::new(0, 500),
            pos: None,
            grid: &mut grid,
        };

        let mut drop_result: SandDropResult = SandDropResult::Jammed;
        for _ in 0..93 {
            drop_result = sim.drop_sand()
        }
        assert_eq!(drop_result, SandDropResult::Jammed);

        println!("{}", sim.grid);
    }
}
