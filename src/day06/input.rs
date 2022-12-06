use crate::day06::Input;

const INPUT: &str = include_str!("../../input/06/input");
const EXAMPLE: &str = include_str!("../../input/06/example");

pub fn read() -> Input {
    INPUT.chars().collect::<Vec<char>>()
}

pub fn readex() -> Input {
    EXAMPLE.chars().collect::<Vec<char>>()
}
