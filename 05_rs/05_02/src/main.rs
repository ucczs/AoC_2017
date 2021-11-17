use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open(".\\input.txt")?;
    let reader = BufReader::new(file);

    let mut number_list:Vec<i32> = Vec::new();

    for line in reader.lines() {
        number_list.push(line.unwrap().parse().unwrap());
    }

    let mut current_index:i32 = 0;
    let mut next_index:i32 = 0;
    let mut step_count:i32 = 0;

    while next_index >= 0 && next_index < number_list.len() as i32 {
        step_count += 1;

        current_index = next_index;
        next_index += number_list[current_index as usize];

        if number_list[current_index as usize] >= 3 {
            number_list[current_index as usize] -=1;
        }
        else {
            number_list[current_index as usize] +=1;
        }
    }

    println!("Step count: {}", step_count);

    Ok(())
}


