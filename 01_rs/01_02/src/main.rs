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
    let length_vec = input_digit_vec.len();
    let index_addition = length_vec / 2;

    for i in 0..length_vec-1 {
        let comparison_index: usize = (i + index_addition) % length_vec;

        if input_digit_vec[i] == input_digit_vec[comparison_index] {
            sum += input_digit_vec[i];
        }
    }

    println!("Result: {}", sum);

    Ok(())
}


