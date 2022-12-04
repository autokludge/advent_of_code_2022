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
    let mut sum_badge_priorities = 0;

    for group in input.chunks(3) {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                let v = item_value(c);
                println!("found badge {}=>{}", c, v);
                sum_badge_priorities += v;
                break;
            }
        }
    }

    Output::U32(sum_badge_priorities)
}
