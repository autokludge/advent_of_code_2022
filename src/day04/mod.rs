pub mod input;
pub mod part1;
pub mod part2;

use clap::builder::Str;

use crate::{Output, Part};

#[derive(Debug)]
pub struct SectorPair {
    first_lower: u8,
    first_upper: u8,
    second_lower: u8,
    second_upper: u8,
}

impl SectorPair {
    pub fn new(sectors: Vec<u8>) -> Self {
        Self {
            first_lower: sectors[0],
            first_upper: sectors[1],
            second_lower: sectors[2],
            second_upper: sectors[3],
        }
    }

    pub fn is_fully_overlapping(&self) -> bool {
        let first_contains_second =
            &self.first_lower <= &self.second_lower && &self.first_upper >= &self.second_upper;
        let second_contains_first =
            &self.second_lower <= &self.first_lower && &self.second_upper >= &self.first_upper;
        first_contains_second || second_contains_first
    }

    pub fn is_partly_overlapping(&self) -> bool {
        let first_overlaps_left =
            &self.first_lower <= &self.second_lower && &self.first_upper >= &self.second_lower;
        let first_overlaps_right =
            &self.second_lower <= &self.first_lower && &self.second_upper >= &self.first_lower;
        let b = self.is_fully_overlapping() || first_overlaps_left || first_overlaps_right;
        b
    }
}

pub type Input = Vec<SectorPair>;

pub fn run(part: Part) -> Output {
    let exampleinput = input::readex();
    let input = input::read();
    match part {
        Part::OneEx => part1::solve(&exampleinput),
        Part::One => part1::solve(&input),
        Part::TwoEx => part2::solve(&exampleinput),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
