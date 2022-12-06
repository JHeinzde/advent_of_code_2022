use std::ops::{Range, RangeFull};
use std::str::Split;
use crate::read_file;

pub fn solve_4() {
    let content = read_file("/Users/jonathan/Projekte/AdventOfCode/input_4.txt");
    let res = content.lines()
        .map(|line| line.split(","))
        .map(|split| contain_each_other(split))
        .filter(|b| *b == true)
        .count();

    println!("{}", (14..=50).contains(&50));
    println!("{}", res);
}


fn contain_each_other(line: Split<&str>) -> bool {
    let ranges: Vec<&str> = line.collect();

    let range_0 = ranges.get(0).unwrap().split("-");
    let range_1 = ranges.get(1).unwrap().split("-");

    let range_0: Vec<&str> = range_0.collect();
    let range_1: Vec<&str> = range_1.collect();

    let range_0 = range_0.get(0).unwrap().parse::<u64>().unwrap()..=range_0.get(1).unwrap().parse::<u64>().unwrap();
    let range_1 = range_1.get(0).unwrap().parse::<u64>().unwrap()..=range_1.get(1).unwrap().parse::<u64>().unwrap();

    let r_0_r_1 = range_0.contains(&range_1.start()) || range_0.contains(&range_1.end());
    let r_1_r_0 = range_1.contains(&range_0.start()) || range_1.contains(&range_0.end());

    return r_0_r_1 || r_1_r_0;
}
