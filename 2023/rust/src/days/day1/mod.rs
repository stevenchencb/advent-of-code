use std::fs;

use crate::utils::get_file_lines;

pub fn part1() {
    let lines = get_file_lines("./src/days/day1/input.txt");

    let mut calibration_sum: u32 = 0;

    for line in lines {
        let numbers: Vec<u32> = line
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        calibration_sum += if !numbers.is_empty() {
            numbers[0] * 10 + numbers[numbers.len() - 1]
        } else {
            0
        };
    }

    println!("Solution to Day 1 Part 1: {calibration_sum}");
}

pub fn part2() {
    let mut input = fs::read_to_string("./src/days/day1/input.txt").unwrap();
    let mut replaced_input: Vec<char> = input.chars().collect();
    const NUMBER_WORDS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, _) in NUMBER_WORDS.iter().enumerate() {
        let mut digit_word_index = input.find(NUMBER_WORDS[i]);
        while digit_word_index.is_some() {
            replaced_input[digit_word_index.unwrap() + 1] = char::from_digit(i as u32, 10).unwrap();
            input = replaced_input.clone().into_iter().collect();
            digit_word_index = input.find(NUMBER_WORDS[i])
        }
    }

    let lines: Vec<&str> = input.lines().collect();

    let mut calibration_sum: u32 = 0;

    for line in lines {
        let numbers: Vec<u32> = line
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        calibration_sum += if !numbers.is_empty() {
            numbers[0] * 10 + numbers[numbers.len() - 1]
        } else {
            0
        };
    }

    println!("Solution to Day 1 Part 2: {calibration_sum}");
}
