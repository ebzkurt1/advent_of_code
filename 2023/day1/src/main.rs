use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut final_sum = 0;

    for line in reader.lines() {
        let mut vec = Vec::new();
        for c in line?.chars(){
            let num = match c.to_string().parse::<i32>() {
                Ok(i) => i,
                Err(_e) => -1,
            };
            if num >= 0 {
                vec.push(num.to_string());
            }
        }
        let mut str_num = String::new();
        if vec.len() > 1 {
            str_num.push_str(&vec[0]);
            str_num.push_str(&vec[vec.len() - 1]);
        }
        else if vec.len() == 1 {
            str_num.push_str(&vec[0]);
            str_num.push_str(&vec[0]);
        }
        if str_num != "" {
            let line_num = str_num.parse::<i32>();
            final_sum += line_num.unwrap();
        }
    }
    println!("{}", final_sum);


    Ok(())
}
