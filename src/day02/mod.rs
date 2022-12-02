pub mod input;
pub mod part1;
pub mod part2;

extern crate strum;
extern crate strum_macros;
use crate::{Output, Part};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}
impl Choice {
    pub fn score(&self) -> u8 {
        match (&self) {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn compare(&self, other: &Choice) -> GameResult {
        match (self, other) {
            (Self::Rock, Self::Rock) => GameResult::Draw,
            (Self::Rock, Self::Paper) => GameResult::Win,
            (Self::Rock, Self::Scissors) => GameResult::Loss,
            (Self::Paper, Self::Rock) => GameResult::Loss,
            (Self::Paper, Self::Paper) => GameResult::Draw,
            (Self::Paper, Self::Scissors) => GameResult::Win,
            (Self::Scissors, Self::Rock) => GameResult::Win,
            (Self::Scissors, Self::Paper) => GameResult::Loss,
            (Self::Scissors, Self::Scissors) => GameResult::Draw,
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    pub fn score(&self) -> u8 {
        match (&self) {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

#[derive(EnumString)]
enum OpponentCode {
    A,
    B,
    C,
}

impl OpponentCode {
    pub fn to_choice(&self) -> Choice {
        match &self {
            Self::A => Choice::Rock,
            Self::B => Choice::Paper,
            Self::C => Choice::Scissors,
        }
    }
}

#[derive(EnumString)]
enum ResponseCode {
    X,
    Y,
    Z,
}

pub struct StrategyLine {
    opponent_code: OpponentCode,
    response_code: ResponseCode,
}

impl StrategyLine {
    pub fn new(strat_line: &str) -> StrategyLine {
        let (opponent_input, response_input) = strat_line.split_once(" ").unwrap();
        let opponent_code = OpponentCode::from_str(opponent_input).unwrap();
        let response_code = ResponseCode::from_str(response_input).unwrap();
        StrategyLine {
            opponent_code,
            response_code,
        }
    }
}

pub type Input = Vec<StrategyLine>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
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
