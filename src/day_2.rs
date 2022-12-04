use std::fs::File;
use std::io::Read;
use std::str::Split;

pub fn solve_2() {
    let mut file = File::open("/Users/jonathan/Projekte/AdventOfCode/input_2")
        .expect("could not open input file");
    let mut content = String::new();
    let _ = file.read_to_string(&mut content).expect("could not read from input file");

    let result  = content.lines().map(|line| line.trim())
        .map(|line| line.split(" "))
        .map(|split| split_to_result(split))
        .reduce(|one, two| one + two)
        .unwrap();

    println!("{}", result);
}

pub fn split_to_result(split: Split<&str>) -> u64 {
    let t_vec: Vec<&str> = split.collect();
    let vals = t_vec.as_slice();
    let you = vals[1];

    // A Stone 1, B Paper 2, C Sicssors 3
    // X lose 0 , Y draw 3 , Z win 6

    let match_points = match vals {
        ["A", "X"] => 3,
        ["A", "Y"] => 4,
        ["A", "Z"] => 8,
        ["B", "X"] => 1,
        ["B", "Y"] => 5,
        ["B", "Z"] => 9,
        ["C", "X"] => 2,
        ["C", "Y"] => 6,
        ["C", "Z"] => 7,
        _ => 0,
    };

    return match_points;
}
