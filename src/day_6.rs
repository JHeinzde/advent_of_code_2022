use crate::read_file;

pub fn solve_6(){
    let content = read_file("/Users/jonathan/Projekte/AdventOfCode/input_6.txt");
    //let content = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let slice = content.as_bytes();

    // mjqjpqmgbljsphdz
    // tnvjfqwrcgsmlb
    // mjqjp
    //
    // qmgbljsphdztnv
    // jfqwrcgsmlb

    let mut state = [false;26];

    //for i in 0..slice.len()-4 {
    //    if slice[i] != slice[i+1] && slice[i] != slice[i+2] && slice[i] != slice[i+3]
    //        && slice[i + 1] != slice[i+2] && slice[i+1] != slice[i+3] && slice[i+2] != slice[i+3] {
    //        println!("{}", i+4);
    //        return
    //    }
    //}

    let mut i = 0;
    let mut group_counter = 0;
    loop {
        let index: usize = (slice[i + group_counter] - 97) as usize;
        if group_counter == 14 {
            println!("{}", i+group_counter+1);
            break
        }

        if state[index] == true {
            state = [false;26];
            i += 1;
            group_counter = 0;
        } else {
            state[index] = true;
        }

        group_counter += 1;
    }
}
