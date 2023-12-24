use std::array;

use crate::utils::{
    helpers::{get_file_lines, get_input_as_matrix},
    matrix::Matrix,
};

pub fn part1() {
    let matrix = get_input_as_matrix("./src/days/day11/input.txt");

    let path_length_sum = get_path_length_sum(&matrix, 2);

    println!("Solution to Day 11 Part 1 : {path_length_sum}")
}

pub fn part2() {
    let input_vec: Vec<char> = get_file_lines("./src/days/day11/input.txt")
        .into_iter()
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let num_rows_cols = f32::sqrt(input_vec.len() as f32) as usize;

    let matrix = Matrix::new(num_rows_cols, num_rows_cols, input_vec);

    let path_length_sum = get_path_length_sum(&matrix, 1_000_000);

    println!("Solution to Day 11 Part 2 : {path_length_sum}")
}

fn get_path_length_sum(matrix: &Matrix, expansion_factor: i64) -> i64 {
    let mut galaxy_coords: Vec<(usize, usize)> = vec![];
    for (i, c) in matrix.iter().enumerate() {
        if *c == '#' {
            galaxy_coords.push(matrix.as_coordinates(i))
        }
    }

    let mut sum = 0;

    for (i, coords) in galaxy_coords.iter().enumerate() {
        for j in galaxy_coords.iter().skip(i + 1) {
            sum += get_distance(matrix, *coords, *j, expansion_factor)
        }
    }

    sum
}

fn get_distance(
    matrix: &Matrix,
    c1: (usize, usize),
    c2: (usize, usize),
    expansion_factor: i64,
) -> i64 {
    let mut num_expansions = 0;
    let min_row = usize::min(c1.0, c2.0);
    let max_row = usize::max(c1.0, c2.0);
    let min_col = usize::min(c1.1, c2.1);
    let max_col = usize::max(c1.1, c2.1);

    for i in min_row + 1..=max_row {
        if is_empty_row(matrix, i) {
            num_expansions += 1;
        }
    }

    for j in min_col + 1..=max_col {
        if is_empty_col(matrix, j) {
            num_expansions += 1;
        }
    }

    i64::abs(c2.0 as i64 - c1.0 as i64)
        + i64::abs(c2.1 as i64 - c1.1 as i64)
        + num_expansions * (expansion_factor - 1)
}

fn is_empty_row(matrix: &Matrix, row: usize) -> bool {
    for i in 0..matrix.cols {
        if matrix[(row, i)] == '#' {
            return false;
        }
    }
    true
}

fn is_empty_col(matrix: &Matrix, col: usize) -> bool {
    for i in 0..matrix.rows {
        if matrix[(i, col)] == '#' {
            return false;
        }
    }
    true
}
