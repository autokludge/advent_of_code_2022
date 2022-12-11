use itertools::Itertools;

use crate::day08::{Input, TreeGrid};

const INPUT: &str = include_str!("../../input/08/input");
const EXAMPLE: &str = include_str!("../../input/08/example");

pub fn read() -> Input {
    parse_tree_grid(INPUT)
}

pub fn readex() -> Input {
    parse_tree_grid(EXAMPLE)
}

fn parse_tree_grid(input: &str) -> TreeGrid {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .join("")
        .chars()
        .map(|v| v.to_string().parse::<u8>().unwrap())
        .collect();
    TreeGrid::new(grid, rows, cols)
}
