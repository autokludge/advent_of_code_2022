use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let max_tree_scenic_score = input
        .grid
        .iter()
        .enumerate()
        .map(|idx| input.idx_to_cell(idx.0))
        .map(|(c, r)| input.tree_scenic_score(c, r))
        .max()
        .unwrap();

    Output::U32(max_tree_scenic_score as u32)
}
