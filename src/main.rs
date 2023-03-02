use std::{collections::HashSet, env};
mod day03;
mod day04;
mod day05;
mod day06;
mod utils;

fn main() {
    let args: HashSet<String> = env::args().collect();
    println!("Hello, world!");
    if args.contains("3") {
        day03::solve();
    }
    if args.contains("4") {
        day04::solve();
    }
    if args.contains("5") {
        day05::solve();
    }
    if args.contains("6") {
        day06::solve();
    }
}
