use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

use crate::utils::{
    direction::Direction,
    helpers::get_input_as_matrix,
    matrix::{self, Matrix},
    node::Node,
};

pub fn part1() {
    // let matrix = get_input_as_matrix("./src/days/day17/input.txt");

    // let min_heat_loss = a_star(&matrix, (matrix.rows - 1, matrix.cols - 1));

    // println!("Solution to Day 17 Part 1 : {min_heat_loss}");
}

pub fn part2() {}

// modified A* algorithm with Manhattan distance as heuristic
// fn a_star(matrix: &Matrix, end: (usize, usize)) -> usize {
//     0
// }

fn get_neighbors(
    matrix: &Matrix,
    current_index: usize,
    current_direction: Direction,
    moves_in_line: &mut usize,
) -> Vec<usize> {
    let mut neighbors: Vec<usize> = vec![];

    match current_direction {
        Direction::Up => {
            // left boundary check
            if current_index % matrix.cols != 0 {
                neighbors.push(current_index - 1);
            }
            // upper boundary and straight line check
            if matrix.as_coordinates(current_index).0 != 0 && *moves_in_line < 3 {
                neighbors.push(current_index - matrix.cols);
            }
            // right boundary check
            if current_index % matrix.cols != matrix.cols - 1 {
                neighbors.push(current_index + 1);
            }
        }
        Direction::Down => {
            // left boundary check
            if current_index % matrix.cols != 0 {
                neighbors.push(current_index - 1);
            }
            // lower boundary and straight line check
            if matrix.as_coordinates(current_index).0 != matrix.rows - 1 && *moves_in_line < 3 {
                neighbors.push(current_index + matrix.cols);
            }
            // right boundary check
            if current_index % matrix.cols != matrix.cols - 1 {
                neighbors.push(current_index + 1);
            }
        }
        Direction::Left => {
            // left boundary and straight line check
            if current_index % matrix.cols != 0 && *moves_in_line < 3 {
                neighbors.push(current_index - 1);
            }
            // lower boundary check
            if matrix.as_coordinates(current_index).0 != matrix.rows - 1 {
                neighbors.push(current_index + matrix.cols);
            }
            // upper boundary check
            if matrix.as_coordinates(current_index).0 != 0 {
                neighbors.push(current_index - matrix.cols);
            }
        }
        Direction::Right => {
            // upper boundary check
            if current_index % matrix.cols == 0 {
                neighbors.push(current_index - matrix.cols);
            }
            // right boundary and straight line check
            if current_index % matrix.cols != matrix.cols - 1 && *moves_in_line < 3 {
                neighbors.push(current_index + 1);
            }
            // lower boundary check
            if matrix.as_coordinates(current_index).0 != matrix.rows - 1 {
                neighbors.push(current_index + matrix.cols);
            }
        }
    }

    neighbors
}

fn manhattan(matrix: &Matrix, i: usize, end: (usize, usize)) -> usize {
    let coordinates = matrix.as_coordinates(i);

    (i32::abs_diff(coordinates.0 as i32, end.0 as i32)
        + i32::abs_diff(coordinates.1 as i32, end.1 as i32)) as usize
}
