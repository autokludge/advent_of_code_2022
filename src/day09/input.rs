use crate::day09::Input;

use super::RopeSimulation;

const INPUT: &str = include_str!("../../input/09/input");
const EXAMPLE: &str = include_str!("../../input/09/example");
const EXAMPLE2: &str = include_str!("../../input/09/example2");

pub fn read() -> Input {
    parse_to_rope_simulation(INPUT)
}

pub fn readex() -> Input {
    parse_to_rope_simulation(EXAMPLE)
}

pub fn readex_two() -> Input {
    parse_to_rope_simulation(EXAMPLE2)
}

fn parse_to_rope_simulation(input: &str) -> RopeSimulation {
    RopeSimulation::new(input)
}
