use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use crate::day05::{CargoInstruction, Input};

use super::WarehouseCrane;

const INPUT: &str = include_str!("../../input/05/input");
const EXAMPLE: &str = include_str!("../../input/05/example");

pub fn read() -> Input {
    let input = INPUT;
    parse_input_to_warehouse(input)
}

pub fn readex() -> Input {
    let input = EXAMPLE;
    parse_input_to_warehouse(input)
}

fn parse_input_to_warehouse(input: &str) -> WarehouseCrane {
    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];
    let first: Vec<char> = vec![' '];
    stacks.push(first); //blank first stack so it is 1-based

    let mut stack_lines = stack_input.lines().into_iter().rev();
    stack_lines.next();
    for line in stack_lines {
        for (i, c) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            let crate_val = c[1];
            if crate_val == ' ' {
                continue;
            }
            // offsets to make 1 indexed
            if i + 2 > stacks.len() {
                let st: Vec<char> = vec![crate_val];
                stacks.push(st);
            } else {
                stacks[i + 1].push(crate_val);
            }
        }
    }

    use regex::Regex;
    let re = Regex::new(r"^\D+(\d+)\D+(\d+)\D+(\d+)$").unwrap();
    let mut instructions: Vec<CargoInstruction> = vec![];

    for line in instruction_input.lines() {
        let cap = re.captures(line).unwrap();
        if cap.len() != 4 {
            //3 capture groups + 1 linematch group
            println!("{}, {:?}", cap.len(), cap);
            panic!();
        }
        //println!("{}, {:?}", cap.len(), cap);
        let (crates_to_move, source, target) = (
            u8::from_str(&cap[1]).unwrap(),
            u8::from_str(&cap[2]).unwrap(),
            u8::from_str(&cap[3]).unwrap(),
        );
        let stack_instruction = CargoInstruction::new(source, target, crates_to_move);
        instructions.push(stack_instruction);
    }

    WarehouseCrane {
        stacks,
        instructions,
    }
}
