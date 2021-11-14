use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open(".\\input.txt")?;
    let reader = BufReader::new(file);

    let mut sum:i32 = 0;

    for line_wrapped in reader.lines() {
        let line = line_wrapped.unwrap();

        let line_splitted = line.split_whitespace();
        let mut values: Vec<i32> = Vec::new();

        for number in line_splitted {
            values.push(number.parse().unwrap());
        }

        let min_value = values.iter().min();
        let max_value = values.iter().max();
        sum += max_value.unwrap() - min_value.unwrap();
    }

    println!("{}", sum);

    Ok(())
}


