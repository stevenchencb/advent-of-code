use crate::utils::get_file_lines;

enum FieldSymbol {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
    Flooded,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl FieldSymbol {
    fn to_char(&self) -> char {
        match self {
            FieldSymbol::NS => '|',
            FieldSymbol::EW => '-',
            FieldSymbol::NE => 'L',
            FieldSymbol::NW => 'J',
            FieldSymbol::SW => '7',
            FieldSymbol::SE => 'F',
            FieldSymbol::Ground => '.',
            FieldSymbol::Start => 'S',
            FieldSymbol::Flooded => 'X',
        }
    }

    fn as_enum(c: &char) -> FieldSymbol {
        match c {
            '|' => FieldSymbol::NS,
            '-' => FieldSymbol::EW,
            'L' => FieldSymbol::NE,
            'J' => FieldSymbol::NW,
            '7' => FieldSymbol::SW,
            'F' => FieldSymbol::SE,
            'S' => FieldSymbol::Start,
            'X' => FieldSymbol::Flooded,
            _ => FieldSymbol::Ground,
        }
    }
}

pub fn part1() {
    let matrix: Vec<Vec<char>> = get_file_lines("./src/days/day10/input.txt")
        .into_iter()
        .map(|l| l.chars().collect())
        .collect();

    let farthest_distance = traverse_pipes(&matrix).0 / 2;

    println!("Solution to Day 10 Part 1 : {farthest_distance}")
}

pub fn part2() {
    let matrix: Vec<Vec<char>> = get_file_lines("./src/days/day10/input.txt")
        .into_iter()
        .map(|l| l.chars().collect())
        .collect();

    let cleaned_matrix = remove_impossible_pipes(&matrix);

    // view pipes as boundaries of a polygon and use shoelace formula + Pick's theorem to calculate number of inside tiles
    let boundaries = traverse_pipes(&cleaned_matrix).1;
    let area = shoelace(&boundaries);
    let inside_tiles = picks_theorem(area, boundaries.len() as i32);

    println!("Solution to Day 10 Part 2 : {inside_tiles}")
}

fn remove_impossible_pipes(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut cleaned = matrix.to_owned();

    for (i, row) in matrix.iter().enumerate() {
        let is_first_row = i == 0;
        let is_last_row = i == matrix.len() - 1;
        for (j, symbol) in row.clone().iter().enumerate() {
            let is_first_symbol = j == 0;
            let is_last_symbol = j == matrix[i].len() - 1;

            let row_above: Vec<char> = if is_first_row {
                vec![]
            } else {
                cleaned[i - 1].to_vec()
            };

            let row_below: Vec<char> = if is_last_row {
                vec![]
            } else {
                cleaned[i + 1].to_vec()
            };

            let north = FieldSymbol::as_enum(row_above.get(j).unwrap_or(&'.'));
            let south = FieldSymbol::as_enum(row_below.get(j).unwrap_or(&'.'));

            let east = if is_last_symbol {
                FieldSymbol::as_enum(&'.')
            } else {
                FieldSymbol::as_enum(&cleaned[i][j + 1])
            };

            let west = if is_first_symbol {
                FieldSymbol::as_enum(&'.')
            } else {
                FieldSymbol::as_enum(&cleaned[i][j - 1])
            };

            if !is_possible_pipe(FieldSymbol::as_enum(symbol), north, south, east, west) {
                cleaned[i][j] = '.';
            }
        }
    }

    cleaned
}

fn is_possible_pipe(
    s: FieldSymbol,
    north: FieldSymbol,
    south: FieldSymbol,
    east: FieldSymbol,
    west: FieldSymbol,
) -> bool {
    let north_connecting = ['|', '7', 'F', 'S'];
    let south_connecting = ['|', 'L', 'J', 'S'];
    let west_connecting = ['-', 'L', 'F', 'S'];
    let east_connecting = ['-', 'J', '7', 'S'];

    match s {
        FieldSymbol::NS => {
            north_connecting.contains(&north.to_char())
                && south_connecting.contains(&south.to_char())
        }
        FieldSymbol::EW => {
            east_connecting.contains(&east.to_char()) && west_connecting.contains(&west.to_char())
        }
        FieldSymbol::NE => {
            north_connecting.contains(&north.to_char()) && east_connecting.contains(&east.to_char())
        }
        FieldSymbol::NW => {
            north_connecting.contains(&north.to_char()) && west_connecting.contains(&west.to_char())
        }
        FieldSymbol::SW => {
            south_connecting.contains(&south.to_char()) && west_connecting.contains(&west.to_char())
        }
        FieldSymbol::SE => {
            south_connecting.contains(&south.to_char()) && east_connecting.contains(&east.to_char())
        }
        FieldSymbol::Start => true,
        _ => false,
    }
}

fn traverse_pipes(matrix: &[Vec<char>]) -> (i32, Vec<Vec<usize>>) {
    let mut vertices: Vec<Vec<usize>> = vec![];

    let row_length = matrix.first().unwrap().len();
    let flattened_start = matrix.iter().flatten().position(|c| *c == 'S').unwrap();
    let start_row = flattened_start / row_length;
    let start_position = flattened_start % row_length;

    vertices.push(vec![start_row, start_position]);

    let (mut current_row, mut current_position, mut current_char, mut coming_from) =
        get_initial_step(matrix, start_row, start_position);

    let mut pipe_length: i32 = 1;

    while current_char != 'S' {
        vertices.push(vec![current_row, current_position]);
        (current_row, current_position, coming_from) =
            get_next_coordinates(current_char, current_row, current_position, &coming_from);
        current_char = matrix[current_row][current_position];
        pipe_length += 1;
    }

    (pipe_length, vertices)
}

fn get_initial_step(
    matrix: &[Vec<char>],
    start_row: usize,
    start_position: usize,
) -> (usize, usize, char, Direction) {
    if start_row != 0 && ['|', '7', 'F'].contains(&matrix[start_row - 1][start_position]) {
        (
            start_row - 1,
            start_position,
            matrix[start_row - 1][start_position],
            Direction::South,
        )
    } else if start_position != matrix.first().unwrap().len() - 1
        && ['-', 'J', '7'].contains(&matrix[start_row][start_position + 1])
    {
        (
            start_row,
            start_position + 1,
            matrix[start_row][start_position + 1],
            Direction::West,
        )
    } else if start_row != matrix.len() - 1
        && ['|', 'L', 'J'].contains(&matrix[start_row + 1][start_position])
    {
        (
            start_row + 1,
            start_position,
            matrix[start_row + 1][start_position],
            Direction::North,
        )
    } else {
        (
            start_row,
            start_position - 1,
            matrix[start_row][start_position - 1],
            Direction::East,
        )
    }
}

fn get_next_coordinates(
    current_char: char,
    current_row: usize,
    current_position: usize,
    coming_from: &Direction,
) -> (usize, usize, Direction) {
    match current_char {
        '|' => match coming_from {
            Direction::North => (current_row + 1, current_position, Direction::North),
            Direction::South => (current_row - 1, current_position, Direction::South),
            _ => panic!("Impossible path"),
        },
        '-' => match coming_from {
            Direction::West => (current_row, current_position + 1, Direction::West),
            Direction::East => (current_row, current_position - 1, Direction::East),
            _ => panic!("Impossible path"),
        },
        'L' => match coming_from {
            Direction::North => (current_row, current_position + 1, Direction::West),
            Direction::East => (current_row - 1, current_position, Direction::South),
            _ => panic!("Impossible path"),
        },
        'J' => match coming_from {
            Direction::North => (current_row, current_position - 1, Direction::East),
            Direction::West => (current_row - 1, current_position, Direction::South),
            _ => panic!("Impossible path"),
        },
        '7' => match coming_from {
            Direction::South => (current_row, current_position - 1, Direction::East),
            Direction::West => (current_row + 1, current_position, Direction::North),
            _ => panic!("Impossible path"),
        },
        'F' => match coming_from {
            Direction::South => (current_row, current_position + 1, Direction::West),
            Direction::East => (current_row + 1, current_position, Direction::North),
            _ => panic!("Impossible path"),
        },
        _ => panic!("Impossible path"),
    }
}

fn shoelace(vertices: &[Vec<usize>]) -> f32 {
    let mut result: f32 = 0.0;
    for i in 0..vertices.len() {
        result += (vertices[i % vertices.len()][0] as f32
            * vertices[(i + 1) % vertices.len()][1] as f32)
            - (vertices[(i + 1) % vertices.len()][0] as f32
                * vertices[i % vertices.len()][1] as f32)
    }

    result / 2.0
}

fn picks_theorem(area: f32, num_of_boundaries: i32) -> f32 {
    area.abs() - (num_of_boundaries as f32 / 2.0) + 1.0
}
