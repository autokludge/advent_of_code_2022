use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    Output::U32(input.into_iter().take(3).sum())
}
