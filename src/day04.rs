use std::{collections::HashSet, fs};

pub fn solve() {
    let input = fs::read_to_string(&"04.txt").expect("Can't read 04.txt");
    let one = input
        .clone()
        .trim_end()
        .split('\n')
        .filter(|s| {
            let words: Vec<&str> = s.split(' ').collect();
            let nwords = words.len();
            let set: HashSet<&str> = words.into_iter().collect();
            nwords == set.len()
        })
        .count();
    println!("{}", one);
    let two = input
        .clone()
        .trim_end()
        .split('\n')
        .filter(|s| {
            let words: Vec<Vec<char>> = s
                .split(' ')
                .map(|word| -> Vec<char> {
                    let mut sword = word.chars().collect::<Vec<char>>();
                    sword.sort();
                    sword
                })
                .collect();
            let nwords = words.len();
            let set: HashSet<Vec<char>> = words.into_iter().collect();
            nwords == set.len()
        })
        .count();
    println!("{}", two);
}
