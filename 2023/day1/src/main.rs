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


fn main() {
    let input_str = "1asdgbrtrtwonineeight4th6cas7as";
    let result = convert_numbers(input_str);
    println!("{}", result);
}
