use crate::day10::Input;

const INPUT: &str = include_str!("../../input/10/input");
const EXAMPLE: &str = include_str!("../../input/10/example");

pub fn read() -> Input {
    INPUT.lines().map(|s| s.to_owned()).collect()
}

pub fn readex() -> Input {
    EXAMPLE.lines().map(|s| s.to_owned()).collect()
}
