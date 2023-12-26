use std::{cmp::Reverse, collections::HashSet};

use priority_queue::PriorityQueue;

use crate::utils::{
    direction::Direction,
    helpers::get_input_as_matrix,
    matrix::{self, Matrix},
    node::Node,
};

pub fn part1() {
    let matrix = get_input_as_matrix("./src/days/day17/input.txt");

    let min_heat_loss = a_star(&matrix, 0, (matrix.rows - 1, matrix.cols - 1), 0, 3);

    println!("Solution to Day 17 Part 1 : {min_heat_loss}");
}

pub fn part2() {
    let matrix = get_input_as_matrix("./src/days/day17/input.txt");

    let min_heat_loss = a_star(&matrix, 0, (matrix.rows - 1, matrix.cols - 1), 4, 10);

    println!("Solution to Day 17 Part 2 : {min_heat_loss}");
}

fn a_star(
    matrix: &Matrix,
    start: usize,
    end: (usize, usize),
    min_moves: usize,
    max_moves: usize,
) -> usize {
    let mut pq: PriorityQueue<(usize, usize, Direction, usize), Reverse<usize>> =
        PriorityQueue::new();
    initialize_pq(&mut pq, start);

    let mut visited: HashSet<(usize, Direction, usize)> = HashSet::new();

    let mut current: (usize, usize, Direction, usize);

    while !pq.is_empty() {
        current = pq.pop().unwrap().0;
        if current.1 == matrix.data.len() - 1 && current.3 >= min_moves {
            return current.0;
        }

        for neighbor in get_neighbors(matrix, current, min_moves, max_moves) {
            let priority = neighbor.0 + manhattan(matrix, neighbor.1, end);

            if !visited.contains(&(neighbor.1, neighbor.2, neighbor.3)) {
                pq.push(neighbor, Reverse(priority));
            }
        }

        visited.insert((current.1, current.2, current.3));
    }

    panic!("No path found");
}

fn initialize_pq(
    pq: &mut PriorityQueue<(usize, usize, Direction, usize), Reverse<usize>>,
    start: usize,
) {
    pq.push((0, start, Direction::Left, 0), Reverse(0));
    pq.push((0, start, Direction::Down, 0), Reverse(0));
    pq.push((0, start, Direction::Right, 0), Reverse(0));
    pq.push((0, start, Direction::Up, 0), Reverse(0));
}

fn get_neighbors(
    matrix: &Matrix,
    node: (usize, usize, Direction, usize),
    min_moves: usize,
    max_moves: usize,
) -> Vec<(usize, usize, Direction, usize)> {
    let mut neighbors: Vec<(usize, usize, Direction, usize)> = vec![];

    match node.2 {
        Direction::Up => {
            // left boundary check
            if node.1 % matrix.cols != 0 && node.3 >= min_moves {
                let heat_loss = node.0 + matrix.data[node.1 - 1].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 - 1, Direction::Left, 1));
            }
            // upper boundary and straight line check
            if matrix.as_coordinates(node.1).0 != 0 && node.3 < max_moves {
                let heat_loss =
                    node.0 + matrix.data[node.1 - matrix.cols].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 - matrix.cols, Direction::Up, node.3 + 1));
            }
            // right boundary check
            if node.1 % matrix.cols != matrix.cols - 1 && node.3 >= min_moves {
                let heat_loss = node.0 + matrix.data[node.1 + 1].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 + 1, Direction::Right, 1));
            }
        }
        Direction::Down => {
            // left boundary check
            if node.1 % matrix.cols != 0 && node.3 >= min_moves {
                let heat_loss = node.0 + matrix.data[node.1 - 1].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 - 1, Direction::Left, 1));
            }
            // lower boundary and straight line check
            if matrix.as_coordinates(node.1).0 != matrix.rows - 1 && node.3 < max_moves {
                let heat_loss =
                    node.0 + matrix.data[node.1 + matrix.cols].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 + matrix.cols, Direction::Down, node.3 + 1));
            }
            // right boundary check
            if node.1 % matrix.cols != matrix.cols - 1 && node.3 >= min_moves {
                let heat_loss = node.0 + matrix.data[node.1 + 1].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 + 1, Direction::Right, 1));
            }
        }
        Direction::Left => {
            // left boundary and straight line check
            if node.1 % matrix.cols != 0 && node.3 < max_moves {
                let heat_loss = node.0 + matrix.data[node.1 - 1].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 - 1, Direction::Left, node.3 + 1));
            }
            // lower boundary check
            if matrix.as_coordinates(node.1).0 != matrix.rows - 1 && node.3 >= min_moves {
                let heat_loss =
                    node.0 + matrix.data[node.1 + matrix.cols].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 + matrix.cols, Direction::Down, 1));
            }
            // upper boundary check
            if matrix.as_coordinates(node.1).0 != 0 && node.3 >= min_moves {
                let heat_loss =
                    node.0 + matrix.data[node.1 - matrix.cols].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 - matrix.cols, Direction::Up, 1));
            }
        }
        Direction::Right => {
            // upper boundary check
            if matrix.as_coordinates(node.1).0 != 0 && node.3 >= min_moves {
                let heat_loss =
                    node.0 + matrix.data[node.1 - matrix.cols].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 - matrix.cols, Direction::Up, 1));
            }
            // right boundary and straight line check
            if node.1 % matrix.cols != matrix.cols - 1 && node.3 < max_moves {
                let heat_loss = node.0 + matrix.data[node.1 + 1].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 + 1, Direction::Right, node.3 + 1));
            }
            // lower boundary check
            if matrix.as_coordinates(node.1).0 != matrix.rows - 1 && node.3 >= min_moves {
                let heat_loss =
                    node.0 + matrix.data[node.1 + matrix.cols].to_digit(10).unwrap() as usize;
                neighbors.push((heat_loss, node.1 + matrix.cols, Direction::Down, 1));
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
