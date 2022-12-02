use std::collections::BinaryHeap;

use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input");

pub fn read() -> Input {
    fn parse_elf(ls: &str) -> u32 {
        ls.lines().map(|l| l.parse::<u32>().unwrap()).sum()
    }
    BinaryHeap::from_iter(INPUT.split("\n\n").map(|ls| parse_elf(ls)))
}
