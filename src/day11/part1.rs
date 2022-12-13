use std::fmt::Display;

use itertools::Itertools;

use crate::day11::{Input, Output};

use super::input::MonkeyItems;

impl Display for MonkeyItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item_worry_levels)
    }
}

pub fn solve(input: &mut Input) -> Output {
    let mut monkey_inspections: Vec<usize> = vec![0; input.len()];

    for round in (1..=20) {
        for monkey_idx in (0..input.len()) {
            while let Some(item_worry) = input[monkey_idx].item_worry_levels.pop_front() {
                monkey_inspections[monkey_idx] += 1;
                let mut worry = item_worry;
                let op_items: Vec<&str> = input[monkey_idx]
                    .worry_operation
                    .split(" ")
                    .take(3)
                    .collect();
                let first: usize = op_items[0].parse().unwrap_or(worry);
                let op = match op_items[1] {
                    "*" => |a: usize, b: usize| a * b,
                    "+" => |a: usize, b: usize| a + b,
                    _ => panic!("unexpected operator"),
                };
                let second: usize = op_items[2].parse().unwrap_or(worry);
                worry = op(first, second);
                worry = worry / 3;
                let throw_idx = if worry % input[monkey_idx].test_divisible == 0 {
                    input[monkey_idx].test_true
                } else {
                    input[monkey_idx].test_false
                };
                input[throw_idx as usize].item_worry_levels.push_back(worry);
            }
        }
    }

    let out: usize = monkey_inspections.iter().sorted().rev().take(2).product();
    Output::U64(out as u64)
}
