#![allow(dead_code)]
#![allow(unused_imports)]
mod days;
mod utils;

use std::time::Instant;

use days::day1;
use days::day10;
use days::day11;
use days::day6;
use days::day8;
use days::day9;

fn main() {
    run_part1();
    run_part2();
}

fn run_part1() {
    let start = Instant::now();

    day11::part1();

    let elapsed = start.elapsed();

    println!("Time elapsed: {:?}\n", elapsed);
}

fn run_part2() {
    let start = Instant::now();

    day11::part2();

    let elapsed = start.elapsed();

    println!("Time elapsed: {:?}\n", elapsed);
}
