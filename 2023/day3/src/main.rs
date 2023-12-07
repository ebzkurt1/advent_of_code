use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_number_indices(line: &str) -> (Vec<char>, Vec<Vec<u32>>) {
    let mut number_indices: Vec<Vec<u32>> = Vec::new();
    let mut line_chars: Vec<char> = Vec::new();
    let mut sub_vec: Vec<u32> = Vec::new();
    for (index, c) in line.chars().enumerate() {
        line_chars.push(c);
        if c.is_digit(10) {
            let u32_index: u32 = index.try_into().unwrap();
            if sub_vec.is_empty() || u32_index - sub_vec.last().unwrap_or(&0) <= 1 {
                sub_vec.push(u32_index);
            } else {
                number_indices.push(sub_vec);
                sub_vec = Vec::new();
            }
        } else {
            if !sub_vec.is_empty() {
                number_indices.push(sub_vec);
                sub_vec = Vec::new();
            }
        }
    }
    println!("{:?}", number_indices);
    (line_chars, number_indices)
}

fn add_numbers_in_line(prev_line: &str, current_line: &str, next_line: &str, total: &u32) {
    let (prev_chars, _) = get_number_indices(prev_line);
    let (current_chars, current_indices) = get_number_indices(current_line);
    let (next_chars, _) = get_number_indices(next_line);
    for (num_index, sub_vec) in current_indices.iter().enumerate() {
        let mut add_this_num: bool = false;
        for (c_index, c) in sub_vec.iter().enumerate() {
            if (prev_chars[c_index - 1] != '.' && !prev_chars[c_index - 1].is_digit()) ||
                (prev_chars[c_index] != '.' && !prev_chars[c_index].is_digit()) ||
                (prev_chars[c_index + 1] != '.' && !prev_chars[c_index + 1].is_digit()) ||
                (current_chars[c_index - 1] != '.' && !current_chars[c_index - 1].is_digit()) ||
                (current_chars[c_index] != '.' && !current_chars[c_index].is_digit()) ||
                (current_chars[c_index + 1] != '.' && !current_chars[c_index + 1].is_digit()) ||
                (next_chars[c_index - 1] != '.' && !next_chars[c_index - 1].is_digit()) ||
                (next_chars[c_index] != '.' && !next_chars[c_index].is_digit()) ||
                (next_chars[c_index + 1] != '.' && !next_chars[c_index + 1].is_digit()) {
                    add_this_num = true;
                    continue;
                }
        }
        if add_this_num {
            let actual_num: u32 = sub_vec
                .iter()
                .map(|&index| current_line.chars().nth(index as usize).unwrap())
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            println!("{}", actual_num);
            total += actual_num;
        }
    }
}
        

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut line_vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        line_vec.push(line);
    }
    let n = line_vec.len();
    for index in 1..n - 1 {
        add_numbers_in_line(&line_vec[index - 1], &line_vec[index], &line_vec[index + 1], &total);
    }

    Ok(())
}
