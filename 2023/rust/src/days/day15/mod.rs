use std::collections::HashMap;

use crate::utils::helpers::{extract, extract_named, get_file_lines};

pub fn part1() {
    let input_string = &get_file_lines("./src/days/day15/input.txt")[0];
    let inputs = input_string.split(',');

    let result: u32 = inputs.map(hash).sum();

    println!("Solution for Day 15 Part 1 : {result}");
}

pub fn part2() {
    let input_string = &get_file_lines("./src/days/day15/input.txt")[0];
    let inputs = input_string.split(',');

    let mut cache: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    for i in 0..256 {
        cache.insert(i, vec![]);
    }

    for i in inputs {
        build_hash_map(i, &mut cache);
    }

    let result: u32 = cache
        .into_iter()
        .map(|v| {
            let mut box_total = 0;
            for (i, lens) in v.1.iter().enumerate() {
                box_total += (v.0 + 1) * (i + 1) as u32 * lens.1;
            }
            box_total
        })
        .sum();

    println!("Solution for Day 15 Part 2 : {result}");
}

fn hash(input: &str) -> u32 {
    let mut acc = 0;

    let ascii_values: Vec<u32> = input.chars().map(|c| c as u32).collect();

    for ascii in ascii_values {
        acc += ascii;
        acc *= 17;
        acc %= 256;
    }

    acc
}

fn build_hash_map<'a>(input: &'a str, cache: &mut HashMap<u32, Vec<(&'a str, u32)>>) {
    let label = extract(input, r"[a-zA-Z]+")[0];
    let hash = hash(label);

    let operation = extract_named(input, r"[a-zA-Z]+(?<operation>(=|-))\d*", "operation");

    if operation == "=" {
        let focal_length =
            extract_named(input, r"[a-zA-Z]+(=|-)(?<focal_length>\d+)", "focal_length");
        let lenses_in_box = cache.get_mut(&hash).unwrap();
        if lenses_in_box.iter().any(|x| x.0 == label) {
            let index = lenses_in_box.iter().position(|l| l.0 == label).unwrap();
            lenses_in_box[index] = (label, focal_length.parse().unwrap());
        } else {
            lenses_in_box.push((label, focal_length.parse().unwrap()));
        }
    } else {
        let lenses_in_box = cache.get_mut(&hash).unwrap();
        if lenses_in_box.iter().any(|x| x.0 == label) {
            let index = lenses_in_box.iter().position(|l| l.0 == label).unwrap();
            lenses_in_box.remove(index);
        }
    }
}
