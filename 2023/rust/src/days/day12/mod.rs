use crate::utils::{extract, get_file_lines};
use std::{collections::HashMap, iter::once};

// part 1 initially brute-forced
pub fn part1() {
    let lines = get_file_lines("./src/days/day12/input.txt");

    let inputs: Vec<(String, Vec<usize>)> = lines
        .iter()
        .map(|l| {
            let mut inputs = l.split_whitespace();
            let springs = inputs.next().unwrap().to_string();
            let groups: Vec<usize> = inputs
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (springs, groups)
        })
        .collect();

    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    let sum: usize = inputs
        .iter()
        .map(|i| validate_input(&i.0, &i.1, &mut cache))
        .sum();

    println!("Solution for Day 12 Part 1 : {sum}");
}

// part 2 solved using memoization/caching
// courtesy of https://github.com/wilkotom/Aoc2023/blob/main/day12/src/main.rs
pub fn part2() {
    let lines = get_file_lines("./src/days/day12/input.txt");

    let inputs: Vec<(String, Vec<usize>)> = lines
        .iter()
        .map(|l| {
            let mut inputs = l.split_whitespace();
            let springs = inputs.next().unwrap().to_string();
            let groups: Vec<usize> = inputs
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (springs, groups)
        })
        .collect();

    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    let mut sum: usize = 0;

    for (springs, groups) in inputs {
        let new_springs: String = springs
            .chars()
            .chain(once('?'))
            .cycle()
            .take(5 * (springs.len() + 1) - 1)
            .collect();
        let new_groups: Vec<usize> = groups
            .iter()
            .cycle()
            .take(groups.len() * 5)
            .copied()
            .collect();

        sum += validate_input(&new_springs, &new_groups, &mut cache);
    }

    println!("Solution for Day 12 Part 2 : {sum}");
}

// courtesy of https://github.com/wilkotom/Aoc2023/blob/main/day12/src/main.rs
// enhanced with comments to further explain why this solution is correct
fn validate_input(
    springs: &str,
    groups: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    let cache_key = (springs.to_string(), groups.to_vec());
    let result = if let Some(cache_hit) = cache.get(&cache_key) {
        // case 0:
        // cache hit
        *cache_hit
    } else if springs.starts_with('.') {
        // case 1:
        // nothing to match current group with, skip current spring
        validate_input(springs.strip_prefix('.').unwrap(), groups, cache)
    } else if springs.starts_with('?') {
        // case 2:
        // expand unknown spring to be either broken (#) or functional (.) (in which case we can skip the next spring since we would land in case 1 and skip either way)
        let unknown_as_broken_spring = springs.replacen('?', "#", 1);
        validate_input(&unknown_as_broken_spring, groups, cache)
            + validate_input(springs.strip_prefix('?').unwrap(), groups, cache)
    } else if springs.is_empty() {
        // case 3:
        // valid arrangement if no springs are left and no groups are to be matched
        groups.is_empty() as usize
    } else if groups.is_empty() && springs.contains('#')
        || springs.len() < groups[0]
        || springs[..groups[0]].contains('.')
    {
        // case 4:
        // arrangement is invalid for one of the following cases:
        // 4.1) no more groups left to match, but an additional spring is found
        // 4.2) number of springs is less than the current group to be matched
        // 4.3) springs to be matched with current group cannot be matched due to insufficient number of broken springs (e.g. ##. 3)
        0
    } else if springs.len() == groups[0] {
        // case 5:
        // valid arrangement if springs left has the same length as the current group to be matched and there is only one group left
        // safe to assume because a) springs start with # (covered by cases 1 and 2) and b) there is a sufficient number of broken springs (covered by case 4.3)
        (groups.len() == 1) as usize
    } else if springs.chars().nth(groups[0]) == Some('#') {
        // case 6:
        // invalid arrangement if there are more broken springs left than the current group to be matched
        0
    } else {
        // case 7:
        // successfully matched current group, continue with next group and skipping matched springs
        validate_input(&springs[groups[0] + 1..], &groups[1..], cache)
    };

    cache.insert(cache_key, result);

    result
}
