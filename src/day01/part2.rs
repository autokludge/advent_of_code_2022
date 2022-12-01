use crate::day01::{Input, Output};
use itertools::{self, Itertools};

pub fn solve(input: &Input) -> Output {
    let total = input.iter().sorted().rev().take(3).sum();
    Output::U32(total)
}
