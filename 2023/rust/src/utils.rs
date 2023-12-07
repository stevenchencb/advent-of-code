use std::fs::read_to_string;

pub fn get_file_lines(file: &str) -> Vec<String> {
    return read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

pub fn get_numbers(s: &str) -> Vec<i64> {
    return s
        .chars()
        .filter(|c| c.is_digit(10) || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("not a number"))
        .collect();
}
