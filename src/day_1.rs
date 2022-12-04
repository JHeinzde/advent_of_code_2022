use std::fs::File;
use std::io::Read;

pub fn solve_1() {
    let mut file = File::open("/Users/jonathan/Projekte/AdventOfCode/input.txt").expect("To open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read input file");

    let mut sums: Vec<u64> = Vec::new();
    let mut sum = 0;

    for line in content.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
            continue;
        }

        let i = line.parse::<u64>().expect("To parse the line");
        sum += i;
    }

    sums.sort();

    let all_sum = sums.get(sums.len() - 1).unwrap() + sums.get(sums.len() - 2).unwrap() + sums.get(sums.len() - 3).unwrap();

    println!("{}", all_sum);
}
