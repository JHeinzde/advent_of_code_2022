use std::collections::VecDeque;
use crate::read_file;

pub fn solve_5() {
    let mut stack_1 = vec!["H", "C", "R"];
    let mut stack_2 = vec!["B", "J", "H", "L", "S", "F"];
    let mut stack_3 = vec!["R", "M", "D", "H", "J", "T", "Q"];
    let mut stack_4 = vec!["S", "G", "R", "H", "Z", "B", "J"];
    let mut stack_5 = vec!["R", "P", "F", "Z", "T", "D", "C", "B"];
    let mut stack_6 = vec!["T", "H", "C", "G"];
    let mut stack_7 = vec!["S", "N", "V", "Z", "B", "P", "W", "L"];
    let mut stack_8 = vec!["R", "J", "Q", "G", "C"];
    let mut stack_9 = vec!["L", "D", "T", "R", "H", "P", "F", "S"];

    let mut stacks: Vec<Vec<&str>> = vec![stack_1, stack_2, stack_3, stack_4, stack_5, stack_6, stack_7, stack_8, stack_9];
    let content = read_file("/Users/jonathan/Projekte/AdventOfCode/input_5.txt");

    for line in content.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let (from, to, amount) = match split[..] {
            [_, a, _, f, _, t] => (f.parse::<usize>().unwrap_or(0), t.parse::<usize>().unwrap_or(0), a.parse::<u64>().unwrap_or(0)),
            _ => (0, 0, 0)
        };

        let mut tmp_stack = VecDeque::new();
        let mut stack = stacks.get_mut(from -1).unwrap();


        for _ in 0..amount {
            let element = stack.pop();
            tmp_stack.push_back(element.unwrap());
        }

        let mut stack = stacks.get_mut(to -1).unwrap();
        for _ in 0..amount {
            stack.push(tmp_stack.pop_back().unwrap());
        }
    }

    for stack in stacks {
        println!("{}", stack.last().unwrap())
    }
}
