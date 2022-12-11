use itertools::Itertools;

use super::DirFile;
use crate::day07::{Input, Output};

pub fn solve(input: &Input) -> Output {
    println!("{}", input);

    let disk_capacity: i64 = 70_000_000;
    let update_requirement: i64 = 30_000_000;
    let root_size = input.arena[0].size_on_disk as i64;
    let free_requirement: i64 = update_requirement - (disk_capacity - root_size);

    let smallest_dir_size: u64 = input
        .arena
        .iter()
        .filter(|d| d.val.dirfile == DirFile::Dir)
        .map(|d| d.size_on_disk.clone())
        .filter(|v| *v as i64 >= free_requirement)
        .min()
        .unwrap();

    Output::U64(smallest_dir_size)
}
