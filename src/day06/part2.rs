use crate::day06::{Input, Output};

pub fn solve(input: &Input) -> Output {
    for (idx, w) in input.windows(14).enumerate() {
        let mut v = Vec::from(w);
        v.sort();
        v.dedup();

        if v.len() == 14 {
            return Output::String(format!("{}", idx + 14));
        }
    }
    Output::String("Coudn't find anything".to_owned())
}
