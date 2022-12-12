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
    println!("{}", cycle_values.len());
    for x in (0..240) {
        let pixel = match cycle_values[x + 1].0 - (x % 40) as i64 {
            (-1..=1) => '#',
            _ => '.',
        };

        print!("{}", pixel);
        if [40, 80, 120, 160, 200, 240].contains(&(x + 1)) {
            println!();
        }
    }

    Output::I64(signal_total)
}
