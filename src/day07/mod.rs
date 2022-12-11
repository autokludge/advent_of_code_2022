pub mod arenatree;
pub mod input;
pub mod part1;
pub mod part2;

use std::fmt::{Display, Error, Formatter};

use crate::{Output, Part};
use arenatree::ArenaTree;

use self::arenatree::Node;

#[derive(PartialEq, Default, Debug)]
pub enum DirFile {
    File,
    #[default]
    Dir,
}

#[derive(Debug)]
struct NotFoundError(String);
#[derive(Debug)]
struct MakeDirError(String);
#[derive(Debug)]
struct MakeFileError(String);

#[derive(PartialEq, Default, Debug)]
pub struct ElfFile {
    pub file_name: String,
    pub file_path: String,
    pub dirfile: DirFile,
    pub file_size: u32,
}

impl ElfFile {
    pub fn new(file_name: String, file_path: String, dirfile: DirFile, size_on_disk: u32) -> Self {
        Self {
            file_name,
            file_path,
            dirfile,
            file_size: size_on_disk,
        }
    }

    pub fn newfile(file_name: String, file_path: String, size_on_disk: u32) -> Self {
        ElfFile::new(file_name, file_path, DirFile::File, size_on_disk)
    }

    pub fn newdir(file_name: String, file_path: String) -> Self {
        ElfFile::new(file_name, file_path, DirFile::Dir, 0)
    }

    pub fn root() -> Self {
        Self {
            file_name: "/".into(),
            file_path: "/".into(),
            dirfile: DirFile::Dir,
            file_size: 0,
        }
    }
}

impl Display for ArenaTree<ElfFile> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let idx = 0;
        let decendants = self.collect_decendants(idx);

        for dec in decendants {
            let size_on_disk = format!("(size on disk: {})", self.size_on_disk(dec));
            let prefix = " |".repeat(self.depth(dec));
            let display = format!("{}", self.arena[dec]);
            write!(f, "{:<30}\t{}{}\n", size_on_disk, prefix, display);
        }
        Ok(())
    }
}
impl Display for Node<ElfFile> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.val.dirfile {
            DirFile::Dir => write!(f, "- {} (dir)", self.val.file_name),
            DirFile::File => write!(
                f,
                "- {} (file, size={})",
                self.val.file_name, self.val.file_size
            ),
        }
    }
}

impl ArenaTree<ElfFile> {
    fn cd(&mut self, path: &str) -> Result<usize, NotFoundError> {
        let mut idx: Option<usize> = None;
        if path == "/" {
            for node in &self.arena {
                if node.val.file_name == "/" {
                    idx = Some(node.idx);
                    break;
                }
            }
        }
        if path == ".." {
            idx = Some(self.current().parent.unwrap());
        }
        for cidx in &self.current().children {
            let c = &self.arena[*cidx];
            if c.val.dirfile == DirFile::Dir && c.val.file_name == path {
                idx = Some(*cidx);
                break;
            }
        }
        match idx {
            Some(idx) => {
                // println!(
                //     "changed dir idx:{} name: {}",
                //     idx, &self.arena[idx].val.file_name
                // );
                self.set_current(idx);
                Ok(idx)
            }
            None => Err(NotFoundError(format!("dir {} not found", path))),
        }
    }

    fn mkdir(&mut self, path: &str) -> Result<usize, MakeDirError> {
        // Cannot create directory with name that exists
        for cidx in &self.current().children {
            let c = &self.arena[*cidx];
            if c.val.file_name == path {
                return Err(MakeDirError(format!("file {} already exists", path)));
            }
        }

        let parent_path = &self.current().val.file_path;
        let file_path = format!("{}{}/", parent_path, path);

        // Make directory file, add to arena
        let dir = ElfFile::newdir(path.into(), file_path);
        let dir_node_idx = self.node(dir);
        //println!("new node id: {} name:{}", dir_node_idx, path);

        // link new directory to parent
        self.arena[self.current_node].children.push(dir_node_idx);
        self.arena[dir_node_idx].parent = Some(self.current_node);

        Ok(dir_node_idx)
    }

    fn touch(&mut self, path: &str, size: u32) -> Result<usize, MakeFileError> {
        // Cannot create directory with name that exists
        for cidx in &self.current().children {
            let c = &self.arena[*cidx];
            if c.val.file_name == path {
                return Err(MakeFileError(format!("file {} already exists", path)));
            }
        }

        let parent_path = &self.current().val.file_path;
        let file_path = format!("{}{}/", parent_path, path);

        // Make directory file, add to arena
        let file = ElfFile::newfile(path.into(), file_path, size);
        let file_node_idx = self.node(file);
        //println!("new node id: {} name:{}", file_node_idx, path);

        // link new directory to parent
        self.arena[self.current_node].children.push(file_node_idx);
        self.arena[file_node_idx].parent = Some(self.current_node);

        // println!("touch created idx {}", file_node_idx);

        self.update_ancestors(file_node_idx, size);

        Ok(file_node_idx)
    }

    fn update_ancestors(&mut self, idx: usize, size_on_disk: u32) {
        let mut next_id = idx;
        let mut counter = 0;

        let leaf_is_file = self.arena[idx].val.dirfile == DirFile::File;
        while let Some(candidate_idx) = self.arena[next_id].parent {
            counter += 1;
            if leaf_is_file {
                self.arena[candidate_idx].size_on_disk += size_on_disk as u64;
            }
            self.arena[candidate_idx].descendant_files.push(idx);
            next_id = candidate_idx;
        }

        // println!("Update_ancestors on idx {} ran {} times", idx, counter);
    }

    fn size_on_disk(&self, idx: usize) -> u64 {
        self.arena[idx].size_on_disk
    }
}

pub type Input = ArenaTree<ElfFile>;

pub fn run(part: Part) -> Output {
    match part {
        Part::OneEx => {
            let exampleinput = input::readex();
            part1::solve(&exampleinput)
        }
        Part::One => {
            let input = input::read();
            part1::solve(&input)
        }
        Part::TwoEx => {
            let exampleinput = input::readex();
            part2::solve(&exampleinput)
        }
        Part::Two => {
            let input = input::read();
            part2::solve(&input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one_example() {
        let result = run(Part::OneEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two_example() {
        let result = run(Part::TwoEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
