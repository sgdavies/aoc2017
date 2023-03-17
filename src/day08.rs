use std::collections::{HashMap, VecDeque};
use std::fs;

pub fn solve() {
    let input = fs::read_to_string(&"08.txt").expect("Can't read 08.txt");
    let mut registers = HashMap::<&str, i32>::new();

    let mut max_ever = 0;

    for line in input.trim_end().split('\n') {
        let mut words: VecDeque<&str> = line.trim_end().split_whitespace().collect();
        let reg = words.pop_front().unwrap();
        let op = words.pop_front().unwrap(); // inc or dec
        let amount = words.pop_front().unwrap().parse::<i32>().unwrap();
        words.pop_front(); // if
        let treg = words.pop_front().unwrap();
        let cmp = words.pop_front().unwrap();
        let camount = words.pop_front().unwrap().parse::<i32>().unwrap();

        let tregval = registers.get(treg).unwrap_or(&0);
        let iff = match cmp {
            "<" => tregval < &camount,
            "<=" => tregval <= &camount,
            ">" => tregval > &camount,
            ">=" => tregval >= &camount,
            "==" => tregval == &camount,
            "!=" => tregval != &camount,
            bad => {
                println!("Bad comparator: {}", bad);
                panic!()
            }
        };
        if iff {
            let newval = registers.get(reg).unwrap_or(&0)
                + match op {
                    "inc" => amount,
                    "dec" => -amount,
                    bad => {
                        println!("Bad op: {}", bad);
                        panic!()
                    }
                };
            registers.insert(reg, newval);
            if newval > max_ever {
                max_ever = newval;
            }
        }
    }

    println!("08 part one {}", registers.values().max().unwrap());
    println!("08 part two {}", max_ever);
}
