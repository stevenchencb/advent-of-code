use std::collections::HashMap;

use crate::utils::helpers::{extract, extract_named};

// Part 1 modeled using a Deterministic Finite Automaton
pub struct Dfa {
    pub start_state: String,
    pub transitions: HashMap<String, HashMap<char, String>>,
    pub end_states: Vec<String>,
    pub steps: u32,
}

impl Dfa {
    pub fn new(lines: &[String]) -> Self {
        let mut transitions: HashMap<String, HashMap<char, String>> = HashMap::new();

        for line in lines.iter() {
            let (state_name, left_transition, right_transition) = get_state_from_line(line);

            let mut transition = HashMap::new();
            transition.insert('L', left_transition);
            transition.insert('R', right_transition);

            transitions.insert(state_name, transition);
        }

        Dfa {
            start_state: String::from("AAA"),
            transitions,
            end_states: vec![String::from("ZZZ")],
            steps: 0,
        }
    }

    pub fn process(&mut self, input: &str) {
        let mut current_state = self.start_state.clone();
        let input_chars: Vec<char> = input.chars().collect();
        let mut idx = 0;

        while !self.end_states.contains(&current_state) {
            let current_input = input_chars[idx];
            current_state = self
                .transition_to_next_state(&current_state, current_input)
                .clone();
            idx = (idx + 1) % input_chars.len();
            self.steps += 1;
        }
    }

    fn transition_to_next_state(&self, current: &String, input: char) -> &String {
        let transitions_of_state = self.transitions.get(current).expect("State does not exist");
        let next_state = transitions_of_state
            .get(&input)
            .expect("Transition does not exist");
        next_state
    }
}

// Part 2 modeled using a Nondeterministic Finite Automaton
pub struct Nfa {
    pub start_states: Vec<String>,
    pub transitions: HashMap<String, HashMap<char, String>>,
    pub end_states: Vec<String>,
    pub all_min_steps: Vec<u64>,
    pub steps_to_end: u64,
}

impl Nfa {
    pub fn new(lines: &[String]) -> Self {
        let mut start_states: Vec<String> = vec![];
        let mut end_states: Vec<String> = vec![];
        let mut transitions: HashMap<String, HashMap<char, String>> = HashMap::new();

        for line in lines.iter() {
            let (state_name, left_transition, right_transition) = get_state_from_line(line);

            if state_name.ends_with('A') {
                start_states.push(state_name.clone());
            } else if state_name.ends_with('Z') {
                end_states.push(state_name.clone());
            }

            let mut transition = HashMap::new();
            transition.insert('L', left_transition);
            transition.insert('R', right_transition);

            transitions.insert(state_name, transition);
        }

        Nfa {
            start_states,
            transitions,
            end_states,
            all_min_steps: vec![],
            steps_to_end: 0,
        }
    }

    // For every starting state, there is a cycle with a fixed length in order to arrive at an end state,
    // meaning that the length of start->end and end->end of a given starting state are the same and won't change.
    // --> Sync cycles: calculate the least common multiple for all cycle lengths of all starting states
    pub fn process(&mut self, input: &str) {
        let input_chars: Vec<char> = input.chars().collect();
        let mut idx = 0;

        for (i, _) in self.start_states.iter().enumerate() {
            let mut current_state = self.start_states[i].clone();
            let mut current_steps = 0;
            while !self.end_states.contains(&current_state) {
                let current_input = input_chars[idx];
                current_state = self
                    .transition_to_next_state(&current_state, current_input)
                    .clone();
                idx = (idx + 1) % input_chars.len();
                current_steps += 1;
            }
            self.all_min_steps.push(current_steps);
        }

        self.steps_to_end = lcm(self.all_min_steps[0], self.all_min_steps[1..].to_vec());
    }

    fn transition_to_next_state(&self, current: &String, input: char) -> &String {
        let transitions_of_state = self.transitions.get(current).expect("State does not exist");
        let next_state = transitions_of_state
            .get(&input)
            .expect("Transition does not exist");
        next_state
    }
}

fn get_state_from_line(line: &str) -> (String, String, String) {
    let state_name =
        extract_named(line, r"(?<state_name>[a-zA-Z0-9]+).*=.*", "state_name").to_string();
    let left_transition = extract_named(
        line,
        r"\((?<left_transition>[a-zA-Z0-9]+),.*\)",
        "left_transition",
    )
    .to_string();
    let right_transition = extract_named(
        line,
        r"\(.*, (?<right_transition>[a-zA-Z0-9]+)\)",
        "right_transition",
    )
    .to_string();

    (state_name, left_transition, right_transition)
}

// calculating the lcm of multiple numbers can be done recursively
fn lcm(first: u64, numbers: Vec<u64>) -> u64 {
    if numbers.len() == 1 {
        return f64::floor((first * numbers[0] / gcd(first, numbers[0])) as f64) as u64;
    }

    lcm(lcm(first, numbers[0..1].to_vec()), numbers[1..].to_vec())
}

// greatest common denominator with Euclid's algorithm
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }

    a
}
