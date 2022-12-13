use crate::day11::{Input, Output};
use itertools::Itertools;
use std::ops::{Div, Mul, Rem};

fn gcd(a: usize, b: usize) -> usize {
    let mut ab = (a, b);
    while ab.1 != 0 {
        ab = (ab.1, ab.0 % ab.1)
    }
    ab.0
}
fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    (a / g) * b
}

pub fn solve(input: &mut Input) -> Output {
    let mut monkey_inspections: Vec<usize> = vec![0; input.len()];
    //let lcm = input.iter().fold(1, |acc, x| lcm(acc, x.test_divisible));
    let naive_lcm: usize = input.iter().map(|v| v.test_divisible).product();
    //println!("Naiive lcm  {}", naive_lcm);
    //println!("Correct lcm  {}", lcm);

    for round in (1..=10000) {
        for monkey_idx in (0..input.len()) {
            while let Some(item_worry) = input[monkey_idx].item_worry_levels.pop_front() {
                monkey_inspections[monkey_idx] += 1;
                let mut worry = item_worry;
                let op_items: Vec<&str> = input[monkey_idx]
                    .worry_operation
                    .split(" ")
                    .take(3)
                    .collect();
                let first: usize = op_items[0].parse().unwrap_or(worry);
                let op = match op_items[1] {
                    "*" => |a: usize, b: usize| a * b,
                    "+" => |a: usize, b: usize| a + b,
                    _ => panic!("unexpected operator"),
                };
                let second: usize = op_items[2].parse().unwrap_or(worry);
                worry = op(first, second);
                // as we are only interested in the modulo of this number,
                // it can be safely reduced with lcm of all `.test_divisible` numbers
                worry %= naive_lcm;
                let throw_idx = if worry % input[monkey_idx].test_divisible == 0 {
                    input[monkey_idx].test_true
                } else {
                    input[monkey_idx].test_false
                };
                input[throw_idx].item_worry_levels.push_back(worry);
            }
        }
    }

    let out: usize = monkey_inspections.iter().sorted().rev().take(2).product();
    Output::U64(out as u64)
}
