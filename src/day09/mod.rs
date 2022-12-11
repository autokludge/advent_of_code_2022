pub mod grid;
pub mod input;
pub mod part1;
pub mod part2;

use std::fmt::Display;

use itertools::Itertools;
use strum_macros::Display;

use crate::{Output, Part};

use self::grid::VecGrid;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone)]
pub struct RopeInstruction {
    direction: Direction,
    distance: u8,
}

impl std::fmt::Display for RopeInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?} {}", self.direction, self.distance)
    }
}

impl RopeInstruction {
    pub fn new(direction: Direction, distance: u8) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

pub struct RopeSimulation {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    x_start: i32,
    y_start: i32,

    //reverse order, ie pop() takes first instruction
    instructions: Vec<RopeInstruction>,
    //grid for display
    display_grid: VecGrid<char>,
    tail_visited_grid: VecGrid<char>,
}

impl Clone for RopeSimulation {
    fn clone(&self) -> Self {
        Self {
            xmin: self.xmin.clone(),
            xmax: self.xmax.clone(),
            ymin: self.ymin.clone(),
            ymax: self.ymax.clone(),
            x_start: self.x_start.clone(),
            y_start: self.y_start.clone(),
            instructions: self.instructions.clone(),
            display_grid: self.display_grid.clone(),
            tail_visited_grid: self.tail_visited_grid.clone(),
        }
    }
}

impl std::fmt::Display for RopeSimulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "cols:{}  rows:{}",
            self.display_grid.cols, self.display_grid.rows
        );
        writeln!(
            f,
            "{:.>20},{}\n...{:.>10},{:.<10}...\n{},{:.<20}",
            self.xmax, self.ymax, self.x_start, self.y_start, self.xmin, self.ymin,
        );
        writeln!(
            f,
            "{:.>20},{}\n...{:.>10},{:.<10}...\n{},{:.<20}",
            self.display_grid.cols, 1, self.x_start, self.y_start, 1, self.display_grid.rows,
        );
        writeln!(f, "grid:          tail_visited:");

        let dc = self.display_grid.grid.chunks(self.display_grid.cols);
        let tc = self.tail_visited_grid.grid.chunks(self.display_grid.cols);

        // for (disp_row, _tail_row) in dc.zip(tc) {
        //     // writeln!(f, "{:?}\t{:?}", disp_row, tail_row);
        //     //writeln!(f, "{:?}", disp_row.iter().join(""));
        //     writeln!(f, "{:?}", disp_row);
        // }
        Ok(())
    }
}

impl RopeSimulation {
    pub fn new(input: &str) -> Self {
        let mut instructions = vec![];
        let mut h: i32 = 1;
        let mut w: i32 = 1;
        let mut h_max: i32 = 1;
        let mut h_min: i32 = 1;
        let mut w_max: i32 = 1;
        let mut w_min: i32 = 1;

        for line in input.lines() {
            //println!("grid {}x{}   {}", w_max, h_max, line);
            let (dir, dist) = line.split_once(" ").unwrap();

            let distance: usize = dist.parse().unwrap();
            let direction = match dir {
                "U" => {
                    h += distance as i32;
                    h_max = h_max.max(h);
                    //println!("up {} pointer_x:{} height {}", distance, h, h_max);
                    Direction::Up
                }
                "D" => {
                    h -= distance as i32;
                    h_min = h_min.min(h);
                    Direction::Down
                }
                "L" => {
                    w -= distance as i32;
                    w_min = w_min.min(w);
                    Direction::Left
                }
                "R" => {
                    w += distance as i32;
                    w_max = w_max.max(w);
                    //println!("right {} pointer_x:{} width:{}", distance, w, w_max);
                    Direction::Right
                }
                _ => panic!(),
            };

            let ri = RopeInstruction::new(direction, distance as u8);
            instructions.push(ri);
        }
        let rows = (h_max + (1 - h_min)) as usize; //carefull, how do we know upper limit on width, height?
        let cols = (w_max + (1 - w_min)) as usize;

        // 1 = x_s + n

        let start_x = 2 - w_min; //  1 -> 1   0 -> 2   -1 -> 3   -2 -> 4
        let start_y = rows as i32 + (h_min - 1);

        // w_min = 1 -> start_y = rows          ==>> start_y = rows + (w_min(1) -1) ==>> rows + 0
        // w_min = 0 -> start_y = rows - 1      ==>> start_y = rows + (w_min(0) -1) ==>> rows + (-1)
        // w_min = -1 -> start_y = rows - 2     ==>> start_y = rows + (w_min(-1) -1) ==>> rows + (-2)

        let default_grid: Vec<char> = vec!['.'; rows * cols];
        let instructions = instructions.into_iter().rev().collect();

        let mut grid = VecGrid::new(default_grid, cols, rows);
        let idx = grid.coord_to_idx((start_x as usize, start_y as usize).into());
        grid.grid[idx] = 'H';
        Self {
            xmin: w_min,
            xmax: w_max,
            ymin: h_min,
            ymax: h_max,
            x_start: start_x,
            y_start: start_y,
            instructions,
            display_grid: grid.clone(),
            tail_visited_grid: grid,
        }
    }
}

pub type Input = RopeSimulation;

pub fn run(part: Part) -> Output {
    match part {
        Part::OneEx => {
            let exampleinput = input::readex();
            part1::solve(&exampleinput)
        }
        Part::One => {
            let input = input::read();
            part1::solve(&input)
        }
        Part::TwoEx => {
            let exampleinput = input::readex();
            part2::solve(&exampleinput)
        }
        Part::Two => {
            let input = input::read();
            part2::solve(&input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one_example() {
        let result = run(Part::OneEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two_example() {
        let result = run(Part::TwoEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
