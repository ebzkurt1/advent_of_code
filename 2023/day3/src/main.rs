use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_number_indeces(line: &str) -> Vec<u32> {
    let mut number_indices: Vec<u32> = Vec::new();
    for (index, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            number_indices.push(index.try_into().unwrap());
        }
    }
    number_indices
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        get_number_indeces(&line);
    }

    Ok(())
}
