use std::io::Read;
use std::iter::zip;
use std::str::Split;
use crate::read_file;

pub fn solve_3() {
    let mut input = read_file("/Users/jonathan/Projekte/AdventOfCode/input_3.txt");

    let lines: Vec<&str> = input.lines().collect();

    let mut res: u64 = 0;

    for i in (0..lines.len()).step_by(3) {
        res += handle_group(lines.get(i).unwrap(), lines.get(i + 1).unwrap(), lines.get(i + 2).unwrap());
    }

    println!("{}", res);
}

fn handle_group(line_1: &str, line_2: &str, line_3: &str) -> u64 {
    let array_1 = process_line(line_1);
    let array_2 = process_line(line_2);
    let array_3 = process_line(line_3);


    for i in 0..52 {
        if array_1[i] != 0 && array_2[i] != 0 && array_3[i] != 0 {
            return (i+1) as u64;
        }
    }

    return 0;
}

fn process_line(line: &str) -> [u8;52] {
    let mut res: [u8;52] = [0;52];

    for i in line.bytes() {
        if 65 <= i && i <= 90 {
            let index = (i - 39) as usize;
            res[index] += 1;
        } else if 97 <= i && i <= 122 {
            let index = (i - 97) as usize;
            res[index] += 1;
        }
    }

    res
}


fn get_score_for_line(split: (&str, &str)) -> u64 {
    let mut array: [u8; 2 * 26] = [0; 52];
    let mut array_2: [u8; 2 * 26] = [0; 52];

    for (x, y) in zip(split.0.bytes(), split.1.bytes()) {
        if 65 <= x && x <= 90 {
            let index = (x - 39) as usize;
            array[index] += 1;
        } else if 97 <= x && x <= 122 {
            let index = (x - 97) as usize;
            array[index] += 1;
        }

        if 65 <= y && y <= 90 {
            let index = (y - 39) as usize;
            array_2[index] += 1;
        } else if 97 <= y && y <= 122 {
            let index = (y - 97) as usize;
            array_2[index] += 1;
        }
    }

    for i in 0..52 {
        if array[i] != 0 && array_2[i] != 0 {
            return (i + 1) as u64;
        }
    }

    return 0;
}
