use crate::day03::Input;

const INPUT: &str = include_str!("../../input/03/input");
const EXAMPLE: &str = include_str!("../../input/03/example");

pub fn read() -> Input<'static> {
    INPUT.lines().collect()
}

pub fn readex() -> Input<'static> {
    EXAMPLE.lines().collect()
}
