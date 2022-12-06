use crate::day05::{Input, Output};

pub fn solve(input: &mut Input) -> Output {
    let mut crate_stacks = &mut input.stacks;
    for instruction in &input.instructions {
        let (m, s, t) = (
            instruction.crates_to_move,
            instruction.source,
            instruction.target,
        );
        for i in (0..m) {
            let temp = crate_stacks[s as usize].pop().unwrap();
            &crate_stacks[t as usize].push(temp);
        }
    }
    //collect tops of stacks to string
    let out = crate_stacks
        .into_iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    Output::String(out)
}
