use std::fs::File;
use std::io::{self, prelude::*, BufReader};

enum Movement {
    PlusX,
    PlusY,
    MinusX,
    MinusY
}

fn next_step(x_val: &mut i32, y_val: &mut i32, limit: &mut i32, action: &mut Movement) {
    match action {
        Movement::PlusX => {
            *x_val = *x_val + 1;
            if *x_val == *limit {
                *action = Movement::PlusY;
            }
        }
        Movement::PlusY => {
            *y_val = *y_val + 1;
            if *y_val == *limit {
                *action = Movement::MinusX;
            }
        }
        Movement::MinusX => {
            *x_val = *x_val - 1;
            if *x_val == -*limit {
                *action = Movement::MinusY;
            }
        }
        Movement::MinusY => {
            *y_val = *y_val - 1;
            if *y_val == -*limit {
                *action = Movement::PlusX;
            }
        }
    }
}

fn calculate_square_value(map: &Vec<[i32; 3]>, index: usize, x_coord: i32, y_coord: i32) -> i32 {
    let mut sum: i32 = 0;

    for i in 0..index {
        if map[i][0] <= x_coord+1 && map[i][0] >= x_coord-1 {
            if map[i][1] <= y_coord+1 && map[i][1] >= y_coord-1 {
                sum += map[i][2];
            }
        }
    }

    return sum;
}

fn main() -> io::Result<()> {
    let file = File::open(".\\input.txt")?;
    let reader = BufReader::new(file);

    let mut search_number = 0;
    for line in reader.lines() {
        search_number = line.unwrap().parse().unwrap();
    }

    let mut map:Vec<[i32; 3]> = Vec::new();

    let mut current_limit = 0;
    let mut current_action = Movement::PlusX;

    let mut x_val: i32 = 0;
    let mut y_val: i32 = 0;
    let mut result = 0;


    map.push([x_val,y_val, 1]);
    current_limit += 1;

    for i in 1..search_number {
        if (map[i-1][0] < 0) && (map[i-1][0] == map[i-1][1]) {
            current_limit += 1;
            println!("Limit increase at value {}", i);
        }

        next_step(&mut x_val, &mut y_val, &mut current_limit, &mut current_action);
        let square_value = calculate_square_value(&map, i, x_val, y_val);

        map.push([x_val, y_val, square_value]);

        if square_value > search_number as i32 {
            result = square_value;
            break;
        }
    }

    println!("Result: {}", result);

    Ok(())
}
