use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn get_game_number(line: &str) -> u32 {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let game: Vec<&str> = re.find_iter(line)
        .map(|m| m.as_str()).collect();
    game[0].parse::<u32>().unwrap()
}

fn check_bag(line: &str, red: u32, green: u32, blue: u32, game_num: u32) -> bool {
    let blues: Vec<&str> = line.split("blue").collect();
    println!("{:?}", blues);
    true
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
        let condition = check_bag(&line, RED, GREEN, BLUE, game_num);
    }

    Ok(())
}
