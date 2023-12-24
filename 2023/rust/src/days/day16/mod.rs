use std::{
    collections::HashSet,
    fmt::{self, Display},
};

use crate::utils::{helpers::get_input_as_matrix, matrix::Matrix};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };
        write!(f, "{:?}", string)
    }
}

pub fn part1() {
    let mut matrix = get_input_as_matrix("./src/days/day16/input.txt");

    let energized_sum = get_energized_tiles(&mut matrix, (0, 0), Direction::Right);

    println!("Solution for Day 16 Part 1 : {energized_sum}");
}

pub fn part2() {
    let matrix = get_input_as_matrix("./src/days/day16/input.txt");
    let mut max_energized = 0;

    // top edge
    for i in 0..matrix.cols {
        let mut cloned = matrix.clone();

        max_energized = usize::max(
            max_energized,
            get_energized_tiles(&mut cloned, (0, i.try_into().unwrap()), Direction::Down),
        );
    }

    // left edge
    for i in 0..matrix.rows {
        let mut cloned = matrix.clone();

        max_energized = usize::max(
            max_energized,
            get_energized_tiles(&mut cloned, (i.try_into().unwrap(), 0), Direction::Right),
        );
    }

    // right edge
    for i in 0..matrix.cols {
        let mut cloned = matrix.clone();

        max_energized = usize::max(
            max_energized,
            get_energized_tiles(
                &mut cloned,
                (i.try_into().unwrap(), (matrix.cols - 1).try_into().unwrap()),
                Direction::Left,
            ),
        );
    }

    // bottom edge
    for i in 0..matrix.cols {
        let mut cloned = matrix.clone();

        max_energized = usize::max(
            max_energized,
            get_energized_tiles(
                &mut cloned,
                ((matrix.rows - 1).try_into().unwrap(), i.try_into().unwrap()),
                Direction::Up,
            ),
        );
    }

    println!("Solution for Day 16 Part 2 : {max_energized}");
}

fn get_energized_tiles(matrix: &mut Matrix, start: (i32, i32), direction: Direction) -> usize {
    let mut cache: HashSet<((i32, i32), String)> = HashSet::new();
    energize_matrix(matrix, start, direction, &mut cache);

    matrix
        .data
        .iter()
        .filter(|c| ['#', 'a', 'b', 'c', 'd'].contains(c))
        .map(|_| 1)
        .sum()
}

fn energize_matrix(
    matrix: &mut Matrix,
    start: (i32, i32),
    direction: Direction,
    cache: &mut HashSet<((i32, i32), String)>,
) {
    let mut current_index = start;
    let mut current_direction = direction;
    loop {
        if cache
            .get(&(current_index, current_direction.to_string()))
            .is_some()
            || current_index.0 < 0
            || current_index.1 < 0
            || current_index.0 >= matrix.rows.try_into().unwrap()
            || current_index.1 >= matrix.cols.try_into().unwrap()
        {
            return;
        } else if let Some(current_char) = matrix
            .data
            .get(current_index.0 as usize * matrix.cols + current_index.1 as usize)
        {
            cache.insert((current_index, current_direction.to_string()));
            match current_char {
                '.' => {
                    matrix[(current_index.0 as usize, current_index.1 as usize)] = '#';
                    current_index = get_next_index(current_index, &current_direction);
                }
                '#' => {
                    current_index = get_next_index(current_index, &current_direction);
                }
                '\\' | 'a' => {
                    matrix[(current_index.0 as usize, current_index.1 as usize)] = 'a';

                    current_direction = match current_direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    current_index = get_next_index(current_index, &current_direction);
                }
                '/' | 'b' => {
                    matrix[(current_index.0 as usize, current_index.1 as usize)] = 'b';

                    current_direction = match current_direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    current_index = get_next_index(current_index, &current_direction);
                }
                '-' | 'c' => {
                    matrix[(current_index.0 as usize, current_index.1 as usize)] = 'c';

                    match current_direction {
                        Direction::Up | Direction::Down => {
                            let new_start_left = get_next_index(current_index, &Direction::Left);
                            let new_start_right = get_next_index(current_index, &Direction::Right);
                            energize_matrix(matrix, new_start_left, Direction::Left, cache);
                            energize_matrix(matrix, new_start_right, Direction::Right, cache);
                            return;
                        }
                        Direction::Left | Direction::Right => {
                            current_index = get_next_index(current_index, &current_direction);
                        }
                    }
                }
                '|' | 'd' => {
                    matrix[(current_index.0 as usize, current_index.1 as usize)] = 'd';
                    match current_direction {
                        Direction::Left | Direction::Right => {
                            let new_start_up = get_next_index(current_index, &Direction::Up);
                            let new_start_down = get_next_index(current_index, &Direction::Down);
                            energize_matrix(matrix, new_start_up, Direction::Up, cache);
                            energize_matrix(matrix, new_start_down, Direction::Down, cache);
                            return;
                        }
                        Direction::Up | Direction::Down => {
                            current_index = get_next_index(current_index, &current_direction);
                        }
                    }
                }
                _ => panic!(),
            }
        } else {
            return;
        }
    }
}

fn get_next_index(current: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (current.0 - 1, current.1),
        Direction::Down => (current.0 + 1, current.1),
        Direction::Left => (current.0, current.1 - 1),
        Direction::Right => (current.0, current.1 + 1),
    }
}
