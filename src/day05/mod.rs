pub mod input;
pub mod part1;
pub mod part2;

use std::collections::HashMap;

use crate::{Output, Part};

#[derive(Debug)]
pub struct CargoInstruction {
    source: u8,
    target: u8,
    crates_to_move: u8,
}

impl CargoInstruction {
    pub fn new(source: u8, target: u8, crates_to_move: u8) -> Self {
        Self {
            source,
            target,
            crates_to_move,
        }
    }
}

#[derive(Debug)]
pub struct WarehouseCrane {
    stacks: Vec<Vec<char>>,
    instructions: Vec<CargoInstruction>,
}

impl WarehouseCrane {
    pub fn new(stacks: Vec<Vec<char>>, instructions: Vec<CargoInstruction>) -> Self {
        Self {
            stacks,
            instructions,
        }
    }
}

pub type Input = WarehouseCrane;

pub fn run(part: Part) -> Output {
    let mut exampleinput = input::readex();
    let mut input = input::read();
    match part {
        Part::OneEx => part1::solve(&mut exampleinput),
        Part::One => part1::solve(&mut input),
        Part::TwoEx => part2::solve(&mut exampleinput),
        Part::Two => part2::solve(&mut  input),
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
