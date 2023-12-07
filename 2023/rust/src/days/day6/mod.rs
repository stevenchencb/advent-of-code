use crate::utils::{self, get_numbers};

pub fn part1() {
    let lines = utils::get_file_lines("./src/days/day6/input.txt");

    let times = get_numbers(&lines[0]);
    let distances = get_numbers(&lines[1]);

    let mut ways_to_win = 1.0;

    for (i, el) in times.iter().enumerate() {
        ways_to_win *= mitternacht(el, &distances[i]);
    }

    println!("Solution for Part 1: {}", ways_to_win);
}

pub fn part2() {
    let lines = utils::get_file_lines("./src/days/day6/input.txt");

    let time_string = lines[0].replace(" ", "");

    let distance_string = lines[1].replace(" ", "");

    let time = get_numbers(&time_string)[0];
    let distance = get_numbers(&distance_string)[0];

    println!("Solution for Part 2: {}", mitternacht(&time, &distance))
}

fn mitternacht(time: &i64, distance: &i64) -> f64 {
    let under_square_root = (time.pow(2) - 4 * distance) as f64;
    let square_root = f64::sqrt(under_square_root);

    let x1: f64 = (-time as f64 + square_root) / -2.0;
    let x2: f64 = (-time as f64 - square_root) / -2.0;

    return f64::ceil(x2 - 1.0) - f64::floor(x1 + 1.0) + 1.0;
}
