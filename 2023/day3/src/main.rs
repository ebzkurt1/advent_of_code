use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_number_indices(line: &str) -> (Vec<char>, Vec<Vec<u32>>) {
    let mut number_indices: Vec<Vec<u32>> = Vec::new();
    let line_chars: Vec<char> = line.chars().collect();
    let mut sub_vec: Vec<u32> = Vec::new();
    for (index, c) in line.chars().enumerate() {
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
    if !sub_vec.is_empty() {
        number_indices.push(sub_vec);
        sub_vec = Vec::new();
    }
    println!("{:?}", number_indices);
    (line_chars, number_indices)
}

fn add_for_first_line(current_line: &str, next_line: &str, total: &mut u32) {
    let (current_chars, current_indices) = get_number_indices(current_line);
    let (next_chars, _) = get_number_indices(next_line);
    for sub_vec in current_indices.iter() {
        let mut add_this_num: bool = false;
        for c_index in sub_vec.iter() {
            if c_index > &0 {
                let c_index = *c_index as usize;
                if (current_chars[c_index - 1] != '.' && !current_chars[c_index - 1].is_digit(10)) ||
                    (current_chars[c_index] != '.' && !current_chars[c_index].is_digit(10)) ||
                    (current_chars[c_index + 1] != '.' && !current_chars[c_index + 1].is_digit(10)) ||
                    (next_chars[c_index - 1] != '.' && !next_chars[c_index - 1].is_digit(10)) ||
                    (next_chars[c_index] != '.' && !next_chars[c_index].is_digit(10)) ||
                    (next_chars[c_index + 1] != '.' && !next_chars[c_index + 1].is_digit(10)) {
                        add_this_num = true;
                        continue;
                    }
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
            *total += actual_num;
        }
    }
}

fn add_numbers_in_line(prev_line: &str, current_line: &str, next_line: &str, total: &mut u32) {
    let (prev_chars, _) = get_number_indices(prev_line);
    let (current_chars, current_indices) = get_number_indices(current_line);
    let (next_chars, _) = get_number_indices(next_line);
    for sub_vec in current_indices.iter() {
        let mut add_this_num: bool = false;
        for c_index in sub_vec.iter() {
            if c_index > &0 && *c_index as usize != current_chars.len() - 1 {
                let c_index = *c_index as usize;
                if (prev_chars[c_index - 1] != '.' && !prev_chars[c_index - 1].is_digit(10)) ||
                    (prev_chars[c_index] != '.' && !prev_chars[c_index].is_digit(10)) ||
                    (prev_chars[c_index + 1] != '.' && !prev_chars[c_index + 1].is_digit(10)) ||
                    (current_chars[c_index - 1] != '.' && !current_chars[c_index - 1].is_digit(10)) ||
                    (current_chars[c_index] != '.' && !current_chars[c_index].is_digit(10)) ||
                    (current_chars[c_index + 1] != '.' && !current_chars[c_index + 1].is_digit(10)) ||
                    (next_chars[c_index - 1] != '.' && !next_chars[c_index - 1].is_digit(10)) ||
                    (next_chars[c_index] != '.' && !next_chars[c_index].is_digit(10)) ||
                    (next_chars[c_index + 1] != '.' && !next_chars[c_index + 1].is_digit(10)) {
                        add_this_num = true;
                        continue;
                    }
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
            *total += actual_num;
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
        add_numbers_in_line(&line_vec[index - 1], &line_vec[index], &line_vec[index + 1], &mut total);
        println!("{}", total);
    }
    add_for_first_line(&line_vec[0], &line_vec[1], &mut total);
    println!("{}", total);
    add_for_first_line(&line_vec[line_vec.len() - 1], &line_vec[line_vec.len() - 2], &mut total);
    println!("{}", total);

    Ok(())
}
