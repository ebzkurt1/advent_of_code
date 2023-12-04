use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn get_game_number(line: &str) -> u32 {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let game: Vec<&str> = re.find_iter(line)
        .map(|m| m.as_str()).collect();
    game[0].parse::<u32>().unwrap()
}

fn get_numbers_from_string(line: &str) -> u32 {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let numbers: Vec<&str> = re.find_iter(line)
        .map(|m| m.as_str())
        .collect();
    numbers[numbers.len() - 1].parse::<u32>().unwrap()
}

fn get_min_possible(line: &str) -> u32{
    let re = Regex::new(r"([0-9]+)").unwrap();
    let numbers: Vec<u32> = re.find_iter(line)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();
    *numbers.iter().min().unwrap()
}

fn check_bag(line: &str, red: u32, green: u32, blue: u32) -> bool {
    let blues: Vec<&str> = line.split("blue").collect();
    let reds: Vec<&str> = line.split("red").collect();
    let greens: Vec<&str> = line.split("green").collect();
    for sample in blues.iter().take(blues.len() - 1) {
        if get_numbers_from_string(sample) > blue {
            return false
        }
    }
    for sample in reds.iter().take(reds.len() - 1) {
        if get_numbers_from_string(sample) > red {
            return false
        }
    }
    for sample in greens.iter().take(greens.len() - 1) {
        if get_numbers_from_string(sample) > green {
            return false
        }
    }
    true
}

fn power_min_possible(line: &str) -> u32 {
    let blues: Vec<&str> = line.split("blue").collect();
    let reds: Vec<&str> = line.split("red").collect();
    let greens: Vec<&str> = line.split("green").collect();
    let mut min_blue = u32::MIN;
    let mut min_red = u32::MIN;
    let mut min_green = u32::MIN;
    for sample in blues.iter().take(blues.len() - 1) {
        let blue_sample = get_numbers_from_string(sample);
        if  blue_sample > min_blue {
            min_blue = blue_sample;
        }
    }
    for sample in reds.iter().take(reds.len() - 1) {
        let red_sample = get_numbers_from_string(sample);
        if  red_sample > min_red {
            min_red = red_sample;
        }
    }
    for sample in greens.iter().take(greens.len() - 1) {
        let green_sample = get_numbers_from_string(sample);
        if  green_sample > min_green {
            min_green = green_sample;
        }
    }
    println!("Min Blue: {}, Min Red: {}, Min Green: {}", min_blue, min_red, min_green);
    min_blue * min_red * min_green
}


fn main() -> std::io::Result<()> {
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let game_num = get_game_number(&line);
        // let condition = check_bag(&line, RED, GREEN, BLUE);
        let power = power_min_possible(&line);
        if power > 0 {
            total += power;
        }
    }
    println!("Total: {}", total);

    Ok(())
}
