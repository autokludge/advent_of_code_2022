use crate::day02::{Input, Output};

use super::{Choice, GameResult, OpponentCode, ResponseCode, StrategyLine};

impl ResponseCode {
    fn to_p1_choice(&self) -> Choice {
        match &self {
            Self::X => Choice::Rock,
            Self::Y => Choice::Paper,
            Self::Z => Choice::Scissors,
        }
    }
}

impl StrategyLine {
    fn p1_game_result(&self) -> GameResult {
        use Choice::{Paper, Rock, Scissors};
        use GameResult::{Draw, Loss, Win};
        let opponent_choice = self.opponent_code.to_choice();
        let response_choice = self.response_code.to_p1_choice();
        opponent_choice.compare(&response_choice)
    }

    pub fn p1_calculate_score(&self) -> u8 {
        self.response_code.to_p1_choice().score() + self.p1_game_result().score()
    }
}

pub fn solve(input: &Input) -> Output {
    let scores: Vec<u8> = input
        .into_iter()
        .map(StrategyLine::p1_calculate_score)
        .collect();
    let max_score: u32 = scores.into_iter().map(|v| v as u32).sum();

    Output::U32(max_score)
}
