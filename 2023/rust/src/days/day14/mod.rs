use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use crate::utils::matrix::Matrix;

pub fn part1() {
    let field_string = read_to_string("./src/days/day14/input.txt").unwrap();
    let field_chars: Vec<Vec<char>> = field_string.lines().map(|s| s.chars().collect()).collect();

    let rows = field_chars.len();
    let cols = field_chars[0].len();

    let mut matrix = Matrix::new(rows, cols, field_chars.into_iter().flatten().collect());
    tilt(&mut matrix);

    let sum = calculate_col_load(&matrix);

    println!("Solution for Day 14 Part 1 : {sum}");
}

pub fn part2() {
    let field_string = read_to_string("./src/days/day14/input.txt").unwrap();
    let field_chars: Vec<Vec<char>> = field_string.lines().map(|s| s.chars().collect()).collect();

    let rows = field_chars.len();
    let cols = field_chars[0].len();

    let mut matrix = Matrix::new(rows, cols, field_chars.into_iter().flatten().collect());
    let mut cache: HashMap<String, usize> = HashMap::new();

    let cycles = 1_000_000_000;
    let mut cycles_done = 0;

    // cycle length as in length of detected cycle during processing
    let mut cycle_length = 0;

    for i in 0..cycles {
        for _ in 0..4 {
            tilt(&mut matrix);
            matrix.rotate_right();
        }

        let matrix_string = matrix.to_string();
        // cycle detection
        if cache.contains_key(&matrix_string) {
            let previous_cycle = cache.get(&matrix_string).unwrap();
            cycle_length = i - previous_cycle;
            cycles_done += 1;
            break;
        }
        cache.insert(matrix.to_string(), i);
        cycles_done += 1;
    }

    // cycles left until 1_000_000_000
    let remaining_cycles = cycles - cycles_done;

    // cycles to do until a grid with the same load as the grid after 1_000_000_000 cycles is reached
    let cycles_to_do = remaining_cycles % cycle_length;

    for _ in 0..cycles_to_do {
        for _ in 0..4 {
            tilt(&mut matrix);
            matrix.rotate_right();
        }
    }

    let sum = calculate_col_load(&matrix);

    println!("Solution for Day 14 Part 2 : {sum}");
}

fn calculate_col_load(matrix: &Matrix) -> usize {
    let mut total = 0;

    for (i, row) in matrix.get_rows(0..matrix.rows).into_iter().enumerate() {
        let rocks_in_row = row
            .into_iter()
            .filter(|c| *c == 'O')
            .collect::<Vec<char>>()
            .len();
        total += rocks_in_row * (matrix.rows - i);
    }

    total
}

fn tilt(matrix: &mut Matrix) {
    let untilted_cols = matrix.get_cols(0..matrix.cols);

    for (i, mut col) in untilted_cols.into_iter().enumerate() {
        tilt_col(&mut col);
        matrix.replace_col(i, col)
    }
}

fn tilt_col(col: &mut [char]) {
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for i in 0..col.len() - 1 {
            if col[i] == '.' && col[i + 1] == 'O' {
                col.swap(i, i + 1);
                has_changed = true;
            }
        }
    }
}
