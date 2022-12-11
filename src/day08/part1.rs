use crate::day08::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let trees_visible_from_outside = input
        .grid
        .iter()
        .enumerate()
        .map(|idx| input.idx_to_cell(idx.0))
        .filter(|(c, r)| input.cell_visible_from_outside(*c, *r))
        .count();

    Output::U32(trees_visible_from_outside as u32)
}
