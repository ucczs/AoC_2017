use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

fn find_max_index(number_vec: &Vec<i32>) -> usize {
    let index_of_max: usize = number_vec
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();

    index_of_max
}

fn perform_next_step(number_vec: &mut Vec<i32>) {
    let mut vec_reversed:Vec<i32> = number_vec.clone();
    vec_reversed.reverse();
    let mut idx:usize = number_vec.len() - 1 - find_max_index(&vec_reversed);

    let mut available_value:i32 = number_vec[idx];
    number_vec[idx] = 0;
    idx = (idx + 1) % number_vec.len();

    while available_value > 0 {
        number_vec[idx] += 1;
        available_value -= 1;
        idx = (idx + 1) % number_vec.len();
    }
}

fn is_this_state_new(vec_history: &mut Vec<Vec<i32>>) -> bool {
    let len_before = vec_history.len();
    vec_history.sort_unstable();
    vec_history.dedup();
    let len_after = vec_history.len();

    if len_before == len_after {
        true
    }
    else {
        false
    }
    
}

fn main() -> io::Result<()> {
    let file = File::open(".\\input.txt")?;
    let reader = BufReader::new(file);

    let mut number_vec:Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_unwrap = line.unwrap();
        let line_splitted = line_unwrap.split_whitespace();

        for number in line_splitted {
            number_vec.push(number.parse().unwrap());
        }
    }

    let mut vec_history:Vec<Vec<i32>> = Vec::new();
    let mut state_never_seen_before:bool = true;
    let mut stepCount = 0;
    vec_history.push(number_vec.clone());

    while state_never_seen_before {
        perform_next_step(&mut number_vec);

        vec_history.push(number_vec.clone());
        
        state_never_seen_before = is_this_state_new(&mut vec_history);
        stepCount += 1;
    }

    let mut circle_counter = 0;
    let found_setting:Vec<i32> = number_vec.clone();
    perform_next_step(&mut number_vec);
    circle_counter += 1;

    while found_setting != number_vec {
        perform_next_step(&mut number_vec);
        circle_counter += 1;
    }

    println!("Step count: {}", circle_counter);

    Ok(())
}
