use std::fs::read_to_string;

use crate::utils::Matrix;

pub fn part1() {
    let fields: Vec<Vec<String>> = read_to_string("./src/days/day13/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|f| f.lines().map(String::from).collect())
        .collect();

    let mut sum = 0;

    for f in fields.iter() {
        let row_size = f.len();
        let col_size = f[0].len();
        let field_matrix = Matrix::new(
            row_size,
            col_size,
            f.iter()
                .flat_map(|s| s.chars().collect::<Vec<char>>())
                .collect(),
        );

        sum += check_reflection(&field_matrix);
    }

    println!("Solution for Day 13 Part 1 : {sum}")
}

pub fn part2() {
    let fields: Vec<Vec<String>> = read_to_string("./src/days/day13/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|f| f.lines().map(String::from).collect())
        .collect();

    let mut sum = 0;

    for f in fields.iter() {
        let row_size = f.len();
        let col_size = f[0].len();
        let field_matrix = Matrix::new(
            row_size,
            col_size,
            f.iter()
                .flat_map(|s| s.chars().collect::<Vec<char>>())
                .collect(),
        );

        sum += check_reflection_with_smudge(&field_matrix);
    }

    println!("Solution for Day 13 Part 2 : {sum}")
}

fn check_reflection(matrix: &Matrix<char>) -> usize {
    // check horizontal reflection
    for i in 0..matrix.rows - 1 {
        let v1 = matrix.get_rows(0..i + 1);
        let v2 = matrix.get_rows(i + 1..matrix.rows);

        let mut longer: Vec<String>;
        let mut shorter: Vec<String>;

        if v1.len() > v2.len() {
            longer = v1.iter().map(|c| c.iter().collect()).collect();
            shorter = v2.iter().map(|c| c.iter().collect()).collect();
        } else {
            longer = v2.iter().map(|c| c.iter().collect()).collect();
            shorter = v1.iter().map(|c| c.iter().collect()).collect();
        }

        if i >= (matrix.rows / 2) {
            longer.reverse();
        } else {
            shorter.reverse();
        }

        for c in shorter.clone().iter() {
            if longer[0] == *c {
                longer.remove(0);
                shorter.remove(0);
            }
        }

        if (longer.len() % 2) == 1 && shorter.is_empty() {
            return (i + 1) * 100;
        }
    }

    // check vertical reflection
    for i in 0..matrix.cols - 1 {
        let v1 = matrix.get_cols(0..i + 1);
        let v2 = matrix.get_cols(i + 1..matrix.cols);

        let mut longer: Vec<String>;
        let mut shorter: Vec<String>;

        if v1.len() > v2.len() {
            longer = v1.iter().map(|c| c.iter().collect()).collect();
            shorter = v2.iter().map(|c| c.iter().collect()).collect();
        } else {
            longer = v2.iter().map(|c| c.iter().collect()).collect();
            shorter = v1.iter().map(|c| c.iter().collect()).collect();
        }

        if i >= (matrix.cols / 2) {
            longer.reverse();
        } else {
            shorter.reverse();
        }

        for c in shorter.clone().iter() {
            if longer[0] == *c {
                longer.remove(0);
                shorter.remove(0);
            }
        }

        if (longer.len() % 2) == 1 && shorter.is_empty() {
            return i + 1;
        }
    }

    0
}

fn check_reflection_with_smudge(matrix: &Matrix<char>) -> i32 {
    // check horizontal reflection
    let mut h_index: i32 = -1;
    let mut h_smudge_index: i32 = -1;
    for i in 0..matrix.rows - 1 {
        let v1 = matrix.get_rows(0..i + 1);
        let v2 = matrix.get_rows(i + 1..matrix.rows);

        let mut longer: Vec<String>;
        let mut shorter: Vec<String>;

        if v1.len() > v2.len() {
            longer = v1.iter().map(|c| c.iter().collect()).collect();
            shorter = v2.iter().map(|c| c.iter().collect()).collect();
        } else {
            longer = v2.iter().map(|c| c.iter().collect()).collect();
            shorter = v1.iter().map(|c| c.iter().collect()).collect();
        }

        if i >= (matrix.rows / 2) {
            longer.reverse();
        } else {
            shorter.reverse();
        }

        let mut smudge_detected = false;

        for c in shorter.clone().iter() {
            if !smudge_detected && equal_with_smudge(&longer[0], c) {
                longer.remove(0);
                shorter.remove(0);
                smudge_detected = true;
            } else if longer[0] == *c {
                longer.remove(0);
                shorter.remove(0);
            }
        }

        if (longer.len() % 2) == 1 && shorter.is_empty() {
            if smudge_detected {
                h_smudge_index = (i + 1) as i32;
            } else {
                h_index = (i + 1) as i32;
            }
        }
    }

    // check vertical reflection
    let mut v_index: i32 = -1;
    let mut v_smudge_index: i32 = -1;
    for i in 0..matrix.cols - 1 {
        let v1 = matrix.get_cols(0..i + 1);
        let v2 = matrix.get_cols(i + 1..matrix.cols);

        let mut longer: Vec<String>;
        let mut shorter: Vec<String>;

        if v1.len() > v2.len() {
            longer = v1.iter().map(|c| c.iter().collect()).collect();
            shorter = v2.iter().map(|c| c.iter().collect()).collect();
        } else {
            longer = v2.iter().map(|c| c.iter().collect()).collect();
            shorter = v1.iter().map(|c| c.iter().collect()).collect();
        }

        if i >= (matrix.cols / 2) {
            longer.reverse();
        } else {
            shorter.reverse();
        }

        let mut smudge_detected = false;

        for c in shorter.clone().iter() {
            if !smudge_detected && equal_with_smudge(&longer[0], c) {
                longer.remove(0);
                shorter.remove(0);
                smudge_detected = true;
            } else if longer[0] == *c {
                longer.remove(0);
                shorter.remove(0);
            }
        }

        if (longer.len() % 2) == 1 && shorter.is_empty() {
            if smudge_detected {
                v_smudge_index = (i + 1) as i32;
            } else {
                v_index = (i + 1) as i32;
            }
        }
    }

    // prioritize smudge reflections
    if h_smudge_index > 0 {
        h_smudge_index * 100
    } else if v_smudge_index > 0 {
        v_smudge_index
    } else if h_index > 0 {
        h_index * 100
    } else if v_index > 0 {
        v_index
    } else {
        0
    }
}

fn equal_with_smudge(a: &str, b: &str) -> bool {
    let a_as_nums: Vec<i32> = a.chars().map(|s| if s == '.' { 0 } else { 1 }).collect();
    let b_as_nums: Vec<i32> = b.chars().map(|s| if s == '.' { 0 } else { 1 }).collect();

    let num_differences: i32 = a_as_nums
        .iter()
        .zip(b_as_nums.iter())
        .map(|(a, b)| a ^ b)
        .sum();

    num_differences == 1
}
