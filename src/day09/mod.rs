pub mod input;
pub mod part1;
pub mod part2;

use std::{fmt::Display, ops::Sub};

use itertools::Itertools;
use std::ops::Add;
use strum_macros::Display;

use crate::{Output, Part};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point2d {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}
impl From<Point2d> for (i32, i32) {
    fn from(p: Point2d) -> Self {
        (p.x, p.y)
    }
}

impl From<Direction> for Point2d {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => (0, 1).into(),
            Direction::Down => (0, -1).into(),
            Direction::Left => (-1, 0).into(),
            Direction::Right => (1, 0).into(),
        }
    }
}

impl Add for Point2d {
    type Output = Point2d;

    fn add(self, other: Point2d) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2d {
    type Output = Point2d;

    fn sub(self, other: Point2d) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct RopeSimulation {
    instructions: Vec<RopeInstruction>,
    knot_locations: Vec<Point2d>,
    visited_locations: Vec<Point2d>,
}

impl Clone for RopeSimulation {
    fn clone(&self) -> Self {
        Self {
            instructions: self.instructions.clone(),
            knot_locations: self.knot_locations.clone(),
            visited_locations: self.visited_locations.clone(),
        }
    }
}

impl std::fmt::Display for RopeSimulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "knot locations: {:?}", self.knot_locations);
        writeln!(f, "visited locations: {:?}", self.visited_locations)
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

        for line in input.lines().rev() {
            let (dir, dist) = line.split_once(" ").unwrap();

            let distance: usize = dist.parse().unwrap();
            let direction = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };

            let ri = RopeInstruction::new(direction, distance as u8);
            instructions.push(ri);
        }

        let origin_point: Point2d = (0, 0).into();

        let knot_locations = vec![origin_point; 10];
        let visited_locations = vec![origin_point];

        Self {
            instructions,
            knot_locations,
            visited_locations,
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
            let exampleinput2 = input::readex_two();

            part2::solve(&exampleinput);

            println!("* * * * * * * * * * * * * *");
            println!(" * * * * * * * * * * * * * ");
            println!("* * * * * * * * * * * * * *");

            part2::solve(&exampleinput2)
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
