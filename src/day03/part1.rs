use crate::day03::{Input, Output};

pub fn item_value(input: char) -> u32 {
    let x = input.into();
    match x {
        (97..=123) => x - 96,
        (65..=91) => x - 38,
        _ => x,
    }
}

pub fn solve(input: &Input) -> Output {
    let mut sum_priorities = 0;
    for line in input {
        let (first, second) = line.split_at(line.len() / 2);

        for c in first.chars() {
            if second.contains(c) {
                sum_priorities += item_value(c);
                break;
            }
        }
    }

    Output::U32(sum_priorities)
}
