use std::fs::File;
use std::io::{prelude::*, BufReader};

fn iterate_vec(input_vec: &Vec<Vec<char>>) -> u32 {
    let row_num = input_vec.len();
    let col_num = input_vec[0].len();
    for r in 0..row_num {
        let mut temp_row: Vec<char> = Vec::new();
        for c in 0..col_num {
            if input_vec[r][c].is_digit(10) {
                temp_row[c] = input_vec[r][c];
            } else if input_vec[r][c] != '.' {
                temp_row[c] = input_vec[r][c];
            } else {
                if input_vec[r - 1][c] != '.' {
                    temp_row[c] = input_vec[r - 1][c];
                } else if input_vec[r + 1][c] != '.' {
                    temp_row[c] = input_vec[r + 1][c];
                } else {
                    temp_row[c] = input_vec[r][c];
                }
            }
            if input_vec[r - 1][c] != '.'{
                if
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut input_vec: Vec<Vec<char>> =  Vec::new();
    let mut total = 0;
    let mut line_num = 0;
    for line in reader.lines() {
        input_vec.push(line?.chars().collect());
    }
    println!("{:?}", input_vec);

    Ok(())
}
