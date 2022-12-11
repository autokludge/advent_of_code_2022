use std::fmt::format;

use super::DirFile;
use crate::day07::{Input, Output};
use itertools::Itertools;

pub fn solve(input: &Input) -> Output {
    println!("{}", input);

    let dir_sizes_under_100_000: u64 = input
        .arena
        .iter()
        .filter(|d| d.val.dirfile == DirFile::Dir)
        .map(|d| d.size_on_disk.clone())
        .filter(|v| *v < 100000 as u64)
        .sum();

    Output::U64(dir_sizes_under_100_000)
}
