use std::collections::HashMap;

use crate::day06::{Input, Output};

pub fn solve(input: &Input) -> Output {
    for (idx, w) in input.windows(4).enumerate() {
        //println!("{:?}", w);

        let mut v = Vec::from(w);
        v.sort();
        v.dedup();

        if v.len() == 4 {
            return Output::String(format!("{}", idx + 4));
        }
    }
    Output::String("Coudn't find anything".to_owned())
}
