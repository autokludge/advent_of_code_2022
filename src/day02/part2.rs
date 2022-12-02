use crate::day02::{Input, Output};

use super::{Choice, GameResult, OpponentCode, ResponseCode, StrategyLine};

enum ResponseAction {
    TryWinning,
    TryDrawing,
    TryLosing,
}

impl ResponseAction {
    pub fn to_p2_choice(&self, opponent_choice: &Choice) -> Choice {
        match (opponent_choice, self) {
            (Choice::Rock, Self::TryWinning) => Choice::Paper,
            (Choice::Rock, Self::TryDrawing) => Choice::Rock,
            (Choice::Rock, Self::TryLosing) => Choice::Scissors,
            (Choice::Paper, Self::TryWinning) => Choice::Scissors,
            (Choice::Paper, Self::TryDrawing) => Choice::Paper,
            (Choice::Paper, Self::TryLosing) => Choice::Rock,
            (Choice::Scissors, Self::TryWinning) => Choice::Rock,
            (Choice::Scissors, Self::TryDrawing) => Choice::Scissors,
            (Choice::Scissors, Self::TryLosing) => Choice::Paper,
        }
    }
}

impl ResponseCode {
    fn to_response_action(&self) -> ResponseAction {
        match self {
            ResponseCode::X => ResponseAction::TryLosing,
            ResponseCode::Y => ResponseAction::TryDrawing,
            ResponseCode::Z => ResponseAction::TryWinning,
        }
    }
}

impl StrategyLine {
    fn p2_game_result(&self) -> GameResult {
        use Choice::{Paper, Rock, Scissors};
        use GameResult::{Draw, Loss, Win};
        let opponent_choice = self.opponent_code.to_choice();
        let response_choice = self
            .response_code
            .to_response_action()
            .to_p2_choice(&opponent_choice);
        opponent_choice.compare(&response_choice)
    }

    pub fn p2_calculate_score(&self) -> u8 {
        let opponent_choice = self.opponent_code.to_choice();
        let response_action = self.response_code.to_response_action();
        response_action.to_p2_choice(&opponent_choice).score() + self.p2_game_result().score()
    }
}

pub fn solve(input: &Input) -> Output {
    let scores: Vec<u8> = input
        .into_iter()
        .map(StrategyLine::p2_calculate_score)
        .collect();
    let max_score: u32 = scores.into_iter().map(|v| v as u32).sum();

    Output::U32(max_score)
}
