use std::collections::VecDeque;

use crate::day11::Input;
use itertools::{self, Itertools};

const INPUT: &str = include_str!("../../input/11/input");
const EXAMPLE: &str = include_str!("../../input/11/example");

#[derive(Debug, Clone)]
pub struct MonkeyItems {
    pub item_worry_levels: VecDeque<usize>,
    pub worry_operation: String,
    pub test_divisible: usize,
    pub test_true: usize,
    pub test_false: usize,
}

impl MonkeyItems {
    pub fn new(
        item_worry_levels: VecDeque<usize>,
        worry_operation: String,
        test_divisible: usize,
        test_true: usize,
        test_false: usize,
    ) -> Self {
        Self {
            //idx,
            item_worry_levels,
            worry_operation,
            test_divisible,
            test_true,
            test_false,
        }
    }
}

pub fn read() -> Input {
    read_into_monkey_items(INPUT)
}

pub fn readex() -> Input {
    read_into_monkey_items(EXAMPLE)
}

fn read_into_monkey_items(input: &str) -> Vec<MonkeyItems> {
    let mut mi_list: Vec<MonkeyItems> = vec![];

    for monkey_entries in input.split("\n\n") {
        let monkey_lines: Vec<&str> = monkey_entries.lines().collect();

        let worry_levels_line = monkey_lines[1].split_once(": ").unwrap().1;

        let item_worry_levels: VecDeque<usize> = worry_levels_line
            .split(", ")
            .map(|v| v.parse().unwrap())
            .collect();

        let worry_operation = monkey_lines[2].split_once("= ").unwrap().1.into();

        let test_divisible = monkey_lines[3]
            .split_once("by ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let test_true = monkey_lines[4]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let test_false = monkey_lines[5]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let mi = MonkeyItems::new(
            item_worry_levels,
            worry_operation,
            test_divisible,
            test_true,
            test_false,
        );
        mi_list.push(mi);
    }

    mi_list
}
