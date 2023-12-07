use std::time::Instant;

mod days;
mod utils;

fn main() {
    run_part1();
    run_part2();
}

fn run_part1() {
    let start = Instant::now();

    days::day1::part1();

    let elapsed = start.elapsed();

    println!("Time elapsed: {:?}\n", elapsed);
}

fn run_part2() {
    let start = Instant::now();

    days::day1::part2();

    let elapsed = start.elapsed();

    println!("Time elapsed: {:?}\n", elapsed);
}
