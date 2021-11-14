use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open(".\\input.txt")?;
    let reader = BufReader::new(file);

    let mut input_digit_vec: Vec<u32> = Vec::new();

    for line in reader.lines() {
        for c in line.expect("lines failed").chars() {
            input_digit_vec.push(c.to_digit(10).unwrap());
        }
    }

    let mut sum = 0;
    for i in 0..input_digit_vec.len()-1 {
        if input_digit_vec[i] == input_digit_vec[i+1] {
            sum += input_digit_vec[i];
        }
    }

    if *input_digit_vec.last().unwrap() == input_digit_vec[0] {
        sum += input_digit_vec[0];
    }

    println!("Result: {}", sum);

    Ok(())
}


