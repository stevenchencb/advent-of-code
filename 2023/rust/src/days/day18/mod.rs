use std::collections::HashSet;

use crate::utils::helpers::{extract_named, get_file_lines, picks_theorem, shoelace};

pub fn part1() {
    let plan = get_file_lines("./src/days/day18/input.txt");

    let boundaries = get_trench_boundaries(&plan);
    let area = shoelace(&boundaries) / 2.0;
    let inside_tiles = picks_theorem(area, boundaries.len());

    println!(
        "Solution to Day 18 Part 1 : {}",
        boundaries.len() + inside_tiles
    );
}

pub fn part2() {
    let plan = get_file_lines("./src/days/day18/input.txt");

    let boundaries = get_trench_boundaries_from_colors(&plan);
    let area = shoelace(&boundaries) / 2.0;
    let inside_tiles = picks_theorem(area, boundaries.len());

    println!(
        "Solution to Day 18 Part 2 : {}",
        boundaries.len() + inside_tiles
    );
}

fn get_trench_boundaries(plan: &[String]) -> Vec<(i32, i32)> {
    let digging_plan = plan
        .iter()
        .map(|s| {
            let regex_string = r"(?<direction>[UDLR]) (?<num_tiles>\d+) .*";
            let direction = extract_named(s, regex_string, "direction");
            let num_tiles = extract_named(s, regex_string, "num_tiles").parse().unwrap();
            (direction, num_tiles)
        })
        .collect::<Vec<(String, usize)>>();

    let mut boundaries: Vec<(i32, i32)> = vec![];

    let mut current_position = (0, 0);

    for d in digging_plan {
        for _ in 0..d.1 {
            match d.0.as_str() {
                "U" => current_position.1 -= 1,
                "D" => current_position.1 += 1,
                "L" => current_position.0 -= 1,
                "R" => current_position.0 += 1,
                _ => unreachable!(),
            }
            boundaries.push(current_position);
        }
    }

    boundaries
}

fn get_trench_boundaries_from_colors(plan: &[String]) -> Vec<(i32, i32)> {
    let digging_plan = plan
        .iter()
        .map(|s| {
            let regex_string = r"[UDLR] \d+ \(#(?<color>[0-9a-f]{6})\)";
            let color = extract_named(s, regex_string, "color");
            let direction = match color.chars().last().unwrap() {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => unreachable!(),
            };
            let num_tiles = usize::from_str_radix(
                color
                    .chars()
                    .take(color.len() - 1)
                    .collect::<String>()
                    .as_str(),
                16,
            )
            .unwrap();
            (direction, num_tiles)
        })
        .collect::<Vec<(&str, usize)>>();

    let mut boundaries: Vec<(i32, i32)> = vec![];

    let mut current_position = (0, 0);

    for d in digging_plan {
        for _ in 0..d.1 {
            match d.0 {
                "U" => current_position.1 -= 1,
                "D" => current_position.1 += 1,
                "L" => current_position.0 -= 1,
                "R" => current_position.0 += 1,
                _ => unreachable!(),
            }
            boundaries.push(current_position);
        }
    }

    boundaries
}
