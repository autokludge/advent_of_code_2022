use crate::day04::Input;

use super::SectorPair;

const INPUT: &str = include_str!("../../input/04/input");
const EXAMPLE: &str = include_str!("../../input/04/example");

pub fn read() -> Input {
    INPUT.lines().map(|l| parse_sector_pair(l)).collect()
}

pub fn readex() -> Input {
    EXAMPLE.lines().map(|l| parse_sector_pair(l)).collect()
}

fn parse_sector_pair(input: &str) -> SectorPair {
    let sectors: Vec<u8> = input
        .split(&['-', ','])
        .map(|s| s.parse::<u16>().unwrap() as u8)
        .collect();
    match sectors.len() {
        4 => SectorPair::new(sectors),
        _ => panic!(),
    }
}
