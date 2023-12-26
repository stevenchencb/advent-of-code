use regex::{Captures, Regex};
use std::fs::read_to_string;

use super::matrix::Matrix;

pub fn get_file_lines(file: &str) -> Vec<String> {
    read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn get_file_blocks(file: &str) -> Vec<Vec<String>> {
    read_to_string(file)
        .unwrap()
        .split("\n\n")
        .map(|b| b.lines().map(String::from).collect::<Vec<String>>())
        .collect()
}

pub fn get_input_as_matrix(file: &str) -> Matrix {
    let field_string = read_to_string(file).unwrap();
    let field_chars: Vec<Vec<char>> = field_string.lines().map(|s| s.chars().collect()).collect();

    let rows = field_chars.len();
    let cols = field_chars[0].len();

    Matrix::new(rows, cols, field_chars.into_iter().flatten().collect())
}

pub fn extract<'a>(s: &'a str, regex: &'a str) -> Vec<&'a str> {
    let regex = Regex::new(regex).unwrap();

    return regex.find_iter(s).map(|m| m.as_str()).collect();
}

pub fn extract_named<'a>(s: &'a str, regex: &'a str, group_name: &'a str) -> String {
    let regex = Regex::new(regex).unwrap();
    let extracted = regex.captures(s);

    match extracted {
        Some(extracted) => extracted[group_name].to_string(),
        None => String::new(),
    }
}

pub fn shoelace(vertices: &[(i32, i32)]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..vertices.len() {
        result += (vertices[i % vertices.len()].0 as f64
            * vertices[(i + 1) % vertices.len()].1 as f64)
            - (vertices[(i + 1) % vertices.len()].0 as f64 * vertices[i % vertices.len()].1 as f64)
    }

    result
}

pub fn picks_theorem(area: f64, num_of_boundaries: usize) -> usize {
    (area.abs() - (num_of_boundaries as f64 / 2.0) + 1.0) as usize
}
