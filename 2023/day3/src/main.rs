use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_number_indices(line: &str) -> Vec<Vec<u32>> {
    let mut number_indices: Vec<Vec<u32>> = Vec::new();
    let mut sub_vec: Vec<u32> = Vec::new();
    for (index, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            let u32_index: u32 = index.try_into().unwrap();
            if sub_vec.len() == 0 || sub_vec[sub_vec.len() - 1] - u32_index > 1 {
                sub_vec.push(u32_index);
            } else {
                number_indices.push(sub_vec);
                sub_vec = Vec::new();
            }
        }
    }
    number_indices
}

fn add_numbers_in_line(line: &str) -> u32 {
    let number_indices: Vec<Vec<u32>> = get_number_indices(&line);
    0
}
        

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        get_number_indices(&line);
    }

    Ok(())
}
