use std::collections::HashSet;

use crate::utils::{extract, get_file_lines};

pub fn part1() {
    let histories: Vec<String> = get_file_lines("./src/days/day9/input.txt");
    let mut extrapolated_sum = 0;

    for history in histories {
        let history_numbers = map_to_history_numbers(&history);
        let sequences = get_sequences(&history_numbers);
        extrapolated_sum += get_extrapolated_value_forward(&sequences);
    }

    println!("{extrapolated_sum}");
}

pub fn part2() {
    let histories: Vec<String> = get_file_lines("./src/days/day9/input.txt");
    let mut extrapolated_sum = 0;

    for history in histories {
        let history_numbers = map_to_history_numbers(&history);
        let sequences = get_sequences(&history_numbers);
        extrapolated_sum += get_extrapolated_value_backward(&sequences);
    }

    println!("{extrapolated_sum}");
}

fn map_to_history_numbers(history: &str) -> Vec<i32> {
    extract(history, r"-?\d+")
        .into_iter()
        .map(|n| n.parse::<i32>().expect("Not a number"))
        .collect::<Vec<i32>>()
}

fn get_sequences(history: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences: Vec<Vec<i32>> = vec![history.to_vec()];

    let mut next_sequence = get_next_sequence(history);

    while has_differences(&next_sequence) {
        sequences.push(next_sequence.clone());
        next_sequence = get_next_sequence(&next_sequence);
    }

    sequences.push(next_sequence);

    sequences
}

fn get_next_sequence(current: &[i32]) -> Vec<i32> {
    let mut next_sequence = vec![];

    for i in 0..current.len() - 1 {
        next_sequence.push(current[i + 1] - current[i]);
    }

    next_sequence
}

fn has_differences(sequence: &[i32]) -> bool {
    let mut s = sequence.to_owned();
    s.sort_unstable();
    s.dedup();
    s.len() > 1
}

fn get_extrapolated_value_forward(sequences: &[Vec<i32>]) -> i32 {
    let mut current_diff: i32 = 0;

    for sequence in sequences.iter().rev().take(sequences.len() - 1) {
        current_diff += sequence.last().expect("No number found in sequence");
    }

    sequences
        .first()
        .expect("No sequence found")
        .last()
        .expect("No number in sequence found")
        + current_diff
}

fn get_extrapolated_value_backward(sequences: &[Vec<i32>]) -> i32 {
    let mut current_diff: i32 = 0;

    for sequence in sequences.iter().rev().take(sequences.len() - 1) {
        current_diff = sequence.first().expect("No number found in sequence") - current_diff;
    }

    sequences
        .first()
        .expect("No sequence found")
        .first()
        .expect("No number in sequence found")
        - current_diff
}
