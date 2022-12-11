use std::ops::Add;

use crate::{
    day01::input,
    day09::{Input, Output},
};

use super::{
    grid::{VecGrid, VecGridCoordinate},
    Direction, RopeSimulation,
};

#[derive(Clone)]
struct VecGridRelCoordinate {
    col: i32,
    row: i32,
}

impl From<VecGridRelCoordinate> for (i32, i32) {
    fn from(coord: VecGridRelCoordinate) -> Self {
        (coord.col, coord.row)
    }
}
impl From<(i32, i32)> for VecGridRelCoordinate {
    fn from(tup: (i32, i32)) -> Self {
        Self {
            col: tup.0,
            row: tup.1,
        }
    }
}
impl From<Direction> for VecGridRelCoordinate {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => (0, -1).into(),
            Direction::Down => (0, 1).into(),
            Direction::Left => (-1, 0).into(),
            Direction::Right => (1, 0).into(),
        }
    }
}

impl Add<&VecGridRelCoordinate> for VecGridCoordinate {
    type Output = VecGridCoordinate;

    fn add(self, relative: &VecGridRelCoordinate) -> Self::Output {
        Self {
            col: (self.col as i64 + relative.col as i64) as usize,
            row: (self.row as i64 + relative.row as i64) as usize,
        }
    }
}

impl RopeSimulation {
    fn process_next_command(&mut self) {
        let cmd = self.instructions.pop();
        if cmd.is_none() {
            return;
        }
        let cmd = cmd.unwrap();
        //println!("{}", cmd);
        let (direction, distance) = (cmd.direction, cmd.distance);
        let coord_shift: VecGridRelCoordinate = direction.into();

        for i in (0..distance) {
            let h_loc = self.locate('H').unwrap();
            let new_h_loc = h_loc + &coord_shift;
            self.display_grid.set_cell(h_loc, '.');

            let mut t_loc = self.locate('T');
            if t_loc.is_none() {
                t_loc = Some(h_loc);
            }

            let t_loc = t_loc.unwrap();
            let x_diff = (new_h_loc.col as i32 - t_loc.col as i32);
            let y_diff = (new_h_loc.row as i32 - t_loc.row as i32); // -ive is up
            let t_coord_shift = self.tail_offset(&(x_diff, y_diff).into());
            let new_t_loc = t_loc + &t_coord_shift;

            if (t_loc.col, t_loc.row) == (1, self.display_grid.rows) {
                self.display_grid.set_cell(t_loc, 's');
                self.tail_visited_grid.set_cell(t_loc, 's');
            } else {
                self.display_grid.set_cell(t_loc, '#');
                self.tail_visited_grid.set_cell(t_loc, '#');
            }

            self.display_grid.set_cell(new_t_loc, 'T');
            self.tail_visited_grid.set_cell(new_t_loc, '#');
            self.display_grid.set_cell(new_h_loc, 'H');
            //println!("{}", self);
        }
    }

    fn locate(&self, val: char) -> Option<VecGridCoordinate> {
        let idx = &self.display_grid.grid.iter().position(|&v| v == val)?;
        Some(self.display_grid.idx_to_cell_coord(*idx))
    }

    fn tail_offset(&self, h_difference: &VecGridRelCoordinate) -> VecGridRelCoordinate {
        match (h_difference.col, h_difference.row) {
            // H covers T
            (0, 0) => (0, 0).into(),
            // 1 distance, no need to move
            (-1, 0) => (0, 0).into(),
            (1, 0) => (0, 0).into(),
            (0, -1) => (0, 0).into(),
            (0, 1) => (0, 0).into(),
            (-1, -1) => (0, 0).into(),
            (1, -1) => (0, 0).into(),
            (-1, 1) => (0, 0).into(),
            (1, 1) => (0, 0).into(),
            // chase H in straight line
            (-2, 0) => (-1, 0).into(),
            (2, 0) => (1, 0).into(),
            (0, -2) => (0, -1).into(),
            (0, 2) => (0, 1).into(),
            //move diagonal to chase head
            //
            //  ..H.H.. (-1,-2).(1,-2)
            //  .Hx.xH. (-2,-1).(2,-1)
            //  ...T...
            //  .Hx.xH. (-2,1).(2,1)
            //  ..H.H.. (-1,2).(1,2)
            //
            //NW
            (-1, -2) => (-1, -1).into(),
            (-2, -1) => (-1, -1).into(),
            //NE
            (1, -2) => (1, -1).into(),
            (2, -1) => (1, -1).into(),
            //SW
            (-2, 1) => (-1, 1).into(),
            (-1, 2) => (-1, 1).into(),
            //SE
            (2, 1) => (1, 1).into(),
            (1, 2) => (1, 1).into(),

            (_, _) => panic!(),
        }
    }
}

pub fn solve(input: &Input) -> Output {
    let mut input = input.clone();
    println!("{}", input);
    //panic!();

    for i in (0..input.instructions.len()) {
        input.process_next_command();
    }
    let visits = input
        .tail_visited_grid
        .grid
        .iter()
        .filter(|&&c| c != '.')
        .count();

    Output::U64(visits as u64)
}
