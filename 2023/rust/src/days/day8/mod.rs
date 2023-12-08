use crate::utils::get_file_lines;

use self::automatons::Dfa;
use self::automatons::Nfa;

mod automatons;

pub fn part1() {
    let lines = get_file_lines("./src/days/day8/input.txt");
    let inputs: Vec<String> = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .cloned()
        .collect();
    let automaton_input = &inputs[0];
    let automaton_definition = &inputs[1..];

    let mut dfa = Dfa::new(automaton_definition);
    dfa.process(automaton_input);

    println!("Solution for Day 8 Part 1: {}", dfa.steps);
}

pub fn part2() {
    let lines = get_file_lines("./src/days/day8/input.txt");
    let inputs: Vec<String> = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .cloned()
        .collect();
    let automaton_input = &inputs[0];
    let automaton_definition = &inputs[1..];

    let mut nfa = Nfa::new(automaton_definition);

    nfa.process(automaton_input);

    println!("Solution for Day 8 Part 2: {}", nfa.steps_to_end);
}
