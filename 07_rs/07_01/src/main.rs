use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open(".\\input.txt")?;
    let reader = BufReader::new(file);

    let mut line_vec_raw: Vec<String> = Vec::new();
    let mut split_lines_vec: Vec<Vec<&str>> = Vec::new();
    let mut split_lines_vec_cut: Vec<Vec<&str>> = Vec::new();
    let mut line_vec: Vec<&Vec<&str>> = Vec::new();

    line_vec_raw = reader.lines().map(|l| l.expect("Couldn't parse lines")).collect();
    split_lines_vec = line_vec_raw.iter().map(|l| l.split_whitespace().collect()).collect();
    line_vec = split_lines_vec.iter().filter(|l| l.len() > 2).collect();

    for line in &line_vec {
        let node = line[0];
        let mut count = 0;

        for search_line in &line_vec {
            let replaced_line: Vec<String> = search_line.iter().map(|l| l.replace(",", "")).collect();
            if replaced_line.contains(&node.to_string()) {
                count += 1;
            }
        }

        if count <= 1 {
            println!(" {:?}", line);
        }
    }

    Ok(())
}


