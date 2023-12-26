use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::helpers::{extract_named, get_file_blocks};

#[derive(Debug)]
struct Rule {
    pub category: char,
    pub comparator: char,
    pub value: usize,
    pub next_workflow: String,
}

#[derive(Debug)]
struct Item {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

pub fn part1() {
    let input = get_file_blocks("./src/days/day19/input.txt");
    let workflows = process_workflows(&input[0]);
    let items = process_items(&input[1]);

    let mut total = 0;

    for item in items {
        if accept(&workflows, &item, "in") {
            total += item.x + item.m + item.a + item.s
        }
    }

    println!("Solution to Day 19 Part 1 : {total}");
}

pub fn part2() {
    let input = get_file_blocks("./src/days/day19/input.txt");
    let workflows = process_workflows(&input[0]);

    let mut ranges: HashMap<char, (usize, usize)> = HashMap::from_iter([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);
    let total = count_possibilities(&workflows, &mut ranges, "in".to_string());

    println!("Solution to Day 19 Part 2 : {total}");
}

fn process_items(items: &[String]) -> Vec<Item> {
    let regex = r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}";

    items
        .iter()
        .map(|i| {
            let x: usize = extract_named(i.as_str(), regex, "x").parse().unwrap();
            let m: usize = extract_named(i.as_str(), regex, "m").parse().unwrap();
            let a: usize = extract_named(i.as_str(), regex, "a").parse().unwrap();
            let s: usize = extract_named(i.as_str(), regex, "s").parse().unwrap();

            Item { x, m, a, s }
        })
        .collect()
}

fn process_workflows(workflows: &[String]) -> HashMap<String, (Vec<Rule>, String)> {
    let mut hash_map: HashMap<String, (Vec<Rule>, String)> = HashMap::new();

    for w in workflows {
        let regex = r"(?<name>\w+)\{(?<rules_and_fallback>.*)\}";
        let name = extract_named(w.as_str(), regex, "name");
        let (rules, fallback) =
            process_rules_and_fallback(&extract_named(w.as_str(), regex, "rules_and_fallback"));
        hash_map.insert(name, (rules, fallback));
    }

    hash_map
}

fn process_rules_and_fallback(rules_and_fallback: &str) -> (Vec<Rule>, String) {
    let split: Vec<String> = rules_and_fallback.split(',').map(String::from).collect();
    let rules: Vec<Rule> = split
        .iter()
        .take(split.len() - 1)
        .map(|r| {
            let regex =
                r"(?<category>[xmas])(?<comparator>[<>])(?<value>\d+):(?<next_workflow>\w+)";
            let category = extract_named(r, regex, "category").chars().next().unwrap();
            let comparator = extract_named(r, regex, "comparator")
                .chars()
                .next()
                .unwrap();
            let value: usize = extract_named(r, regex, "value").parse().unwrap();
            let next_workflow = extract_named(r, regex, "next_workflow");

            Rule {
                category,
                comparator,
                value,
                next_workflow,
            }
        })
        .collect();
    let fallback = split.last().unwrap();

    (rules, fallback.to_string())
}

fn accept(
    workflows: &HashMap<String, (Vec<Rule>, String)>,
    item: &Item,
    workflow_name: &str,
) -> bool {
    if workflow_name == "A" {
        return true;
    }
    if workflow_name == "R" {
        return false;
    }

    let rules = &workflows.get(workflow_name).unwrap().0;
    let fallback = &workflows.get(workflow_name).unwrap().1;

    for rule in rules {
        if compare(item, rule) {
            return accept(workflows, item, rule.next_workflow.as_str());
        }
    }

    accept(workflows, item, fallback.as_str())
}

fn compare(item: &Item, rule: &Rule) -> bool {
    let item_map: HashMap<char, usize> =
        HashMap::from_iter([('x', item.x), ('m', item.m), ('a', item.a), ('s', item.s)]);

    match rule.comparator {
        '<' => item_map.get(&rule.category).unwrap() < &rule.value,
        '>' => item_map.get(&rule.category).unwrap() > &rule.value,
        _ => unreachable!(),
    }
}

fn count_possibilities(
    workflows: &HashMap<String, (Vec<Rule>, String)>,
    ranges: &mut HashMap<char, (usize, usize)>,
    current_workflow: String,
) -> usize {
    if current_workflow == "A" {
        let x_range = ranges.get(&'x').unwrap();
        let m_range = ranges.get(&'m').unwrap();
        let a_range = ranges.get(&'a').unwrap();
        let s_range = ranges.get(&'s').unwrap();

        return (x_range.1 - x_range.0 + 1)
            * (m_range.1 - m_range.0 + 1)
            * (a_range.1 - a_range.0 + 1)
            * (s_range.1 - s_range.0 + 1);
    }

    if current_workflow == "R" {
        return 0;
    }

    let rules = &workflows.get(&current_workflow).unwrap().0;
    let fallback = &workflows.get(&current_workflow).unwrap().1;

    let mut possibilities = 0;
    let mut use_fallback = true;

    for rule in rules {
        let range = ranges.get(&rule.category).unwrap();

        // split range in two: one half fulfills the rule and gets processed in the next workflow,
        // the other half doesn't and gets deferred to the next rule
        let true_half: (usize, usize);
        let false_half: (usize, usize);

        if rule.comparator == '<' {
            true_half = (range.0, usize::min(range.1, rule.value - 1));
            false_half = (usize::max(range.0, rule.value), range.1);
        } else {
            true_half = (usize::max(range.0, rule.value + 1), range.1);
            false_half = (range.0, usize::min(range.1, rule.value));
        }

        if true_half.0 <= true_half.1 {
            let mut ranges_clone = ranges.clone();
            ranges_clone.insert(rule.category, true_half);
            possibilities +=
                count_possibilities(workflows, &mut ranges_clone, rule.next_workflow.clone())
        }

        if false_half.0 <= false_half.1 {
            ranges.insert(rule.category, false_half);
        } else {
            // all possibilities of the category are in the true half and are deferred to the next workflow
            // and are handled in recursive call (line 193)
            // do not use fallback in this case
            use_fallback = false;
            break;
        }
    }

    if use_fallback {
        possibilities += count_possibilities(workflows, ranges, fallback.clone());
    }

    possibilities
}
