use crate::day05::{Input, Output};

pub fn solve(input: &mut Input) -> Output {
    let mut crate_stacks = &mut input.stacks;
    for instruction in &input.instructions {
        let (m, s, t) = (
            instruction.crates_to_move,
            instruction.source,
            instruction.target,
        );
        let mut temp: Vec<char> = vec![];

        let new_len = crate_stacks[s as usize].len() - m as usize;
        let mut pick_up: Vec<char> = crate_stacks[s as usize].drain(new_len..).collect();
        &crate_stacks[t as usize].append(&mut pick_up);
    }
    //collect tops of stacks to string
    let out = crate_stacks
        .into_iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    Output::String(out)
}
