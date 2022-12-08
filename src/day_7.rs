use crate::day_7::FileType::DIRECTORY;
use crate::read_file;


struct FileTreeNode {
    name: String,
    size: Option<u64>,
    kind: FileType,
    content: Vec<Box<FileTreeNode>>,
    parent: Option<Box<FileTreeNode>>
}

enum FileType {
    FILE,
    DIRECTORY,
}

trait Tree {
    fn get_size(&self) -> u64;
    fn next(&self) -> &Vec<Box<FileTreeNode>>;
    fn add_child(&mut self, child: Box<FileTreeNode>);
    fn ch_dir(&mut self, dir_name: &str) -> Box<FileTreeNode>;
}

impl Tree for FileTreeNode {
    fn get_size(&self) -> u64 {
        match self.size {
            Some(size) => size,
            None => self.content
                .iter()
                .map(|f| f.get_size())
                .reduce(|i, j| i + j)
                .unwrap()
        }
    }

    fn next(&self) -> &Vec<Box<FileTreeNode>> {
        &self.content
    }

    fn add_child(&mut self, child: Box<FileTreeNode>) {
        self.content.push(child);
    }

    fn ch_dir(&mut self, dir_name: &str) -> &mut FileTreeNode {
        self.content.iter().filter(|s| s.kind == DIRECTORY)
            .find(|x| x.name == dir_name)
            .unwrap()
            .as_mut()
    }
}


pub fn solve_7() {
    let content = read_file("/Users/jonathan/Projekte/AdventOfCode/input_7.txt");

    for line in content.lines() {
        if line.starts_with("$") {
            match line {
                "$ cd" => (),
                "$ ls" => (),
                _ => (),
            }
        }
    }
}
