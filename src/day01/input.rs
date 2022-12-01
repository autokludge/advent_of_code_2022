use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input");

pub fn read() -> Input {
    let mut troup_elves: Vec<u32> = vec![];
    let mut elf_calories: u32 = 0;
    for line in INPUT.lines() {
        if line.is_empty() {
            troup_elves.push(elf_calories);
            elf_calories = 0;
            continue;
        }
        elf_calories += line.parse::<u32>().unwrap();
    }
    troup_elves
}
