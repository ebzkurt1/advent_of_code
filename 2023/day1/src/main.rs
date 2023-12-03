use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn convert_numbers(input_str: &str) -> String {
    let result: String = input_str
        .chars()
        .map(|c| match c {
            '1' => "one".to_string(),
            '2' => "two".to_string(),
            '3' => "three".to_string(),
            '4' => "four".to_string(),
            '5' => "five".to_string(),
            '6' => "six".to_string(),
            '7' => "seven".to_string(),
            '8' => "eight".to_string(),
            '9' => "nine".to_string(),
            '0' => "zero".to_string(),
            _ => c.to_string(),
        })
        .collect();
    result
}

fn map_written_to_int(input_str: &str) -> u32 {
    let result: u32 = match input_str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    };
    result
}

fn find_number_words(input: &str) -> Vec<String> {
    let re = Regex::new(r"(zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    re.find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn get_numbers(number_vec: Vec<String>) -> u32 {
    let result = match number_vec.len() {
        0 => {
            println!("No numbers found");
            0
        }
        1 => {
            let num = map_written_to_int(number_vec.first().unwrap());
            num * 11
        }
        _ => {
            let num1 = map_written_to_int(number_vec.first().unwrap());
            let num2 = map_written_to_int(number_vec.last().unwrap());
            num1 * 10 + num2
        }
    };
    //println!("Result: {}", result);
    result
}

fn main() -> std::io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        //println!("{}", line);
        let result = convert_numbers(&line);
        //println!("{}", result);
        let words = find_number_words(&result);
        //println!("{:?}", words);
        total += get_numbers(words);
    }
    println!("Total: {}", total);
    Ok(())
}
