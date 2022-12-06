use std::fs::File;
use std::io::Read;
use crate::day_1::solve_1;
use crate::day_2::solve_2;
use crate::day_3::solve_3;
use crate::day_4::solve_4;
use crate::day_5::solve_5;
use crate::day_6::solve_6;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() {
    //solve_1();
    //solve_2();
    //solve_3();
    //solve_4();
    //solve_5();
    solve_6();
}


pub fn read_file(name: &str) -> String {
    let mut file = File::open(name).expect("Open the input file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("To read the content of the input file");

    return content;
}
