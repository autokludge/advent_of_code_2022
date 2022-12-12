use itertools::Itertools;
use std::ops::Add;

use crate::{
    day01::input,
    day09::{Input, Output},
};

use super::{
    //grid::{VecGrid, VecGridCoordinate},
    Direction,
    Point2d,
    RopeSimulation,
};

impl RopeSimulation {
    fn process_next_command(&mut self) {
        let cmd = self.instructions.pop();
        if cmd.is_none() {
            return;
        }
        let cmd = cmd.unwrap();
        let (direction, distance) = (cmd.direction, cmd.distance);
        let coord_shift: Point2d = direction.into();

        for i in (0..distance) {
            // target_knot_loc
            // current_knot_loc
            // current_knot_coord_shift
            // current_knot_new_loc
            for k in (0..=1) {
                let current_knot_new_loc: Point2d;
                if k == 0 {
                    current_knot_new_loc = self.knot_locations[0] + coord_shift;
                } else {
                    let target_knot = self.knot_locations[k as usize - 1];
                    let current_knot = self.knot_locations[k as usize];
                    let difference = target_knot - current_knot;
                    let shift = self.tail_offset(difference);
                    current_knot_new_loc = current_knot + shift;
                    if current_knot != current_knot_new_loc {
                        self.visited_locations.push(current_knot_new_loc)
                    }
                }
                self.knot_locations[k as usize] = current_knot_new_loc;
            }
        }
    }

    pub fn tail_offset(&self, h_difference: Point2d) -> Point2d {
        match (h_difference.x, h_difference.y) {
            // H covers T
            (0, 0) => (0, 0).into(),
            // 1 distance, no need to move
            (-1, 0) => (0, 0).into(),
            (1, 0) => (0, 0).into(),
            (0, 1) => (0, 0).into(),
            (0, -1) => (0, 0).into(),
            (-1, 1) => (0, 0).into(),
            (1, 1) => (0, 0).into(),
            (-1, -1) => (0, 0).into(),
            (1, -1) => (0, 0).into(),
            // chase H in straight line
            (-2, 0) => (-1, 0).into(),
            (2, 0) => (1, 0).into(),
            (0, -2) => (0, -1).into(),
            (0, 2) => (0, 1).into(),
            //move diagonal to chase head
            //
            //  ..H.H.. (-1,2).(1,2)
            //  .Hx.xH. (-2,1).(2,1)
            //  ...T...
            //  .Hx.xH. (-2,-1).(2,-1)
            //  ..H.H.. (-1,-2).(1,-2)
            //
            //NW
            (-1, 2) => (-1, 1).into(),
            (-2, 1) => (-1, 1).into(),
            //NE
            (1, 2) => (1, 1).into(),
            (2, 1) => (1, 1).into(),
            //SW
            (-2, -1) => (-1, -1).into(),
            (-1, -2) => (-1, -1).into(),
            //SE
            (2, -1) => (1, -1).into(),
            (1, -2) => (1, -1).into(),

            (2, 2) => (1, 1).into(),
            (2, -2) => (1, -1).into(),
            (-2, 2) => (-1, 1).into(),
            (-2, -2) => (-1, -1).into(),

            (_, _) => panic!("{},{}", h_difference.x, h_difference.y),
        }
    }
}

pub fn solve(input: &Input) -> Output {
    let mut input = input.clone();
    println!("{}", input);

    for i in (0..input.instructions.len()) {
        input.process_next_command();
    }
    let visits = input.visited_locations.iter().unique().count();

    Output::U64(visits as u64)
}
