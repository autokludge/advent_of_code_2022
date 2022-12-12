use crate::day10::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut cycle_values: Vec<(i64, i64)> = vec![(1, 1)];

    let mut current_cycle: usize = 0;
    let mut current_cycle_value: i64 = 1;

    let mut counter = 0;
    for instruction in input.iter() {
        let mut time: usize;
        let mut change: i64;
        if &instruction[..] == "noop" {
            time = 1;
            change = 0;

            cycle_values.push((current_cycle_value, current_cycle_value + change));
        } else {
            time = 2;
            change = instruction.split_once(" ").unwrap().1.parse().unwrap();

            cycle_values.push((current_cycle_value, current_cycle_value));
            cycle_values.push((current_cycle_value, current_cycle_value + change));
            current_cycle_value += change;
        }
    }

    let mut signal_total: i64 = 0;
    for x in (20..=220).step_by(40) {
        let signal_strength = x as i64 * cycle_values[x].0;
        println!("line: {}  signal strength {}", x, signal_strength);
        signal_total += signal_strength;
    }

    Output::I64(signal_total)
}
