use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let overlaps = input
        .iter()
        .map(|v| if v.is_fully_overlapping() { 1 } else { 0 })
        .sum();
    Output::U32(overlaps)
}
