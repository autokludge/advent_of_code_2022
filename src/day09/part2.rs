use itertools::Itertools;

use crate::day09::{Input, Output};

use super::{
    //grid::{VecGrid, VecGridCoordinate},
    Direction,
    Point2d,
    RopeSimulation,
};

impl RopeSimulation {
    fn process_next_command_with_rope_length(&mut self) {
        // rather than just having a HEAD and TAIL
        // the rope now has 10 knots (H, 1, 2, 3, 4, 5, 6, 7, 8, 9)
        // knots follow existing behaviour
        // tail visited only needs to track where knot 9 has been

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
            for k in (0..=9) {
                let current_knot_new_loc: Point2d;
                if k == 0 {
                    current_knot_new_loc = self.knot_locations[0] + coord_shift;
                } else {
                    let target_knot = self.knot_locations[k as usize - 1];
                    let current_knot = self.knot_locations[k as usize];
                    let difference = target_knot - current_knot;
                    let shift = self.tail_offset(difference);
                    current_knot_new_loc = current_knot + shift;
                    if current_knot_new_loc == current_knot {
                        break;
                    }
                    if k == 9 {
                        if current_knot != current_knot_new_loc {
                            self.visited_locations.push(current_knot_new_loc)
                        }
                    }
                }
                self.knot_locations[k as usize] = current_knot_new_loc;
            }
        }
    }
}

pub fn solve(input: &Input) -> Output {
    let mut input = input.clone();

    for i in (0..input.instructions.len()) {
        input.process_next_command_with_rope_length();
    }
    let visits = input.visited_locations.iter().unique().count();

    Output::U64(visits as u64)
}
