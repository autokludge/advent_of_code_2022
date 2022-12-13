use std::fs::File;

use crate::day07::Input;

use super::{arenatree::ArenaTree, ElfFile};

const INPUT: &str = include_str!("../../input/07/input");
const EXAMPLE: &str = include_str!("../../input/07/example");

pub fn read() -> Input {
    create_elf_directory_from_input(INPUT)
}

pub fn readex() -> Input {
    create_elf_directory_from_input(EXAMPLE)
}

fn create_elf_directory_from_input(input: &str) -> ArenaTree<ElfFile> {
    let mut tree: ArenaTree<ElfFile> = ArenaTree::default();
    let mut current_dir = tree.node(ElfFile::root());

    for (no, line) in input.lines().enumerate() {
        //println!("l:{} -> {}", no, line);
        if line.starts_with("$ cd ") {
            let path = line.strip_prefix("$ cd ").unwrap();
            tree.cd(path);
            continue;
        }
        if line.starts_with("dir ") {
            tree.mkdir(line.strip_prefix("dir ").unwrap());
            continue;
        }
        if line.starts_with(char::is_numeric) {
            let (size, filename) = line.split_once(" ").unwrap();
            tree.touch(filename, size.parse().unwrap());
            continue;
        }
        //println!("line ignored: {}", line);
    }

    tree
}
