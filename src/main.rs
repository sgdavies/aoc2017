use std::{collections::HashSet, env};
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
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
    if args.contains("7") {
        day07::solve();
    }
    if args.contains("8") {
        day08::solve();
    }
    if args.contains("9") {
        day09::solve();
    }
    if args.contains("10") {
        day10::solve();
    }
    if args.contains("11") {
        day11::solve();
    }
    if args.contains("12") {
        day12::solve();
    }
    if args.contains("13") {
        day13::solve();
    }
    if args.contains("14") {
        day14::solve();
    }
    if args.contains("15") {
        day15::solve();
    }
    if args.contains("16") {
        day16::solve();
    }
    if args.contains("17") {
        day17::solve();
    }
    if args.contains("18") {
        day18::solve();
    }
    if args.contains("19") {
        day19::solve();
    }
}
