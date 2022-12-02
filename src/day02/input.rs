use std::{str::FromStr, string::ParseError, vec};

use crate::day02::Input;

use super::StrategyLine;

const INPUT: &str = include_str!("../../input/02/input");

pub fn read() -> Input {
    let mut strategy_input: Vec<StrategyLine> = vec![];
    for strat_line in INPUT.lines() {
        strategy_input.push(StrategyLine::new(strat_line));
    }
    strategy_input
}
