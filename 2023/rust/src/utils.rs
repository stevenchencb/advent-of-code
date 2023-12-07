use regex::Regex;
use std::fs::read_to_string;

pub fn get_file_lines(file: &str) -> Vec<String> {
    return read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

pub fn extract<'a>(s: &'a str, regex: &'a str) -> Vec<&'a str> {
    let regex = Regex::new(regex).unwrap();

    return regex.find_iter(s).map(|m| m.as_str()).collect();
}
